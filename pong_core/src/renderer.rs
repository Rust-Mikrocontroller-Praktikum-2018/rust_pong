use math::Vector;
use alloc::LinkedList;
use framebuffer::FrameBuffer;
use pong::GameState;

/*
trait Drawable {
    fn draw(&self) -> LinkedList<Point>;
}

fn draw_state(state: &GameState, frame_buffer: &mut FrameBuffer) {
    let mut objects: LinkedList<&Drawable> = LinkedList::new();

    objects.push_back(&Circle {
        position: state.ball.position,
        diameter: 5
    });

    let mut points: LinkedList<&Point> = LinkedList::new();
    for o in objects {
        points.append(&mut o.draw());
    }

    for p in points {
        frame_buffer.set_pixel(0xffffff, p.x, p.y);
    }
}


struct Circle {
    position: Vector<usize>,
    diameter: usize,
}

impl Drawable for Circle {
    fn draw(&self) -> LinkedList<Point> {
        let mut list: LinkedList<Point> = LinkedList::new();

        let radius = self.diameter/2;
        for i in self.position.x-radius..self.position.x+radius {
            //TODO sqrt()
            let mut pointbuttom = Point { x: i , y : (radius**2 - quadrat(self.position.x))};

            let mut pointsover = Point {x:i,y:-pointbuttom.y};
            list.push_back(pointbuttom);
            list.push_back(pointsover);
        }

        list
    }
}

struct Rectangle {
    curr: Vector<usize>,
    size: Vector<usize>,
}



struct Point {
    x: usize,
    y: usize,
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
*/