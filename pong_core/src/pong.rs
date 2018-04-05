use math::Vector;
use constants::{LCD_WIDTH, LCD_HEIGHT, PADDLE_OFFSET, PADDLE_HEIGHT, PADDLE_SPEED};

pub struct Ball {
    pub position: Vector,
    pub direction_x: f32,
    pub direction_y: f32,
}

pub struct Paddle {
    pub position: Vector
}

pub enum GameMode {
    NEW_GAME,
    RUNNUNG,
    GAME_OVER,
} 

pub struct GameState {
    ball: Ball,
    paddle_1: Paddle,
    paddle_2: Paddle,
    score_1: usize,
    score_2: usize,
    running: GameMode,
}

pub struct Game {
    pub state: GameState
}

impl Game {
    pub fn new() -> Game {
        let ball = Ball{
            position: Vector{x: LCD_WIDTH/2, y: LCD_HEIGHT/2},
            direction_x: 0.5,
            direction_y: 0.5,
        };

        let paddle_1 = Paddle{position: Vector{x: PADDLE_OFFSET, y: LCD_HEIGHT/2}};
        let paddle_2 = Paddle{position: Vector{x: LCD_WIDTH - PADDLE_OFFSET, y: LCD_HEIGHT/2}};

        Game {
            state: GameState{
                ball: ball,
                paddle_1: paddle_1,
                paddle_2: paddle_2,
                score_1: 0,
                score_2: 0,
                running: GameMode::NEW_GAME
            }
        }
    }

    pub fn update_state(&mut self, player_1_input: i32, player_2_input: i32, t_delta: i32) {
        use core::cmp::min;
        let distance = t_delta * PADDLE_SPEED;

        let paddle_1 = &mut self.state.paddle_1;
        let paddle_2 = &mut self.state.paddle_2;

        let diff_1 = player_1_input  - paddle_1.position.y;
        let diff_2 = player_2_input - paddle_2.position.y;
        let path_1 = min(diff_1.abs(), distance) * diff_1.signum();
        let path_2 = min(diff_2.abs(), distance) * diff_2.signum();

        paddle_1.position.y += path_1;
        paddle_2.position.y += path_2;
    }
}
