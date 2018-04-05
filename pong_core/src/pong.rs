use math::Vector;
use constants::{LCD_WIDTH, LCD_HEIGHT, PADDLE_OFFSET, PADDLE_HEIGHT, PADDLE_SPEED};

pub struct Ball {
    position: Vector,
    direction: Vector,
}

pub struct Paddle {
    position: Vector
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
    state: GameState
}

impl Game {
    fn new() -> Game {
        let ball = Ball{
            position: Vector{x: LCD_WIDTH/2, y: LCD_HEIGHT/2},
            direction: Vector{x: 0.5, y: 0.5}
        };

        let paddle_1 = Paddle{position: Vector{x: PADDLE_OFFSET, y: LCD_HEIGHT/2}};
        let paddle_2 = Paddle{position: Vector{x: LCD_WIDTH - PADDLE_OFFSET, y: LCD_HEIGHT/2}};

        Game {
            ball: ball,
            paddle_1: paddle_1,
            paddle_2: paddle_2,
            score_1: 0,
            score_2: 0,
            running: GameMode::NEW_GAME
        }
    }

    fn get_state() -> GameState {
        state
    }

    fn update_state(player_1_input: usize, player_2_input: usize, t_delta: usize) {
        use core::cmp::min;
        let distance = t_delta * PADDLE_SPEED;

        let diff_1 = (player_1_input - paddle_1.y);
        let diff_2 = (player_2_input - paddle_2.y);
        let path_1 = min(diff_1.abs(), distance) * diff_1.signum();
        let path_2 = min(diff_2.abs(), distance) * diff_2.signum();

        paddle_1.y += path_1;
        paddle_2.y += path_2;
    }
}
