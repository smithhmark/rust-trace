use num::{One, Zero};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq)]
pub struct Tuple<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Zero + One> Tuple<T> {
    pub fn vector(x: T, y: T, z: T) -> Tuple<T> {
        Tuple {
            x,
            y,
            z,
            w: T::zero(),
        }
    }

    pub fn point(x: T, y: T, z: T) -> Tuple<T> {
        Tuple {
            x,
            y,
            z,
            w: T::one(),
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Tuple<T> {
    fn dot(&self, right: &Self) -> T {
        let x = self.x * right.x;
        let y = self.y * right.y;
        let z = self.z * right.z;
        let w = self.w * right.w;
        x + y + z + w
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy + Zero + One> Tuple<T> {
    pub fn cross(&self, right: &Self) -> Self {
        let x = self.y * right.z - self.z * right.y;
        let y = self.z * right.x - self.x * right.z;
        let z = self.x * right.y - self.y * right.x;
        Tuple::vector(x, y, z)
    }
}

impl<T: Mul<Output = T> + Copy> Tuple<T> {
    pub fn hadamard(&self, rhs: &Self) -> Self {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        let w = self.w * rhs.w;
        Tuple { x, y, z, w }
    }
}

impl<T: Add<Output = T>> Add for Tuple<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Tuple<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Tuple<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        let w = self.w * rhs;
        Tuple { x, y, z, w }
    }
}

impl<T: Mul<Output = T> + Copy> Mul for Tuple<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.hadamard(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let rcvd: Tuple<i32> = Tuple::vector(1, 2, 3);
        assert_eq!(rcvd.x, 1);
        assert_eq!(rcvd.y, 2);
        assert_eq!(rcvd.z, 3);
        assert_eq!(rcvd.w, 0);
    }

    #[test]
    fn test_vector_float() {
        let rcvd: Tuple<f32> = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(rcvd.x, 1.0);
        assert_eq!(rcvd.y, 2.0);
        assert_eq!(rcvd.z, 3.0);
        assert_eq!(rcvd.w, 0.0);
    }

    #[test]
    fn test_point() {
        let rcvd: Tuple<i32> = Tuple::point(1, 2, 3);
        assert_eq!(rcvd.x, 1);
        assert_eq!(rcvd.y, 2);
        assert_eq!(rcvd.z, 3);
        assert_eq!(rcvd.w, 1);
    }

    #[test]
    fn test_point_float() {
        let rcvd: Tuple<f32> = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!(rcvd.x, 1.0);
        assert_eq!(rcvd.y, 2.0);
        assert_eq!(rcvd.z, 3.0);
        assert_eq!(rcvd.w, 1.0);
    }

    #[test]
    fn test_vector_add() {
        let left: Tuple<usize> = Tuple::vector(1, 2, 3);
        let right: Tuple<usize> = Tuple::vector(3, 2, 1);
        let rcvd: Tuple<usize> = left + right;
        let expt: Tuple<usize> = Tuple::vector(4, 4, 4);
        assert_eq!(rcvd, expt);
    }

    #[test]
    fn test_point_add() {
        let left: Tuple<f32> = Tuple::point(1.0, 2.0, 3.0);
        let right: Tuple<f32> = Tuple::point(3.0, 2.0, 1.0);
        let rcvd: Tuple<f32> = left + right;
        let expt: Tuple<f32> = Tuple {
            x: 4.0,
            y: 4.0,
            z: 4.0,
            w: 2.0,
        };
        assert_eq!(rcvd, expt);
    }

    #[test]
    fn test_subtract() {
        let left: Tuple<f32> = Tuple {
            x: 0.2,
            y: 0.4,
            z: 0.6,
            w: 0.8,
        };
        let right: Tuple<f32> = Tuple {
            x: 0.1,
            y: 0.2,
            z: 0.3,
            w: 0.4,
        };
        let exp: Tuple<f32> = Tuple {
            x: 0.1,
            y: 0.2,
            z: 0.3,
            w: 0.4,
        };
        let res = left - right;
        assert_eq!(res, exp);
    }

    #[test]
    fn test_dot() {
        let left: Tuple<usize> = Tuple::vector(1, 2, 3);
        let right = Tuple::vector(2, 3, 4);
        let exp = 20;
        let res = Tuple::dot(&left, &right);
        assert_eq!(res, exp);
        let res = left.dot(&right);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_cross() {
        let a: Tuple<i32> = Tuple::vector(1, 2, 3);
        let b: Tuple<i32> = Tuple::vector(2, 3, 4);
        let c: Tuple<i32> = Tuple::vector(-1, 2, -1);
        let d: Tuple<i32> = Tuple::vector(1, -2, 1);
        assert_eq!(a.cross(&b), c);
        assert_eq!(b.cross(&a), d);
    }

    #[test]
    fn test_hadamard() {
        let a: Tuple<i32> = Tuple::vector(1, 2, 3);
        let b: Tuple<i32> = Tuple::vector(1, 1, 1);
        assert_eq!(a.hadamard(&b), a);
    }

    #[test]
    fn test_mul_scaling() {
        let a: Tuple<i32> = Tuple::vector(1, 2, 3);
        let b: Tuple<i32> = Tuple::vector(2, 4, 6);
        let val: i32 = 2;
        assert_eq!(a * val, b);
    }

    #[test]
    fn test_mul_hadamard() {
        let a: Tuple<i32> = Tuple::vector(1, 2, 3);
        let b: Tuple<i32> = Tuple::vector(2, 2, 2);
        let exp: Tuple<i32> = Tuple::vector(2, 4, 6);
        assert_eq!(a * b, exp);
    }
}
