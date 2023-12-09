use glam::{Vec2, Vec3, Mat4};

use crate::{Canvas, Color, ray::Ray, World};

pub struct Camera {
    width: usize,
    height: usize,
    focal_length: f32,
    transform: Mat4,
}

impl Camera {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            focal_length: 1.0,
            transform: Mat4::IDENTITY,
        }
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);

        let canvas_size = Vec2::new(self.width as f32, self.height as f32);
        let inv_canvas_size = 1.0 / canvas_size;
        let ratio = canvas_size.x / canvas_size.y;
        let viewport = Vec2::new(2.0 * ratio, 2.0);

        for row in 0..self.height-1 {
            for col in 0..self.width-1 {
                let pixel_pos = Vec2::new(col as f32, row as f32);
                let uv = pixel_pos * inv_canvas_size * viewport.y - 1.0;
                let mut uvw = Vec3::new(uv.x, uv.y, self.focal_length);
                uvw.x *= ratio;
            
                let ray: Ray = Ray::new(&Vec3::new(0.0, 0.0, -5.0), &uvw);
                canvas[row][col] = world.color_at(&ray);;
            }
        }

        canvas
    }
}
