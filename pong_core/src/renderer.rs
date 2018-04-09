use math::Vector;
use alloc::LinkedList;
use framebuffer::FrameBuffer;
use pong::GameState;
use display::Display;

pub struct Renderer {

}

impl<'a> Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn render(&self, state: &GameState, display: &mut Display) {
        let a = Circle {
            position: Vector::from(state.ball.position),
            diameter: 30
        };


        let b = Rectangle {
            curr: Vector{
                x: 100,
                y: 100,
            } ,
        };

        let mut objects: LinkedList<&Drawable> = LinkedList::new();
        objects.push_back(&a);
        objects.push_back(&b);

        let mut points: LinkedList<Point> = LinkedList::new();
        for o in objects {
            points.append(&mut o.draw());
        }

        for p in points {
            if p.x > 0 && p.y > 0 {
                display.set_pixel(p.x as usize, p.y as usize, 0xffffff);
            }
        }
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
    curr: Vector<i32>,
//    size: Vector<i32>,
}

struct Point {
    x: i32,
    y: i32,
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
                x: x0 + x, y: y0 + y
            });
            list.push_back(Point {
                x: x0 + y, y: y0 + x
            });
            list.push_back(Point {
                x: x0 -y, y: y0 + x
            });
            list.push_back(Point {
                x: x0 -x, y: y0 + y
            });
            list.push_back(Point {
                x: x0 -x, y: y0 - y
            });
            list.push_back(Point {
                x: x0 -y, y: y0 - x
            });
            list.push_back(Point {
                x: x0 + y, y: y0 - x
            });
            list.push_back(Point {
                x: x0 + x, y: y0 - y
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
        let length_y = 90;
        let length_x = 17;

        let leftedgetop = Point { x: self.curr.x - (length_x / 2), y: self.curr.y + (length_y / 2) };
        let rightedgebuttom = Point { x: self.curr.x + (length_x / 2), y: self.curr.y - (length_y / 2) };
        //let  leftedgebuttom =  Point{x : position.x - (length_x/2), y : position.y - (length_x/2)};
        //let  rightedgetop =   Point{x : position.x + (length_x/2), y : position.y + (length_y/2)};

        let mut list: LinkedList<Point> = LinkedList::new();

        for x_coordinate in leftedgetop.x..rightedgebuttom.x {
            for y_coordinate in rightedgebuttom.y..leftedgetop.y{
                let mut point = Point { x: x_coordinate, y: y_coordinate };
                list.push_back(point);
            }
        }

        // Code for rectanlge non-solid
        /*
        for i in leftedgetop.x..=rightedgetop.x{
            let mut point = Point { x : i , y : leftedgetop.y};
            list.push_back(point);
        }

        for i in leftedgebuttom.x..=rightedgebuttom.x{
            let mut point = Point { x  :i , y : leftedgebuttom.y};
            list.push_back(point);
        }

        for i in leftedgebuttom.y..=leftedgetop.y{
            let mut point = Point { x: leftedgetop.x , y: i};
            list.push_back(point);
        }

        for i in rightedgebuttom.y..=rightedgetop.y{
            let mut point = Point { x: rightedgetop.x , y : i};
            list.push_back(point);
        }

        */
    list
    }
}