use pong_core::math::Vector;
use pong_core::framebuffer::FrameBuffer;
use pong_core::pong::GameState;
use pong_core::display::Display;
use alloc::vec::Vec;
use alloc::binary_heap::BinaryHeap;
use core::cmp::Ordering;

use stm32f7::{system_clock};

pub struct Renderer {
    old_points: Vec<Point>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            old_points: Vec::with_capacity(100),
        }
    }

    pub fn render(&mut self, state: &GameState, display: &mut Display) {
        let start_render = system_clock::ticks();

        let ball = Circle {
            position: Vector::from(state.ball.position),
            diameter: state.ball.diameter as i32,
        };


        let paddle_1 = Rectangle {
            position: Vector::from(state.paddle_1.position),
            height: state.paddle_1.height as i32,
            width: state.paddle_1.width as i32,

        };

        let paddle_2 = Rectangle {
            position: Vector::from(state.paddle_2.position),
            height: state.paddle_2.height as i32,
            width: state.paddle_2.width as i32,

        };

       
        let mut new_points = Vec::with_capacity(100);

        let start_draw_ball = system_clock::ticks();
        ball.draw(&mut new_points);
        //hprintln!("ball.draw(): {}", system_clock::ticks() - start_draw_ball);
        let start_draw_paddles= system_clock::ticks();
        paddle_1.draw(&mut new_points);
        paddle_2.draw(&mut new_points);
        //hprintln!("paddles.draw(): {}", system_clock::ticks() - start_draw_paddles);



        for p_old in self.old_points.iter() {
            display.set_pixel(p_old.position.x as usize, p_old.position.y as usize, 0x000000);
        }

        for p_new in new_points.iter() {
            display.set_pixel(p_new.position.x as usize, p_new.position.y as usize, 0xffffff);
        }

        self.old_points = new_points;

        //let start_show_score = system_clock::ticks();
        //display.show_score(state.score_1, state.score_2, 0xffffff);
        //hprintln!("display.show_score(): {}", system_clock::ticks() - start_show_score);

        //hprintln!("render(): {}", system_clock::ticks() - start_render);
    }
}


trait Drawable {
    fn draw(&self, set: &mut Vec<Point>);
}

struct Circle {
    position: Vector<i32>,
    diameter: i32,
}

struct Rectangle {
    position: Vector<i32>,
    height: i32,
    width: i32,
}

#[derive(Debug, Eq, Copy, Clone)]
struct Point {
    position: Vector<i32>,
    value: i32
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.position.x < other.position.x {
            Ordering::Less
        } else if self.position.x > other.position.x {
            Ordering::Greater
        } else {
            self.position.y.cmp(&other.position.y)
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.position.x == other.position.x && self.position.y == other.position.y
    }
}

fn quadrat (value: i32) -> i32 {
    value*value
}


impl Drawable for Circle {
    fn draw(&self, set: &mut Vec<Point>) {
        let radius = self.diameter/2;

        let mut x = radius - 1;
        let mut y = 0;
        let mut dx = 1;
        let mut dy = 1;
        let mut err = dx - self.diameter;
        let x0 = self.position.x;
        let y0 = self.position.y;

        while x >= y {
            set.push(Point {
                position: Vector {x: x0 + x, y: y0 + y},
                value: 0,
            });
            set.push(Point {
                position: Vector {x: x0 + y, y: y0 + x},
                value: 0,
            });
            set.push(Point {
                position: Vector {x: x0 -y, y: y0 + x},
                value: 0,
            });
            set.push(Point {
                position: Vector {x: x0 -x, y: y0 + y},
                value: 0,
            });
            set.push(Point {
                position: Vector {x: x0 -x, y: y0 - y},
                value: 0,
            });
            set.push(Point {
                position: Vector {x: x0 -y, y: y0 - x},
                value: 0,
            });
            set.push(Point {
                position: Vector {x: x0 + y, y: y0 - x},
                value: 0,
            });
            set.push(Point {
                position: Vector {x: x0 + x, y: y0 - y},
                value: 0,
            });

            if err <= 0 {
                y = y + 1;
                err = err + dy;
                dy += 2;
            }

            if err > 0 {
                x = x - 1;
                dx += 2;
                err += dx - self.diameter;
            }
        }
    }
}

impl Drawable for Rectangle {

    fn draw (&self, set: &mut Vec<Point>) {
        let y = self.position.y;
        let x = self.position.x;
        let height = self.height / 2;
        let width = self.width / 2;

        // Left Edge
        for y in y-height..y+height {
            let mut point = Point {
                position: Vector {x: x - width, y},
                value: 0xffffff,
            };
            set.push(point);
        }

        //Right Edge
        for y in y-height..y+height {
            let mut point = Point {
                position: Vector {x: x + width, y},
                value: 0xffffff,
            };
            set.push(point);
        }

        //Top Edge
        for x in x-width..x+width {
            let mut point = Point {
                position: Vector {x: x, y: y + height},
                value: 0xffffff,
            };
            set.push(point);
        }

        //Bottom Edge
        for x in x-width..x+width {
            let mut point = Point {
                position: Vector {x: x, y: y - height},
                value: 0xffffff,
            };
            set.push(point);
        }
    }
}