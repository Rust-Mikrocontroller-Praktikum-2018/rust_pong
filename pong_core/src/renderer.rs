extern crate alloc;
use math::Vector;
use alloc::LinkedList;
use pong_core::framebuffer::FrameBuffer;

fn draw_state (state: &GameState) {
    drawcircle(&state.ball.position);
    drawrectangle(&state.paddle_1.position); 
    drawrectangle(&state.paddle_2.position);
    drawscore(&state.score1,&state.score2);
}

struct CurrentCoordiante {
    x: i32,
    y: i32,
}

struct Circle {
    curr: CurrentCoordiante,
    diameter: i32,
}

struct Rectangle {
    curr: CurrentCoordiante,
    length_x: i32,
    length_y: i32,
}

struct Point {
    x: i32,
    y: i32,
}

fn drawrectangle ( rect: &rect) {
    draw(calculate_pixels_rectangle( rect ));
}


fn drawcircle( circ: &Circle){
     draw(calculate_pixels_circle( circ ));
}



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

fn quadrat (value: i32) -> i32 {
    value*value
}



fn draw (points: LinkedList<Point>) {
     
    for element in points
    {
        //layer.print_point_color_at(element.x,element.y,farbe);
    }
}
