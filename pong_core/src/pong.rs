
use math::{clamp, Vector};
use core::cmp::{min, max};
use display::Display;
use controller::Direction;

use constants::{PADDLE_OFFSET, PADDLE_HEIGHT, PADDLE_SPEED, LCD_HEIGHT, LCD_WIDTH};

pub struct Ball {
    pub position: Vector<f32>,
    pub direction: Vector<f32>
}

pub struct Paddle {
    pub position: Vector<f32>
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
            position: Vector{x: LCD_WIDTH/2.0, y: LCD_HEIGHT/2.0},
            direction: Vector {x: 1.0, y: 1.0},
        };

        let paddle_1 = Paddle { position: Vector { x: PADDLE_OFFSET, y: LCD_HEIGHT / 2.0 } };
        let paddle_2 = Paddle { position: Vector { x: LCD_WIDTH - PADDLE_OFFSET, y: LCD_HEIGHT / 2.0 } };

        GameState {
            ball: ball,
            paddle_1: paddle_1,
            paddle_2: paddle_2,
            score_1: 0,
            score_2: 0,
            running: GameMode::NewGame
        }
    }

    pub fn clip_paddle(paddle: &mut Paddle) {
        paddle.position.y = clamp(paddle.position.y, PADDLE_HEIGHT / 2.0, LCD_HEIGHT - PADDLE_HEIGHT / 2.0);
    }

    pub fn clamp_vector<T: PartialOrd>(vector: Vector<T>, min: Vector<T>, max: Vector<T>) -> Vector<T> {
        Vector {
            x: clamp(vector.x, min.x, max.x),
            y: clamp(vector.y, min.y, max.y),
        }
    }

    fn reflect(&mut self) {
        if self.ball.position.x >= LCD_WIDTH - 25.0 {
            self.ball.direction.x *= -1.0;
        }
        if self.ball.position.x <= 0.0 + 25.0 {
            self.ball.direction.x *= -1.0;
        }
        if self.ball.position.y >= LCD_HEIGHT - 25.0 {
            self.ball.direction.y *= -1.0;
        }
        if self.ball.position.y <= 0.0 + 25.0 {
            self.ball.direction.y *= -1.0;
        }


    }

    pub fn update(&mut self, action_1: Direction, action_2: Direction, t_delta: f32) {
        let action_1 = action_1 as i32 as f32;
        let action_2 = action_2 as i32 as f32;

        let distance = t_delta * PADDLE_SPEED;
        self.paddle_1.position.y += distance * action_1;
        self.paddle_2.position.y += distance * action_2;

        GameState::clip_paddle(&mut self.paddle_1);
        GameState::clip_paddle(&mut self.paddle_2);

        /*
        let new_ball_position = Self::clamp_vector(
            self.ball.position + self.ball.direction,
            Vector {x: 15.0, y: 15.0},
            Vector {x: LCD_WIDTH - 15.0, y: LCD_HEIGHT - 15.0}
        );

        */

        let new_ball_position = self.ball.position + self.ball.direction;
        self.ball.position = new_ball_position;

        self.reflect();

        // Problem: Need to update y_pos in controller, but cqannot do it agnostically
    }
}
