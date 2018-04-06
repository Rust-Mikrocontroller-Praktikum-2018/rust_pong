use minifb::{Key, WindowOptions, Window};

use pong_core::controller::{Controller, Direction};

pub struct DefaultController {
    window: Window
}

impl DefaultController {
    fn new(window: Window) -> Self {
        DefaultController {
            window
        }
    }
}


impl Controller for DefaultController {
    fn start(&self) -> bool {
        self.window.is_key_down(Key::Enter)
    }

    fn get_direction(&self) -> Direction {
        if self.window.is_key_down(Key::Up) {
            return Direction::Up;
        }

        if self.window.is_key_down(Key::Down) {
            return Direction::Down;
        }

        return Direction::None;
    }
}