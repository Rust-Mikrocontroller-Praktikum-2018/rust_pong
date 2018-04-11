use alloc::boxed::Box;
use core::cmp::{min, max};
use core::option::{Option};

use math::{clamp, cross_product, dot_product, unit, length, signum, Vector};
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

impl Ball {
    fn default_ball(x: f32, y: f32) -> Ball {
        Ball {
            position: Vector { x: x, y: y },
            direction: Vector { x: 0.9, y: 1.1 },
            diameter: 25.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Paddle {
    pub position: Vector<f32>,
    pub height: f32,
    pub width: f32,
}

impl Paddle {
    fn default_paddle(x: f32, y: f32) -> Paddle {
        Paddle {
            position: Vector { x: x, y: y },
            width: 20.0,
            height: 80.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub position: Vector<f32>,
    pub direction: Vector<f32>,
}

#[derive(Debug, Copy, Clone)]
pub struct DirectionalEdge {
    pub position: Vector<f32>,
    pub direction: Vector<f32>,
}

impl CollisionEffect for DirectionalEdge {
    fn on_collision(&self, mut new_state: GameState, old_state: GameState, t: f32, u: f32) -> GameState {
        let n = unit(Vector {x: self.direction.y, y: -self.direction.x}); // + Vector::new((u - 0.5) / 0.5) * self.direction;
        let d = Vector::new((t-0.5) / 0.5) * unit(self.direction);

        new_state.ball.position = old_state.ball.position + Vector::new(u) * old_state.ball.direction;
        new_state.ball.direction = unit(n*signum(old_state.ball.direction)*Vector::new(-1.0) + d) * Vector::new(length(old_state.ball.direction));
        new_state.ball.position = new_state.ball.position + Vector::new(1.0 - u) * new_state.ball.direction;

        new_state
    }
}

impl CollisionEffect for Edge {
    fn on_collision(&self, mut new_state: GameState, old_state: GameState, t: f32, u: f32) -> GameState {
        let m = new_state;
        new_state.ball.position = old_state.ball.position + old_state.ball.direction * Vector {x: u, y: u};
        // Reflect ball
        let d = new_state.ball.direction;
        let n = unit(Vector {x: self.direction.y, y: -self.direction.x});

        new_state.ball.direction = Vector::new(length(d)) * unit(d - Vector::new(2.0) * (Vector::new(dot_product(d, n)) * n));
        new_state.ball.position = new_state.ball.position + new_state.ball.direction * Vector {x: (1.0-u), y: (1.0-u)};

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
        GameState {
            ball: Ball::default_ball(width / 2.0, height / 2.0),
            paddle_1: Paddle::default_paddle(PADDLE_OFFSET, height / 2.0),
            paddle_2: Paddle::default_paddle(width - PADDLE_OFFSET, height / 2.0),
            score_1: 0,
            score_2: 0,
            running: GameMode::NewGame,
        }
    }

    pub fn reset(&mut self, height: f32, width: f32) {
        self.ball = Ball::default_ball(width / 2.0, height / 2.0);
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

    fn crash(&self, mut game_state: GameState) -> GameState {
        if game_state.ball.position.x >= self.width - game_state.ball.diameter - 1.0 {
            game_state.score_1 = game_state.score_1 + 1;
            game_state.reset(self.height, self.width);
        } 

        if game_state.ball.position.x <= game_state.ball.diameter + 1.0 {
            game_state.score_2 = game_state.score_2 + 1;
            game_state.reset(self.height, self.width);
        }

        game_state
    }

    fn intersect(p: Vector<f32>, r: Vector<f32>, q: Vector<f32>, s: Vector<f32>) -> (f32, f32) {
        let t = cross_product((q - p), s) / cross_product(r, s);
        let u = cross_product((q - p), r) / cross_product(r, s);

        (t, u)
    }

    fn get_edges_for_border(&self) -> [Edge; 2] {
        let padding = 25.0;
        let top_edge = Edge {
            position: Vector {
                x: 0.0,
                y: padding,
            },
            direction: Vector {
                x: self.width, y: 0.0
            }
        };

        let bottom_edge = Edge {
            position: Vector {
                x: 0.0,
                y: self.height - padding,
            },
            direction: Vector {
                x: self.width, y: 0.0
            }
        };


        let mut edges = [
            top_edge,
            bottom_edge,
        ];

        edges
    }

    fn get_edges_for_paddle(paddle: Paddle) -> [DirectionalEdge; 4] {
        let left_edge = DirectionalEdge {
            position: Vector {
                x: paddle.position.x - paddle.width / 2.0,
                y: paddle.position.y - paddle.height / 2.0,
            },
            direction: Vector {
                x: 0.0, y: paddle.height
            }
        };

        let right_edge = DirectionalEdge {
            position: Vector {
                x: paddle.position.x + paddle.width / 2.0,
                y: paddle.position.y - paddle.height / 2.0,
            },
            direction: Vector {
                x: 0.0, y: paddle.height
            }
        };

        let top_edge = DirectionalEdge {
            position: Vector {
                x: paddle.position.x - paddle.width / 2.0,
                y: paddle.position.y - paddle.height / 2.0,
            },
            direction: Vector {
                x: paddle.width, y: 0.0
            }
        };

        let bottom_edge = DirectionalEdge {
            position: Vector {
                x: paddle.position.x - paddle.width / 2.0,
                y: paddle.position.y + paddle.height / 2.0,
            },
            direction: Vector {
                x: paddle.width, y: 0.0
            }
        };

        let mut edges = [
            left_edge,
            right_edge,
            top_edge,
            bottom_edge,
        ];

        edges
    }

    fn detect_collision_paddle(&self, paddle_new: &Paddle, paddle_old: Paddle, ball_new: Ball, ball_old: Ball) -> [Option<(f32, f32, DirectionalEdge)>; 4] {
        let edges = Self::get_edges_for_paddle(paddle_old);
        let mut collisions: [Option<(f32, f32, DirectionalEdge)>; 4] = [None; 4];

        for i in 0..4 {
            let e = edges[i];
            let (t, u) = Self::intersect(e.position, e.direction, ball_old.position, ball_old.direction);
            if cross_product(ball_old.direction, e.direction) != 0.0 && 0.0 <= t && t <= 1.0 && 0.0 <= u && u <= 1.0 {
                collisions[i] = Some((t, u, e));
            } else {
                collisions[i] = None;
            }
        }

        collisions

    }

    fn detect_collision_border(&self, ball_new: Ball, ball_old: Ball) -> [Option<(f32, f32, Edge)>; 2] {
        let edges = self.get_edges_for_border();
        let mut collisions: [Option<(f32, f32, Edge)>; 2] = [None; 2];

        for i in 0..2 {
            let e = edges[i];
            let (t, u) = Self::intersect(e.position, e.direction, ball_old.position, ball_old.direction);
            if cross_product(ball_old.direction, e.direction) != 0.0 && 0.0 <= t && t <= 1.0 && 0.0 <= u && u <= 1.0 {
                collisions[i] = Some((t, u, e));
            } else {
                collisions[i] = None;
            }
        }

        collisions

    }

    fn detect_collision(&self, new_state: GameState, old_state: GameState) -> GameState {
        let mut paddle_1_collisions = self.detect_collision_paddle(
            &new_state.paddle_1,
            old_state.paddle_1,
            new_state.ball,
            old_state.ball,
        );
        let mut paddle_2_collisions = self.detect_collision_paddle(
            &new_state.paddle_2,
            old_state.paddle_2,
            new_state.ball,
            old_state.ball,
        );

        let mut border_collisions = self.detect_collision_border(
            new_state.ball,
            old_state.ball,
        );


        let mut new_state= new_state;

        let paddle_collisions: [[Option<(f32, f32, DirectionalEdge)>; 4]; 2] = [paddle_1_collisions, paddle_2_collisions];
        let border_collisions: [[Option<(f32, f32, Edge)>; 2]; 1] = [border_collisions];

        for list in paddle_collisions.iter() {
            for c in list.iter() {
                match c {
                    &Some((t, u, effect)) => {
                            new_state = effect.on_collision(new_state, old_state, t, u);
                    },
                    &None => {}
                }
            }
        }

        for list in border_collisions.iter() {
            for c in list.iter() {
                match c {
                    &Some((t, u, effect)) => {
                        new_state = effect.on_collision(new_state, old_state, t, u);
                    },
                    &None => {}
                }
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

        let new_ball_position = game_state.ball.position + game_state.ball.direction * Vector { x: t_delta, y: t_delta };
        game_state.ball.position = new_ball_position;

        let game_state = self.crash(game_state);
        let game_state = self.detect_collision(game_state, old_state);

        game_state

        // Problem: Need to update y_pos in controller, but cqannot do it agnostically
    }
}
