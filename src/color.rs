use super::tuple::Tuple;

pub type ColorF = Tuple<f32>;
pub type Color = Tuple<u8>;

pub fn color(r: u8, b: u8, g: u8) -> Color {
    Tuple::point(r, b, g)
}

pub fn colorf(r: f32, b: f32, g: f32) -> ColorF {
    Tuple::point(r, b, g)
}

pub fn black() -> Color {
    Tuple::point(0, 0, 0)
}

pub fn red() -> Color {
    Tuple::point(255, 0, 0)
}

pub fn green() -> Color {
    Tuple::point(0, 255, 0)
}

pub fn blue() -> Color {
    Tuple::point(0, 0, 255)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let rcvd = color(1, 2, 3);
        assert_eq!(rcvd.x, 1);
        assert_eq!(rcvd.y, 2);
        assert_eq!(rcvd.z, 3);
        assert_eq!(rcvd.w, 1);
    }
}
