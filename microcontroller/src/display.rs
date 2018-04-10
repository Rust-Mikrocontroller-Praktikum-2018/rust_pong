extern crate alloc;

use pong_core::display::Display;
use pong_core::constants::LCD_WIDTH;
use stm32f7::lcd::{Lcd, Color, FontRenderer};
use display::alloc::string::{String, ToString};


static TTF: &[u8] = include_bytes!("..//RobotoMono-Bold.ttf");

pub struct DefaultDisplay<'a> {
    pub display: Lcd,
    pub font_renderer: FontRenderer<'a>,
    pub old_score: String
}

impl<'a> DefaultDisplay<'a> {
    pub fn new(display: Lcd) -> DefaultDisplay<'a> {
        DefaultDisplay{
            display: display,
            font_renderer: FontRenderer::new(TTF, 28.0),
            old_score: "".to_string()
        }
    }
}

impl<'a> Display for DefaultDisplay<'a> {
    fn set_pixel(&mut self, x: usize, y: usize, hex_color: u32) {
        let mut layer_1 = self.display.layer_1().unwrap();
        layer_1.print_point_color_at(x, y, Color::from_hex(hex_color));
    }

    fn show_score(&mut self, score_1: usize, score_2: usize, hex_color: u32) {
        let mut layer_1 = self.display.layer_1().unwrap();

        let score_rep = &mut score_1.to_string();
        score_rep.push_str(" : ");
        score_rep.push_str(&score_2.to_string());

        let offset = ((LCD_WIDTH as usize) / 2) - 30;

        if &self.old_score != score_rep {
            self.font_renderer.render(&self.old_score, |x, y, v| {
                if v > 0.8 {
                    layer_1.print_point_color_at(x + offset, y, Color::from_hex(0x000000));
                }
            });
        }

        self.font_renderer.render(score_rep, |x, y, v| {
            if v > 0.8 {
              layer_1.print_point_color_at(x + offset, y, Color::from_hex(hex_color));
            }
        });

        self.old_score = score_rep.to_string();
    }
}