use core::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add for Vector<T> where T:Add {
    type Output = Vector<T::Output>;

    fn add(self, rhs: Vector<T>) -> Vector<T::Output> {
        Vector{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<Vector<f32>> for Vector<i32> {
    fn from(v: Vector<f32>) -> Self {
        Vector {
            x: v.x as i32,
            y: v.y as i32,
        }
    }
}