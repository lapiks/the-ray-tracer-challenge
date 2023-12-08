use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }
}

impl std::ops::Index<usize> for Canvas {
    type Output = [Color];

    // row major
    fn index(&self, row: usize) -> &[Color] {
        let start = row * self.width;

        &self.pixels[start..start + self.width]
    }
}

impl std::ops::IndexMut<usize> for Canvas {
    // row major
    fn index_mut(&mut self, row: usize) -> &mut [Color] {
        let start = row * self.width;

        &mut self.pixels[start..start + self.width]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for row in 0..c.height-1 {
            for col in 0..c.width-1 {
                assert_eq!(c[row][col], Color::black()); 
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut c = Canvas::new(10, 20);
        c[2][3] = Color::red();
        assert_eq!(c[2][3], Color::red());
    }
}