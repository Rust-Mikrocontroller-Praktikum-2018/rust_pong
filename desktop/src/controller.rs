use std::rc::Rc;
use std::cell::RefCell;

use minifb::{Key, WindowOptions, Window};

use pong_core::controller::{Controller, Direction};

pub struct DefaultController {
    up_key: Key,
    down_key: Key,
    window: Rc<RefCell<Window>>
}

impl DefaultController {
    pub fn new(window: Rc<RefCell<Window>>, up_key: Key, down_key: Key) -> Self {
        DefaultController {
            window,
            up_key,
            down_key
        }
    }
}


impl Controller for DefaultController {
    fn start(&self) -> bool {
        self.window.borrow().is_key_down(Key::Enter)
    }

    fn get_direction(&self) -> Direction {
        if self.window.borrow().is_key_down(self.up_key) {
            return Direction::Up;
        }

        if self.window.borrow().is_key_down(self.down_key) {
            return Direction::Down;
        }

        return Direction::None;
    }
}