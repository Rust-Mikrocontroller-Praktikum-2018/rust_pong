use math::Vector;
use alloc::LinkedList;
use framebuffer::FrameBuffer;
use pong::GameState;

trait Renderer {
    fn render(&self, framebuffer: FrameBuffer) -> FrameBuffer {
        framebuffer
    }
}

trait Drawable {
    fn draw(&self) -> LinkedList<Point>;
}

pub fn draw_state(state: &GameState, frame_buffer: &mut FrameBuffer) {

    let a = Circle {
        position: state.ball.position,
        diameter: 30
    };

    let mut objects: LinkedList<&Drawable> = LinkedList::new();

    objects.push_back(&a);

    let mut points: LinkedList<Point> = LinkedList::new();
    for o in objects {
        points.append(&mut o.draw());
    }

    for p in points {
        if p.x > 0 && p.y > 0 {
            frame_buffer.set_pixel(0xffffff, p.x as usize, p.y as usize);
        }
    }
}


struct Circle {
    position: Vector<i32>,
    diameter: i32,
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
        let mut x0 = self.position.x;
        let mut y0 = self.position.y;

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

struct Rectangle {
    curr: Vector<i32>,
    size: Vector<i32>,
}



struct Point {
    x: i32,
    y: i32,
}


fn quadrat (value: i32) -> i32 {
    value*value
}

/*
fn calculate_pixels_rectangle(rect: &Rectangle) -> LinkedList<Point> {
    
    let length_y = 15;
    let length_x = 8;

    let  leftedgeup =    Point{x : position.x - (length_x/2), y : position.y + (length_y/2)};
    let  leftedgedown =  Point{x : position.x - (length_x/2), y : position.y - (length_x/2)};
    let  rightedgeup =   Point{x : position.x + (length_x/2), y : position.y + (length_y/2)};
    let  rightedgedown = Point{x : position.x + (length_x/2), y : position.y - (length_y/2)};

    let mut list: LinkedList<Point> = LinkedList::new();
    
    for i in leftedgeup.x..=rightedgeup.x{
        let mut point = Point { x : i , y : leftedgeup.y};
        list.push_back(point);
    }

    for i in leftedgedown.x..=rightedgedown.x{
        let mut point = Point { x  :i , y : leftedgedown.y};
        list.push_back(point);
    }

    for i in leftedgedown.y..=leftedgeup.y{
        let mut point = Point { x: leftedgeup.x , y: i};
        list.push_back(point);
    }

    for i in rightedgedown.y..=rightedgeup.y{
        let mut point = Point { x: rightedgeup.x , y : i};
        list.push_back(point);
    }      

    list
}

fn calculate_pixels_circle (circ: &Circle) -> LinkedList<Point> {
     
    let radius : i32 = circ.diameter/2;
     
    let mut list: LinkedList<Point> = LinkedList::new();   

    for i in circ.curr.x-radius..circ.curr.x+radius {

        //TODO sqrt()
        let mut pointbuttom = Point { x: i , y : (quadrat(radius) - quadrat(circ.curr.x))};
    
        let mut pointsover = Point {x:i,y:-pointbuttom.y};
        list.push_back(pointbuttom);
        list.push_back(pointsover);
    }
    list
}




fn draw (points: LinkedList<Point>) {
     
    for element in points
    {
        //layer.print_point_color_at(element.x,element.y,farbe);
    }
}
*/
