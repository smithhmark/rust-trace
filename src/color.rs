
use super::trace;

pub type ColorF = [f32; 4];
pub type Color = [i32; 3];

pub fn color(r: i32, b:i32, g:i32) -> Color {
    [r,b,g]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        assert!(true);
    }
}
