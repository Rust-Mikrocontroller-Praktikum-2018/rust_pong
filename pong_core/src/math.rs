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
