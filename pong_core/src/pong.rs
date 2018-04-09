use math::Vector;
use core::cmp::{min, max};
use display::Display;
use controller::Controller;
use controller::Direction;

use constants::{PADDLE_OFFSET, PADDLE_HEIGHT, PADDLE_SPEED, LCD_HEIGHT};

pub struct Ball {
    pub position: Vector<i32>,
    pub direction_x: f32,
    pub direction_y: f32,
}

pub struct Paddle {
    pub position: Vector<i32>
}

pub enum GameMode {
    NewGame,
    Running,
    GameOver,
}

pub struct GameState {
    pub ball: Ball,
    pub paddle_1: Paddle,
    pub paddle_2: Paddle,
    pub score_1: usize,
    pub score_2: usize,
    pub running: GameMode,
}

impl GameState {
    pub fn new() -> GameState{
        let ball = Ball{
            position: Vector{x: LCD_WIDTH/2, y: LCD_HEIGHT/2},
            direction_x: 0.5,
            direction_y: 0.5,
        };

        let paddle_1 = Paddle { position: Vector { x: PADDLE_OFFSET, y: height / 2 } };
        let paddle_2 = Paddle { position: Vector { x: width - PADDLE_OFFSET, y: height / 2 } };

        GameState {
            ball: ball,
            paddle_1: paddle_1,
            paddle_2: paddle_2,
            score_1: 0,
            score_2: 0,
            running: GameMode::NewGame
        }
    }

    pub fn normalize_paddle(paddle: &mut Paddle) {
        paddle.position.y = min(paddle.position.y, LCD_HEIGHT - PADDLE_HEIGHT / 2);
        paddle.position.y = max(paddle.position.y, PADDLE_HEIGHT / 2);
    }

    pub fn update(&mut self, action_1: Direction, action_2: Direction, t_delta: i32) {
        let action_1 = action_1 as i32;
        let action_2 = action_2 as i32;

        let distance = t_delta * PADDLE_SPEED;
        self.paddle_1.position.y += distance * action_1;
        self.paddle_2.position.y += distance * action_2;

        GameState::normalize_paddle(&mut self.paddle_1);
        GameState::normalize_paddle(&mut self.paddle_2);

        // Problem: Need to update y_pos in controller, but cqannot do it agnostically
    }
}
