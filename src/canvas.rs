use std::path::Path;

use crate::color::Color;

fn scale_color(color: &Color) -> (u8, u8, u8) {
    (
        scale_color_component(color.r),
        scale_color_component(color.g),
        scale_color_component(color.b),
    )
}

fn scale_color_component(component: f32) -> u8 {
    let component = if component < 0.0 {
        0.0
    } else if component > 1.0 {
        1.0
    } else {
        component
    };

    (component * 255.0) as u8
}

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

    pub fn export<P: AsRef<Path>>(&self, path: P) -> image::ImageResult<()> {
        let mut img = image::ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = &self[y as usize][x as usize];
            let (r, g, b) = scale_color(color);
            *pixel = image::Rgb([r, g, b]);
        }

        img.save(path)
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