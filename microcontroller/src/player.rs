use pong_core::constants::LCD_HEIGHT;
use pong_core::controller::Direction;
use pong_core::pong::Paddle;
use core::cmp::Ordering;

pub struct PlayerState {
    pub y: i32,
}

impl PlayerState {

    pub fn new() -> PlayerState {
        PlayerState {
            y: ((LCD_HEIGHT as i32)/2),
        }
    }

    pub fn update(&mut self, paddle: &Paddle) {
        self.y = paddle.position.y as i32;
    }

    pub fn get_direction(&mut self, input: i32) -> Direction {
        match input.cmp(&self.y) {
            Ordering::Less => Direction::Up,
            Ordering::Equal => Direction::None,
            Ordering::Greater => Direction::Down,
        }
    }
}