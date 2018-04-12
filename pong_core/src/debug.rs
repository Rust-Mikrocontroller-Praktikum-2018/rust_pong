use core::fmt::Arguments;

pub trait Debugger {
    fn println(&self, fmt_args: Arguments);
}