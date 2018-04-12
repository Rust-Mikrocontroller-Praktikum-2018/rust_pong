use core::fmt::Arguments;

use stm32f7::semi_hosting::print;
use pong_core::debug::Debugger;


pub struct SemihostingDebugger {
}

impl SemihostingDebugger {
    pub fn new() -> SemihostingDebugger {
        SemihostingDebugger {}
    }
}

impl Debugger for SemihostingDebugger {
    fn println(fmt_args: Arguments) {
        print(fmt_args);
        print(format_args!("\n"));
    }
}

