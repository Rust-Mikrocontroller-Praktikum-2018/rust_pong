use std::rc::Rc;
use std::cell::RefCell;

use minifb::{Key, WindowOptions, Window};

use pong_core::framebuffer::FrameBuffer;
use pong_core::display::Display;


pub struct DefaultDisplay {
    pub window: Rc<RefCell<Window>>,
}

impl DefaultDisplay {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        let window = Window::new(name, width, height, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let rc_window = Rc::new(RefCell::new(window));

        DefaultDisplay {
            window: rc_window
        }
    }
}

impl Display for DefaultDisplay {
    fn show(&mut self, frame_buffer: &FrameBuffer) {
        let buffer = &frame_buffer.buffer;
        self.window.borrow_mut().update_with_buffer(buffer).unwrap();
    }
}