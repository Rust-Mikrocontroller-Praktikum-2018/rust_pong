use pong_core::display::Display;
use stm32f7::lcd::{Lcd, Color};

pub struct DefaultDisplay {
    pub display: Lcd
}

impl DefaultDisplay {
    pub fn new(display: Lcd) -> DefaultDisplay {
        DefaultDisplay{display}
    }
}

impl Display for DefaultDisplay {
    fn set_pixel(&mut self, x: usize, y: usize, hex_color: u32) {
        let mut layer_1 = self.display.layer_1().unwrap();
        layer_1.print_point_color_at(x, y, Color::from_hex(hex_color));
    }
}