use pong_core::display::Display;
use pong_core::framebuffer::FrameBuffer;
use pong_core::constants::{LCD_HEIGHT, LCD_WIDTH};
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
    fn show(&mut self, frame_buffer: &mut FrameBuffer) {
        let mut layer_1 = self.display.layer_1().unwrap();
        layer_1.clear();

        for i in 0..(LCD_HEIGHT as usize) {
            for j in 0..(LCD_WIDTH as usize) {
                if frame_buffer.get_pixel(j, i) != 0 {
                    layer_1.print_point_color_at(j, i, Color::rgb(255, 0, 0));
                }
            }
        }
    }
}