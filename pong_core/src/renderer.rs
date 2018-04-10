use math::Vector;
use alloc::LinkedList;
use framebuffer::FrameBuffer;
use pong::GameState;
use display::Display;

pub struct Renderer {
    old_points: LinkedList<Point>
}

impl<'a> Renderer {
    pub fn new() -> Self {
        Renderer {
            old_points: LinkedList::new()
        }
    }

    pub fn render(&mut self, state: &GameState, display: &mut Display) {
        let ball = Circle {
            position: Vector::from(state.ball.position),
            diameter: 30
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

        let mut objects: LinkedList<&Drawable> = LinkedList::new();
        objects.push_back(&ball);
        objects.push_back(&paddle_1);
        objects.push_back(&paddle_2);

        let mut points: LinkedList<Point> = LinkedList::new();

        for o in objects {
            points.append(&mut o.draw());
        }

        for p in &self.old_points {
            display.set_pixel(p.position.x as usize, p.position.y as usize, 0x000000);
        }

        for p in &points {
            display.set_pixel(p.position.x as usize, p.position.y as usize, 0xffffff);
        }

        self.old_points = points;
    }
}


trait Drawable {
    fn draw(&self) -> LinkedList<Point>;
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

struct Point {
    position: Vector<i32>,
    value: i32
}


fn quadrat (value: i32) -> i32 {
    value*value
}



impl Drawable for Circle {
    fn draw(&self) -> LinkedList<Point> {
        let mut list: LinkedList<Point> = LinkedList::new();
        let radius = self.diameter/2;

        let mut x = radius - 1;
        let mut y = 0;
        let mut dx = 1;
        let mut dy = 1;
        let mut err = dx - self.diameter;
        let x0 = self.position.x;
        let y0 = self.position.y;

        while x >= y {
            list.push_back(Point {
                position: Vector {x: x0 + x, y: y0 + y},
                value: 0,
            });
            list.push_back(Point {
                position: Vector {x: x0 + y, y: y0 + x},
                value: 0,
            });
            list.push_back(Point {
                position: Vector {x: x0 -y, y: y0 + x},
                value: 0,
            });
            list.push_back(Point {
                position: Vector {x: x0 -x, y: y0 + y},
                value: 0,
            });
            list.push_back(Point {
                position: Vector {x: x0 -x, y: y0 - y},
                value: 0,
            });
            list.push_back(Point {
                position: Vector {x: x0 -y, y: y0 - x},
                value: 0,
            });
            list.push_back(Point {
                position: Vector {x: x0 + y, y: y0 - x},
                value: 0,
            });
            list.push_back(Point {
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
        list
    }
}




impl Drawable for Rectangle {

    fn draw (&self) -> LinkedList<Point> {
        let y = self.position.y;
        let x = self.position.x;
        let height = self.height / 2;
        let width = self.width / 2;

        let mut list: LinkedList<Point> = LinkedList::new();

        // Left Edge
        for y in y-height..y+height {
            let mut point = Point {
                position: Vector {x: x - width, y},
                value: 0xffffff,
            };
            list.push_back(point);
        }

        //Right Edge
        for y in y-height..y+height {
            let mut point = Point {
                position: Vector {x: x + width, y},
                value: 0xffffff,
            };
            list.push_back(point);
        }

        //Top Edge
        for x in x-width..x+width {
            let mut point = Point {
                position: Vector {x: x, y: y + height},
                value: 0xffffff,
            };
            list.push_back(point);
        }

        //Bottom Edge
        for x in x-width..x+width {
            let mut point = Point {
                position: Vector {x: x, y: y - height},
                value: 0xffffff,
            };
            list.push_back(point);
        }

        list
    }
}