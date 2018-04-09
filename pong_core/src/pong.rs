use math::Vector;
use core::cmp::{min, max};
use core::fmt::Debug;

use constants::{PADDLE_OFFSET, PADDLE_HEIGHT, PADDLE_SPEED};

#[derive(Debug)]
pub struct Ball {
    pub position: Vector<i32>,
    pub direction_x: f32,
    pub direction_y: f32,
}

#[derive(Debug)]
pub struct Paddle {
    pub position: Vector<i32>
}

#[derive(Debug)]
pub enum GameMode {
    NewGame,
    Running,
    GameOver,
}

#[derive(Debug)]
pub struct GameState {
    pub ball: Ball,
    pub paddle_1: Paddle,
    pub paddle_2: Paddle,
    pub score_1: usize,
    pub score_2: usize,
    pub running: GameMode,
}

pub struct Game {
    pub state: GameState,
    pub width: i32,
    pub height: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        let ball = Ball {
            position: Vector { x: width / 2, y: height / 2 },
            direction_x: 0.5,
            direction_y: 0.5,
        };

        let paddle_1 = Paddle { position: Vector { x: PADDLE_OFFSET, y: height / 2 } };
        let paddle_2 = Paddle { position: Vector { x: width - PADDLE_OFFSET, y: height / 2 } };

        Game {
            width,
            height,
            state: GameState {
                ball: ball,
                paddle_1: paddle_1,
                paddle_2: paddle_2,
                score_1: 0,
                score_2: 0,
                running: GameMode::NewGame,
            },
        }
    }

    pub fn normalize_paddle(paddle: &mut Paddle, height: i32) {
        paddle.position.y = min(paddle.position.y, height - PADDLE_HEIGHT / 2);
        paddle.position.y = max(paddle.position.y, PADDLE_HEIGHT / 2);
    }

    pub fn update_state(&mut self, player_1_input: i32, player_2_input: i32, t_delta: i32) {
        let distance = t_delta * PADDLE_SPEED;

        let paddle_1 = &mut self.state.paddle_1;
        let paddle_2 = &mut self.state.paddle_2;

        let diff_1 = player_1_input - paddle_1.position.y;
        let diff_2 = player_2_input - paddle_2.position.y;
        let path_1 = min(diff_1.abs(), distance) * diff_1.signum();
        let path_2 = min(diff_2.abs(), distance) * diff_2.signum();

        paddle_1.position.y += path_1;
        paddle_2.position.y += path_2;

        Game::normalize_paddle(paddle_1, self.height);
        Game::normalize_paddle(paddle_2, self.height);
    }
}
