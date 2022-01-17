use std::iter::Sum;
use std::ops::{Mul, Sub};

pub fn dot<T>(a: &[T], b: &[T]) -> T
where
    T: Mul<Output = T> + Sum + Copy,
{
    let ita = a.iter();
    let join = ita.zip(b.iter());
    join.map(|(x, y)| (*x) * (*y)).sum()
}

/// this is a bad idea ###
pub fn cross<T>(left: &[T], right: &[T]) -> Vec<T>
where
    T: Mul<Output = T> + Sum + Copy + Sub<Output=T>,
{
    let mut out = vec![];
    let x = left[1] * right[2] - left[2] * right[1];
    let y = left[2] * right[0] - left[0] * right[2];
    let z = left[0] * right[1] - left[1] * right[0];
    out.push(x);
    out.push(y);
    out.push(z);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_int_array() {
        let r: [i32; 2] = [1, 2];
        let l: [i32; 2] = [1, 1];
        let expected: i32 = 3;
        assert_eq!(dot(&r, &l), expected);
    }

    #[test]
    fn test_dot_float_array() {
        let r: [f32; 2] = [1.0, 2.0];
        let l: [f32; 2] = [1.0, 1.0];
        let expected: f32 = 3.0;
        assert_eq!(dot(&r, &l), expected);
    }

    #[test]
    fn test_dot_float_vector() {
        let r: Vec<f32> = vec![1.0, 2.0];
        let l: Vec<f32> = vec![1.0, 1.0];
        let expected: f32 = 3.0;
        assert_eq!(dot(&r, &l), expected);
    }

    #[test]
    fn test_dot_float_vector_uneven() {
        let r: Vec<f32> = vec![1.0, 2.0];
        let l: Vec<f32> = vec![1.0, 1.0, 1.0];
        let expected: f32 = 3.0;
        assert_eq!(dot(&r, &l), expected);
        let r: Vec<f32> = vec![1.0, 2.0, 3.0];
        let l: Vec<f32> = vec![1.0, 1.0];
        let expected: f32 = 3.0;
        assert_eq!(dot(&r, &l), expected);
    }
}
