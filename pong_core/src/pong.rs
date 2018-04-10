use alloc::{LinkedList};
use alloc::boxed::Box;
use core::cmp::{min, max};
use core::option::{Option};

use math::{clamp, cross_product, dot_product, unit, length, Vector};
use display::Display;
use controller::Direction;

use constants::{PADDLE_OFFSET, PADDLE_HEIGHT, PADDLE_SPEED};

trait Rectangle {
    fn height(&self) -> f32;
    fn width(&self) -> f32;
}

trait CollisionEffect {
    fn on_collision(&self, new_state: GameState, old_state: GameState, t: f32, u: f32) -> GameState;
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

pub struct Edge {
    pub position: Vector<f32>,
    pub direction: Vector<f32>,
}

impl Rectangle for Paddle {
    fn height(&self) -> f32 {
        self.height
    }

    fn width(&self) -> f32 {
        self.height
    }
}

impl CollisionEffect for Edge {
    fn on_collision(&self, mut new_state: GameState, old_state: GameState, t: f32, u: f32) -> GameState {
        new_state.ball.position = old_state.ball.position + old_state.ball.direction * Vector {x: u, y: u};
        // Reflect ball
        let d = new_state.ball.direction;
        let n = unit(Vector {x: self.direction.y, y: -self.direction.x}); // + Vector::new((u - 0.5) / 0.5) * self.direction;

        new_state.ball.direction = Vector::new(length(d)) * unit(d - Vector::new(2.0) * (Vector::new(dot_product(d, n)) * n));
        new_state.ball.position = new_state.ball.position + new_state.ball.direction * Vector {x: (3.0-u), y: (3.0-u)};

        new_state
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
    pub fn new(width: f32, height: f32) -> GameState {
        let ball = Ball {
            position: Vector { x: width / 2.0, y: height / 2.0 },
            direction: Vector { x: 0.9, y: -1.1 },
            diameter: 25.0,
        };

        let paddle_1 = Paddle {
            position: Vector { x: PADDLE_OFFSET, y: height / 2.0 },
            width: 20.0,
            height: 80.0,
        };
        let paddle_2 = Paddle {
            position: Vector { x: width - PADDLE_OFFSET, y: height / 2.0 },
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

pub struct Game {
    width: f32,
    height: f32
}

impl Game {
    pub fn new(width: f32, height: f32) -> Self {
        Game {width, height}
    }

    fn clamp_paddle(&self, paddle: &mut Paddle) {
        paddle.position.y = clamp(paddle.position.y, PADDLE_HEIGHT / 2.0, self.height - PADDLE_HEIGHT / 2.0);
    }

    fn clamp_vector<T: PartialOrd>(vector: Vector<T>, min: Vector<T>, max: Vector<T>) -> Vector<T> {
        Vector {
            x: clamp(vector.x, min.x, max.x),
            y: clamp(vector.y, min.y, max.y),
        }
    }

    fn reflect(&self, mut game_state: GameState) -> GameState {
        if game_state.ball.position.y >= self.height - game_state.ball.diameter - 1.0 {
            game_state.ball.direction.y *= -1.0;
        }
        if game_state.ball.position.y <= 0.0 + game_state.ball.diameter - 1.0 {
            game_state.ball.direction.y *= -1.0;
        }

        game_state
    }

    fn crash(&self, mut game_state: GameState) -> GameState {
        if game_state.ball.position.x >= self.width - game_state.ball.diameter - 1.0  || game_state.ball.position.x <= 0.0 + game_state.ball.diameter + 1.0 {
            game_state.running = GameMode::GameOver;
            game_state = GameState::new(self.width, self.height);
        }

        game_state
    }

    fn intersect(p: Vector<f32>, r: Vector<f32>, q: Vector<f32>, s: Vector<f32>) -> (f32, f32) {
        let t = cross_product((q - p), s) / cross_product(r, s);
        let u = cross_product((q - p), r) / cross_product(s, r);

        (t, u)
    }

    fn get_edges(paddle: Paddle) -> LinkedList<Edge> {
        let left_edge = Edge {
            position: Vector {
                x: paddle.position.x - paddle.width / 2.0,
                y: paddle.position.y - paddle.height / 2.0,
            },
            direction: Vector {
                x: 0.0, y: paddle.height
            }
        };

        let right_edge = Edge {
            position: Vector {
                x: paddle.position.x + paddle.width / 2.0,
                y: paddle.position.y - paddle.height / 2.0,
            },
            direction: Vector {
                x: 0.0, y: paddle.height
            }
        };

        let top_edge = Edge {
            position: Vector {
                x: paddle.position.x - paddle.width / 2.0,
                y: paddle.position.y - paddle.height / 2.0,
            },
            direction: Vector {
                x: paddle.width, y: 0.0
            }
        };

        let bottom_edge = Edge {
            position: Vector {
                x: paddle.position.x - paddle.width / 2.0,
                y: paddle.position.y + paddle.height / 2.0,
            },
            direction: Vector {
                x: paddle.width, y: 0.0
            }
        };



        let mut edges: LinkedList<Edge> = LinkedList::new();
        edges.push_back(left_edge);
        edges.push_back(right_edge);
        edges.push_back(top_edge);
        edges.push_back(bottom_edge);

        edges
    }

    fn detect_collision_paddle(paddle_new: &Paddle, paddle_old: Paddle, ball_new: Ball, ball_old: Ball) -> LinkedList<Option<(f32, f32, Box<CollisionEffect>)>> {
        let edges = Self::get_edges(paddle_old);
        let mut collisions: LinkedList<Option<(f32, f32, Box<CollisionEffect>)>> = LinkedList::new();

        for e in edges {
            let (t, u) = Self::intersect(e.position, e.direction, ball_old.position, ball_old.direction);

            if cross_product(ball_old.direction, e.direction) != 0.0 && 0.0 <= t && t <= 1.0 && 0.0 <= u && u <= 1.0 {
                collisions.push_back(Some((t, u, Box::new(e))));
            } else {
                collisions.push_back(None);
            }
        }

        collisions

    }

    fn detect_collision(new_state: GameState, old_state: GameState) -> GameState {
        let mut collisions: LinkedList<Option<(f32, f32, Box<CollisionEffect>)>> = LinkedList::new();
        let mut paddle_1_collisions = Self::detect_collision_paddle(
            &new_state.paddle_1,
            old_state.paddle_1,
            new_state.ball,
            old_state.ball,
        );
        let mut paddle_2_collisions = Self::detect_collision_paddle(
            &new_state.paddle_2,
            old_state.paddle_2,
            new_state.ball,
            old_state.ball,
        );

        collisions.append(&mut paddle_1_collisions);
        collisions.append(&mut paddle_2_collisions);

        let mut new_state= new_state;
        for c in collisions {
            match c {
                Some((t, u, effect)) => {
                    new_state = effect.on_collision(new_state, old_state, t, u);
                },
                None => {}
            }
        }

        new_state
    }

    pub fn update(&self, mut game_state: GameState, action_1: Direction, action_2: Direction, t_delta: f32) -> GameState {
        let old_state = game_state;

        let action_1 = action_1 as i32 as f32;
        let action_2 = action_2 as i32 as f32;

        let distance = t_delta * PADDLE_SPEED;
        game_state.paddle_1.position.y += distance * action_1;
        game_state.paddle_2.position.y += distance * action_2;

        Self::clamp_paddle(self,&mut game_state.paddle_1);
        Self::clamp_paddle(self,&mut game_state.paddle_2);

        /*
        let new_ball_position = Self::clamp_vector(
            self.ball.position + self.ball.direction,
            Vector {x: 15.0, y: 15.0},
            Vector {x: LCD_WIDTH - 15.0, y: LCD_HEIGHT - 15.0}
        );

        */

        let new_ball_position = game_state.ball.position + game_state.ball.direction * Vector { x: t_delta, y: t_delta };
        game_state.ball.position = new_ball_position;

        let game_state = self.reflect(game_state);
        let game_state = self.crash(game_state);
        let game_state = Self::detect_collision(game_state, old_state);

        game_state

        // Problem: Need to update y_pos in controller, but cqannot do it agnostically
    }
}
