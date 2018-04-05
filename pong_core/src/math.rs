use core::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}