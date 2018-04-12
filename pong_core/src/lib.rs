#![no_std]
#![feature(alloc)]
#![feature(inclusive_range_syntax)]


extern crate alloc;
#[macro_use]
extern crate stm32f7_discovery as stm32f7;

pub mod debug;
pub mod pong;
pub mod framebuffer;
pub mod display;
pub mod controller;
pub mod renderer;
pub mod constants;
pub mod math;
