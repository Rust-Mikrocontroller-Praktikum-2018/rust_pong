use math::{clamp, cross_product, unit, Vector};
use core::cmp::{min, max};
use display::Display;
use controller::Direction;

use constants::{PADDLE_OFFSET, PADDLE_HEIGHT, PADDLE_SPEED, LCD_HEIGHT, LCD_WIDTH};

trait Rectangle {
    fn height(&self) -> f32;
    fn width(&self) -> f32;
}

#[derive(Debug, Copy, Clone)]
pub struct Ball {
    pub position: Vector<f32>,
    pub direction: Vector<f32>,
    pub diameter: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Paddle {
    pub position: Vector<f32>,
    pub height: f32,
    pub width: f32,
}

impl Rectangle for Paddle {
    fn height(&self) -> f32 {
        self.height
    }

    fn width(&self) -> f32 {
        self.height
    }
}

#[derive(Debug, Copy, Clone)]
pub enum GameMode {
    NewGame,
    Running,
    GameOver,
}

#[derive(Debug, Copy, Clone)]
pub struct GameState {
    pub ball: Ball,
    pub paddle_1: Paddle,
    pub paddle_2: Paddle,
    pub score_1: usize,
    pub score_2: usize,
    pub running: GameMode,
}

impl GameState {
    pub fn new() -> GameState {
        let ball = Ball {
            position: Vector { x: LCD_WIDTH / 2.0, y: LCD_HEIGHT / 2.0 },
            direction: Vector { x: 1.0, y: 1.0 },
            diameter: 25.0,
        };

        let paddle_1 = Paddle {
            position: Vector { x: PADDLE_OFFSET, y: LCD_HEIGHT / 2.0 },
            width: 20.0,
            height: 80.0,
        };
        let paddle_2 = Paddle {
            position: Vector { x: LCD_WIDTH - PADDLE_OFFSET, y: LCD_HEIGHT / 2.0 },
            width: 20.0,
            height: 80.0,
        };

        GameState {
            ball: ball,
            paddle_1: paddle_1,
            paddle_2: paddle_2,
            score_1: 0,
            score_2: 0,
            running: GameMode::NewGame,
        }
    }
}

pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Game {}
    }

    fn clamp_paddle(paddle: &mut Paddle) {
        paddle.position.y = clamp(paddle.position.y, PADDLE_HEIGHT / 2.0, LCD_HEIGHT - PADDLE_HEIGHT / 2.0);
    }

    fn clamp_vector<T: PartialOrd>(vector: Vector<T>, min: Vector<T>, max: Vector<T>) -> Vector<T> {
        Vector {
            x: clamp(vector.x, min.x, max.x),
            y: clamp(vector.y, min.y, max.y),
        }
    }

    fn reflect(mut game_state: GameState) -> GameState {
        if game_state.ball.position.y >= LCD_HEIGHT - 25.0 {
            game_state.ball.direction.y *= -1.0;
        }
        if game_state.ball.position.y <= 0.0 + 25.0 {
            game_state.ball.direction.y *= -1.0;
        }

        game_state
    }

    fn crash(mut game_state: GameState) -> GameState {
        if game_state.ball.position.x >= LCD_WIDTH - 25.0 || game_state.ball.position.x <= 0.0 + 25.0 {
            game_state.running = GameMode::GameOver;
            game_state = GameState::new();
        }

        game_state
    }

    fn intersect(p: Vector<f32>, r: Vector<f32>, q: Vector<f32>, s: Vector<f32>) -> (f32, f32) {
        let t = cross_product((q - p), s) / cross_product(r, s);
        let u = cross_product((q - p), r) / cross_product(s, r);

        (t, u)
    }

    fn detect_collision(mut new_state: GameState, old_state: GameState) -> (GameState, f32, f32) {
        let movement_ball = new_state.ball.position - old_state.ball.position;
        let movement_paddle = Vector {x: 0.0, y: new_state.paddle_2.height};

        let position_ball = old_state.ball.position;
        let position_paddle = Vector {
            x: new_state.paddle_2.position.x - new_state.paddle_2.width/ 2.0,
            y: new_state.paddle_2.position.y - new_state.paddle_2.height / 2.0,
        };

        let (t, u) = Self::intersect(position_paddle, movement_paddle, position_ball, movement_ball);

        if cross_product(movement_ball, movement_paddle) != 0.0 && 0.0 <= t && t <= 1.0 && 0.0 <= u && u <= 1.0 {
            let mut dir = Vector {x: -1.0, y: 1.0};
            /*
            if t > 0.5 {
                dir = Vector {x: -1.0, y: 1.0};
            }
            */
            new_state.ball.position = old_state.ball.position + old_state.ball.direction * Vector {x: u, y: u};
            new_state.ball.direction = old_state.ball.direction * dir;
            new_state.ball.position = new_state.ball.position + new_state.ball.direction * Vector {x: (1.0-u), y: (1.0-u)};
        }

        (new_state, t, u)

    }

    pub fn update(mut game_state: GameState, action_1: Direction, action_2: Direction, t_delta: f32) -> (GameState, f32, f32) {
        let old_state = game_state;

        let action_1 = action_1 as i32 as f32;
        let action_2 = action_2 as i32 as f32;

        let distance = t_delta * PADDLE_SPEED;
        game_state.paddle_1.position.y += distance * action_1;
        game_state.paddle_2.position.y += distance * action_2;

        Self::clamp_paddle(&mut game_state.paddle_1);
        Self::clamp_paddle(&mut game_state.paddle_2);

        /*
        let new_ball_position = Self::clamp_vector(
            self.ball.position + self.ball.direction,
            Vector {x: 15.0, y: 15.0},
            Vector {x: LCD_WIDTH - 15.0, y: LCD_HEIGHT - 15.0}
        );

        */

        let new_ball_position = game_state.ball.position + game_state.ball.direction * Vector { x: t_delta, y: t_delta };
        game_state.ball.position = new_ball_position;

        let game_state = Self::reflect(game_state);
        let game_state = Self::crash(game_state);
        let game_state = Self::detect_collision(game_state, old_state);

        game_state

        // Problem: Need to update y_pos in controller, but cqannot do it agnostically
    }
}
