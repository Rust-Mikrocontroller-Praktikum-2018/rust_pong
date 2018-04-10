use math::Vector;
use alloc::LinkedList;
use framebuffer::FrameBuffer;
use pong::GameState;
use display::Display;
use alloc::btree_set::BTreeSet;
use alloc::vec::Vec;
use core::cmp::Ordering;

pub struct Renderer {
    old_points: BTreeSet<Point>
}

impl<'a> Renderer {
    pub fn new() -> Self {
        Renderer {
            old_points: BTreeSet::new()
        }
    }

    pub fn render(&mut self, state: &GameState, display: &mut Display) {
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

        let mut objects: LinkedList<&Drawable> = LinkedList::new();
        objects.push_back(&ball);
        objects.push_back(&paddle_1);
        objects.push_back(&paddle_2);

       
        let mut new_points = BTreeSet::new();
        for o in objects {
            let mut points: LinkedList<Point> = o.draw();
            for p in points {
                new_points.insert(p);
            }
        }
        
        let points_to_remove: Vec<_> = self.old_points.difference(&new_points).cloned().collect();
        let points_to_draw: Vec<_> = new_points.difference(&self.old_points).cloned().collect();

        for p in points_to_remove {
            display.set_pixel(p.x as usize, p.y as usize, 0x000000);   
        }

        for p in points_to_draw {
            display.set_pixel(p.x as usize, p.y as usize, 0xffffff);
        }

        self.old_points = new_points;
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

#[derive(Eq)]
#[derive(Copy)]
struct Point {
    position: Vector<i32>,
    value: i32
}

impl Clone for Point {
    fn clone(&self) -> Point { 
        Point{x: self.x.clone(), y: self.y.clone()}
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.x < other.x {
            Ordering::Less
        } else if self.x > other.x {
            Ordering::Greater
        } else {
            self.y.cmp(&other.y)
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
        self.x == other.x && self.y == other.y
    }
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