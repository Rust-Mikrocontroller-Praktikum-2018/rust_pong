use core::fmt::Arguments;

use stm32f7::semi_hosting::print;
use pong_core::debug::Debugger;


pub struct SemihostingDebugger {
    enabled: bool
}

impl SemihostingDebugger {
    pub fn new(enabled: bool) -> SemihostingDebugger {
        SemihostingDebugger {enabled}
    }
}

impl Debugger for SemihostingDebugger {
    fn println(&self, fmt_args: Arguments) {
        if self.enabled {
            print(fmt_args);
            print(format_args!("\n"));
        }
    }
}

