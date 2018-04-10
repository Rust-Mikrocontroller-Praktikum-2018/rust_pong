#![feature(lang_items)]
#![feature(const_fn)]
#![feature(alloc)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![no_std]
#![no_main]

mod controller;
mod display;
use controller::{DefaultController, Players};
use display::DefaultDisplay;

extern crate stm32f7_discovery as stm32f7;

extern crate pong_core;

// initialization routines for .data and .bss
extern crate compiler_builtins;
extern crate r0;

// game related structs
use pong_core::{pong, framebuffer, constants, controller as core_controller, renderer, display as core_display};
use constants::{LCD_HEIGHT, LCD_WIDTH};
use renderer::Renderer;
use core_display::Display;
use core_controller::Direction;
use framebuffer::FrameBuffer;

// hardware register structs with accessor methods
use stm32f7::{board, embedded, lcd, sdram, system_clock, i2c, interrupts, touch};
use interrupts::primask_mutex::PrimaskMutex;

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static mut __DATA_END: u32;
        static mut __DATA_START: u32;

        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }

    // initializes the .data section (copy the data segment initializers from flash to RAM)
    r0::init_data(&mut __DATA_START, &mut __DATA_END, &__DATA_LOAD);
    // zeroes the .bss section
    r0::zero_bss(&mut __BSS_START, &__BSS_END);

    stm32f7::heap::init();

    // enable floating point unit
    let scb = stm32f7::cortex_m::peripheral::scb_mut();
    scb.cpacr.modify(|v| v | 0b1111 << 20);
    asm!("DSB; ISB;"::::"volatile"); // pipeline flush

    main(board::hw());
}

// WORKAROUND: rust compiler will inline & reorder fp instructions into
#[inline(never)] //             reset() before the FPU is initialized
fn main(hw: board::Hardware) -> ! {
    use embedded::interfaces::gpio::{Gpio};

    let board::Hardware {
        rcc,
        pwr,
        flash,
        fmc,
        ltdc,
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
        i2c_3,
        ..
    } = hw;

    let mut gpio = Gpio::new(
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
    );

    system_clock::init(rcc, pwr, flash);

    // enable all gpio ports
    rcc.ahb1enr.update(|r| {
        r.set_gpioaen(true);
        r.set_gpioben(true);
        r.set_gpiocen(true);
        r.set_gpioden(true);
        r.set_gpioeen(true);
        r.set_gpiofen(true);
        r.set_gpiogen(true);
        r.set_gpiohen(true);
        r.set_gpioien(true);
        r.set_gpiojen(true);
        r.set_gpioken(true);
    });

    // init sdram (needed for display buffer)
    sdram::init(rcc, fmc, &mut gpio);

    // lcd controller
    let mut lcd = lcd::init(ltdc, rcc, &mut gpio);
    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();

    layer_1.clear();
    layer_2.clear();
    lcd::init_stdout(layer_2);
    lcd.set_background_color(lcd::Color::from_hex(0x000000));

    // i2c
    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);
    i2c_3.test_1();
    i2c_3.test_2();

    let mut game_state = pong::GameState::new(); 
    let mut display = DefaultDisplay::new(lcd);
    let mut renderer = Renderer::new();

    loop {
        renderer.render(&game_state, &mut display);
        game_state.update(Direction::None, Direction::None, 5.0);
    }


/*
    let mutex = PrimaskMutex::new(i2c_3);

    let threshold = 20;
    let mut last_render_time = system_clock::ticks();

    let mut display = DefaultDisplay::new(lcd);
    let mut controller_1 = DefaultController::new(Players::Player1, &mutex);
    let mut controller_2 = DefaultController::new(Players::Player2, &mutex);

    use core::fmt::Write;


    let mut game_state = pong::GameState::new(); 

  //  let renderer = Renderer::new();


    loop {
        if controller_1.start() {
            layer_1.text_writer().write_str("Player one started");
        }

        if controller_2.start() {
            layer_1.text_writer().write_str("Player two started");
        }
    }


/*
    renderer.render(&game_state, &mut frame_buffer);
    display.show(&mut frame_buffer);

    while !controller_1.start() && !controller_2.start() {
        /* Wait for game to start. */
    }
    */
   // loop {
        /*
        let ticks = system_clock::ticks();
        let t_delta: i32 = (ticks - last_render_time) as i32;
       
        if t_delta >= threshold {
            game_state.update(
                controller_1.get_direction(),
                controller_2.get_direction(),
                t_delta
            );   

            controller_1.update_pos(&game_state.paddle_1);
            controller_2.update_pos(&game_state.paddle_2);

            renderer.render(&game_state, &mut frame_buffer);
            display.show(&mut frame_buffer);
            last_render_time = ticks;
        }
        */
   // }*/
}