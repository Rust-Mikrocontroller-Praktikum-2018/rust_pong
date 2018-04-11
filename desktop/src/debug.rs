use pong_core::debug::Debugger;

pub struct DefaultDebugger {}

impl Debugger for DefaultDebugger {
    fn alert(msg: &'static str) {
        println!("{}", msg);
    }
}