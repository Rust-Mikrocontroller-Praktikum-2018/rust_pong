use core::ops::{Add, Sub, Mul, Div};
use core::mem;
use core::{f32, f64};


#[derive(Debug, Eq, Copy, Clone)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Vector<T> {
    pub fn new(value: T) -> Vector<T> {
        Vector {
            x: value,
            y: value,
        }
    }
}

impl<T: PartialEq> PartialEq for Vector<T> {
    fn eq(&self, other: &Vector<T>) -> bool {
        self.x == other.x && self.y == other.y
    }
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

impl<T> Sub for Vector<T> where T:Sub {
    type Output = Vector<T::Output>;

    fn sub(self, rhs: Vector<T>) -> Vector<T::Output> {
        Vector{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul for Vector<T> where T:Mul {
    type Output = Vector<T::Output>;

    fn mul(self, rhs: Vector<T>) -> Vector<T::Output> {
        Vector{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> Div for Vector<T> where T:Div {
    type Output = Vector<T::Output>;

    fn div(self, rhs: Vector<T>) -> Vector<T::Output> {
        Vector{
            x: self.x / rhs.x,
            y: self.y / rhs.y,
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

pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

pub fn dot_product(a: Vector<f32>, b: Vector<f32>) -> f32 {
    a.x * b.x + a.y * b.y
}


pub fn cross_product(a: Vector<f32>, b: Vector<f32>) -> f32 {
    a.x * b.y - a.y * b.x
}

pub fn length(a: Vector<f32>) -> f32 {
    1.0 / (a.x * a.x + a.y * a.y).inv_sqrt32()
}

pub fn unit(a: Vector<f32>) -> Vector<f32> {
    let length = length(a);
    a / Vector::new(length)
}

pub fn signum(a: Vector<f32>) -> Vector<f32> {
    Vector {
        x: if a.x > 0.0 {1.0} else {-1.0},
        y: if a.y > 0.0 {1.0} else {-1.0},
    }
}

pub fn abs(a: Vector<f32>) -> Vector<f32> {
    Vector {
        x: if a.x >= 0.0 {a.x} else {-1.0 * a.x},
        y: if a.y >= 0.0 {a.y} else {-1.0 * a.y},
    }
}

// https://github.com/emkw/rust-fast_inv_sqrt/blob/master/src/lib.rs
pub trait InvSqrt32 {
    fn inv_sqrt32(self) -> f32;
}

impl InvSqrt32 for f32 {
    fn inv_sqrt32(self: f32) -> f32 {
        if cfg!(not(feature = "omit-checking")) {
            if self <= 0.0 {
                return f32::NAN;
            } else if self == f32::INFINITY {
                return 0.0;
            } else if self < f32::MIN_POSITIVE {
                return f32::INFINITY;
            }
        }

        // Magic number based on Chris Lomont work:
        // const MAGIC_U32: u32 = 0x5f375a86;
        // The Original Magic Number:
        // const MAGIC_32: u32 = 0x5f3759df;
        const threehalfs: f32 = 1.5f32;
        let x2: f32 = self * 0.5f32;
        let mut i: u32 = unsafe { mem::transmute(self) }; // evil floating point bit level hacking
        i = 0x5f375a86 - (i >> 1);                        // what the fuck?
        let y: f32 = unsafe { mem::transmute(i) };
        let y  = y * ( threehalfs - ( x2 * y * y ) );     // 1st iteration
//		y  = y * ( threehalfs - ( x2 * y * y ) );       // 2nd iteration, this can be removed

        return y;
    }
}