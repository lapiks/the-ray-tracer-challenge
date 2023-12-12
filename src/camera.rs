use glam::{Vec2, Vec3, Mat4};

use crate::{Canvas, ray::Ray, World};

pub struct Camera {
    width: usize,
    height: usize,
    fov: f32,
    focal_length: f32,
    transform: Mat4,
    pixel_size: f32,
    half_width: f32,
    half_height: f32,
}

impl Camera {
    pub fn new(width: usize, height: usize, fov: f32) -> Self {
        let half_view = f32::tan(fov / 2.0);
        let aspect = width / height;
        let mut half_width = 0.0;
        let mut half_height = 0.0;
        if aspect >= 1 {
            half_width = half_view;
            half_height = half_view / aspect as f32;
        } else {
            half_width = half_view * aspect as f32;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.0) / width as f32;

        Self {
            width,
            height,
            fov,
            focal_length: 1.0,
            transform: Mat4::IDENTITY,
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn with_translation(mut self, x: f32, y: f32, z: f32) -> Self {
        self.transform = Mat4::from_translation(Vec3::new(x, y, z));
        self
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);

        let canvas_size = Vec2::new(self.width as f32, self.height as f32);
        let inv_canvas_size = 1.0 / canvas_size;
        let ratio = canvas_size.x / canvas_size.y;
        let viewport = Vec2::new(2.0 * ratio, 2.0);

        let camera_position = self.transform.col(3);

        for row in 0..self.height-1 {
            for col in 0..self.width-1 {
                let pixel_pos = Vec2::new(col as f32, row as f32);
                let uv = pixel_pos * inv_canvas_size * viewport.y - 1.0;
                let mut uvw = Vec3::new(uv.x, uv.y, self.focal_length);
                uvw.x *= ratio;
            
                let ray: Ray = Ray::new(
                    Vec3::new(camera_position.x, camera_position.y, camera_position.z), 
                    uvw.normalize()
                );
                canvas[self.height - 1 - row][col] = world.color_at(&ray);
            }
        }

        canvas
    }
}


#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn creating_a_camera() {
        let c = Camera::new(
            160,
            120, 
            PI / 2.0,
        );
        assert_eq!(c.width, 160);
        assert_eq!(c.height, 120);
        assert_eq!(c.fov, PI / 2.0);
        assert_eq!(c.transform, Mat4::IDENTITY);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(
            200,
            125, 
            PI / 2.0,
        );
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(
            125,
            200, 
            PI / 2.0,
        );
        assert_eq!(c.pixel_size, 0.01);
    }
}
