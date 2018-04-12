#![feature(lang_items)]
#![feature(const_fn)]
#![feature(alloc)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![no_std]
#![no_main]

mod player;
mod display;
mod renderer;
use player::PlayerState;
use display::DefaultDisplay;

#[macro_use]
extern crate stm32f7_discovery as stm32f7;
extern crate alloc;
extern crate pong_core;

// initialization routines for .data and .bss
extern crate compiler_builtins;
extern crate r0;

// game related structs
use pong_core::{pong, constants};
use pong::{Game, GameState};
use constants::{LCD_HEIGHT, LCD_WIDTH};
use renderer::Renderer;

// hardware register structs with accessor methods
use stm32f7::{board, embedded, lcd, sdram, system_clock, i2c, touch};

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

    let game = Game::new(LCD_WIDTH, LCD_HEIGHT);
    let mut game_state = GameState::new(LCD_WIDTH, LCD_HEIGHT); 
    let mut display = DefaultDisplay::new(lcd);
    let mut renderer = Renderer::new(   );

    let mut player_1 = PlayerState::new();
    let mut player_2 = PlayerState::new();

    let mut t_start = system_clock::ticks();

    loop {
        renderer.render(&game_state, &mut display);
        //hprintln!("Rendering time: {}", system_clock::ticks() - start);

        let start = system_clock::ticks();
        let mut input_1 = player_1.y;
        let mut input_2 = player_2.y;

        for touch in &touch::touches(&mut i2c_3).unwrap() {
            let action_x = touch.x as i32;
            let action_y = touch.y as i32;

            if action_x < (LCD_WIDTH as i32) / 2 {
                input_1 = action_y;
            } else {
                input_2 = action_y;
            }
        }

        let t_now = system_clock::ticks();
        let t_delta = t_now - t_start;
        t_start = t_now;
        game_state = game.update(
            game_state,
            player_1.get_direction(input_1),
            player_2.get_direction(input_2),
            ((t_delta as f32) / (system_clock::get_frequency() / 1_000_000) as f32) * 10.0
        );


        player_1.update(&game_state.paddle_1);
        player_2.update(&game_state.paddle_2);

        //hprintln!("Game time: {}", system_clock::ticks() - start);
    }
}