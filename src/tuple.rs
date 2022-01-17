use num::{One, Zero};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq)]
pub struct Tuple<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T: Zero + One> Tuple<T> {
    fn vector(x: T, y: T, z: T) -> Tuple<T> {
        Tuple {
            x,
            y,
            z,
            w: T::zero(),
        }
    }

    fn point(x: T, y: T, z: T) -> Tuple<T> {
        Tuple {
            x,
            y,
            z,
            w: T::one(),
        }
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

impl<T> Tuple<T> where T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> {}

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
}
