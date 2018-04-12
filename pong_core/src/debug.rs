use core::fmt::Arguments;

use stm32f7::semi_hosting::print;

pub trait Debugger {
    fn println(fmt_args: Arguments);
}

pub struct SemihostingDebugger {}

impl SemihostingDebugger {
    pub fn new() -> SemihostingDebugger {
        SemihostingDebugger {}
    }
}

impl Debugger for SemihostingDebugger {
    fn println(fmt_args: Arguments) {
        if false {
            print(fmt_args);
            print(format_args!("\n"));
        }
    }
}