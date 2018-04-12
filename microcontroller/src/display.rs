extern crate alloc;

use pong_core::constants::{LCD_HEIGHT, LCD_WIDTH};
use pong_core::math::Vector;
use stm32f7::lcd::{Lcd, Color, FontRenderer};
use display::alloc::string::{String, ToString};
use alloc::vec::Vec;
use renderer::Point;


static TTF: &[u8] = include_bytes!("..//RobotoMono-Bold.ttf");

pub struct DefaultDisplay<'a> {
    pub display: Lcd,
    pub font_renderer: FontRenderer<'a>,
}

impl<'a> DefaultDisplay<'a> {
    pub fn new(display: Lcd) -> DefaultDisplay<'a> {
        DefaultDisplay{
            display: display,
            font_renderer: FontRenderer::new(TTF, 35.0),
        }
    }

    pub fn set_pixel_1(&mut self, x: usize, y: usize, hex_color: u32) {
        let mut layer_1 = self.display.layer_1().unwrap();
        layer_1.print_point_color_at(x, y, Color::from_hex(hex_color));
    }

    pub fn set_pixel_2(&mut self, x: usize, y: usize, hex_color: u32) {
        let mut layer = self.display.layer_2().unwrap();
        layer.print_point_color_at(x, y, Color::from_hex(hex_color));
    }

    pub fn unset_pixel_1(&mut self, x: usize, y: usize) {
        let mut layer = self.display.layer_1().unwrap();
        layer.print_point_color_at(x, y, Color::from_argb8888(0));
    }

    pub fn unset_pixel_2(&mut self, x: usize, y: usize) {
        let mut layer = self.display.layer_2().unwrap();
        layer.print_point_color_at(x, y, Color::from_argb8888(0));
    }


    pub fn show_score(&mut self, score_1: usize, score_2: usize, points: &mut Vec<Point>) {
        let score_rep = &mut score_1.to_string();
        score_rep.push_str(" : ");
        score_rep.push_str(&score_2.to_string());
        let offset_x = ((LCD_WIDTH as i32) / 2) - 35;
        let offset_y = ((LCD_HEIGHT as i32) / 2) - 35;

        self.font_renderer.render(score_rep, |x, y, v| {
            if v > 0.8 {
                points.push(Point {
                    position: Vector {x: (x as i32) + offset_x, y: (y as i32) + offset_y},
                    value: 0,
                });
            }
        });
    }
}