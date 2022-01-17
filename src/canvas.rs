use super::color::{black, Color};

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

pub fn new_canvas(width: usize, height: usize) -> Canvas {
    let size = width * height;
    let mut v = Vec::with_capacity(size);
    for _i in 0..size {
        v.push(black());
    }
    Canvas {
        width,
        height,
        pixels: v,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_canvas() {
        let w = 4;
        let h = 3;
        let c = new_canvas(w, h);
        assert_eq!(c.width, w);
        assert_eq!(c.height, h);
        assert_eq!(c.pixels.len(), w * h);
    }
}
