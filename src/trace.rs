
pub type Floats = [f32; 4];

pub fn vector(x: f32, y: f32, z: f32) -> Floats {
    [x, y, z, 0.0]
}

pub fn point(x: f32, y: f32, z: f32) -> Floats {
    [x, y, z, 1.0]
}

pub fn equal(left: &Floats, right: &Floats, ep: f32) -> bool {
    let mut pairs = left.iter().zip(right.iter());
    let has_misses = pairs.any(|(l, r)| (l - r).abs() > ep);
    if has_misses {
        false
    } else {
        true
    }
}

pub fn eq(left: &Floats, right: &Floats) -> bool {
    equal(left, right, 0.000001)
}

pub fn add(left: &Floats, right: &Floats) -> Floats {
    let pairs = left.iter().zip(right.iter());
    let almost = pairs.map(|(l,r)| l+r).enumerate();
    let mut res = [0.0; 4];
    for (ii, val) in almost {
        res[ii] = val;
    }
    res
}

pub fn subtract(left: &Floats, right: &Floats) -> Floats {
    let pairs = left.iter().zip(right.iter());
    let almost = pairs.map(|(l,r)| l-r).enumerate();
    let mut res = [0.0; 4];
    for (ii, val) in almost {
        res[ii] = val;
    }
    res
}

pub fn scale(left: &Floats, right: f32) -> Floats {
    let almost = left.iter().map(|l| l * right).enumerate();
    let mut res = [0.0; 4];
    for (ii, val) in almost {
        res[ii] = val;
    }
    res
}

pub fn dot(left: &Floats, right: &Floats) -> f32 {
    let pairs = left.iter().zip(right.iter());
    pairs.map(|(l,r)| l*r).sum()
}

pub fn cross(left: &Floats, right: &Floats) -> Floats {
    let x = left[1]*right[2] - left[2]*right[1];
    let y = left[2]*right[0] - left[0]*right[2];
    let z = left[0]*right[1] - left[1]*right[0];
    vector(x, y, z)
}

pub fn blend(left: &Floats, right: &Floats) -> Floats {
    let x = left[0]*right[0];
    let y = left[1]*right[1];
    let z = left[2]*right[2];
    point(x, y, z)
}

pub fn magnatude(val: &Floats) -> f32 {
    let s: f32 = val.iter().map(|x| x*x).sum();
    s.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        let left = [0.1, 0.2, 0.3, 0.4];
        let right = [0.1, 0.2, 0.3, 0.4];
        let right2 = [0.2, 0.2, 0.3, 0.4];
        let epsilon = 0.0001;
        assert!(equal(&left, &right, epsilon));
        assert!(!equal(&left, &right2, epsilon));
        let epsilon = 1.0001;
        assert!(equal(&left, &right2, epsilon));
    }

    #[test]
    fn test_vector() {
        let x = 0.1;
        let y = 0.2;
        let z = 0.3;
        let expected = [x, y, z, 0.0];
        assert!(eq(&vector(x, y, z), &expected))
    }

    #[test]
    fn test_point() {
        let x = 0.1;
        let y = 0.2;
        let z = 0.3;
        let expected = [x, y, z, 1.0];
        assert!(eq(&point(x, y, z), &expected))
    }

    #[test]
    fn test_add() {
        let left = [0.1, 0.2, 0.3, 0.4];
        let right = [0.1, 0.2, 0.3, 0.4];
        let exp = [0.2, 0.4, 0.6, 0.8];
        let res = add(&left, &right);
        assert!(eq(&res, &exp));
    }

    #[test]
    fn test_subtract() {
        let left = [0.2, 0.4, 0.6, 0.8];
        let right = [0.1, 0.2, 0.3, 0.4];
        let exp = [0.1, 0.2, 0.3, 0.4];
        let res = subtract(&left, &right);
        assert!(eq(&res, &exp));
    }

    #[test]
    fn test_dot() {
        let left = vector(1.0, 2.0, 3.0);
        let right = vector(2.0, 3.0, 4.0);
        let exp = 20.0;
        let res = dot(&left, &right);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_magnatude() {
        let fteen: f32 = 14.0;
        let exp = fteen.sqrt();
        assert_eq!( magnatude(&vector(1.0, 0.0, 0.0)), 1.0);
        assert_eq!( magnatude(&vector(0.0, 1.0, 0.0)), 1.0);
        assert_eq!( magnatude(&vector(0.0, 0.0, 1.0)), 1.0);
        assert_eq!( magnatude(&vector(1.0, 2.0, 3.0)), exp);
        assert_eq!( magnatude(&vector(-1.0, -2.0, -3.0)), exp);
    }

    #[test]
    fn test_scale() {
        let initial = vector(1.0, 0.0, 0.0);
        assert_eq!( magnatude(&initial), 1.0);
        let scaled = scale(&initial, 5.0);
        assert_eq!( magnatude(&scaled), 5.0);
        assert_eq!( scaled[0], 5.0);
    }

    #[test]
    fn test_cross() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        let c = vector(-1.0, 2.0, -1.0);
        let d = vector(1.0, -2.0, 1.0);
        assert!(eq(&cross(&a,&b), &c));
        assert!(eq(&cross(&b,&a), &d));
    }
}

