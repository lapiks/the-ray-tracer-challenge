use glam::{Vec2, Vec3, Mat4, vec3};

use crate::{Canvas, ray::Ray, World};

pub struct Camera {
    width: usize,
    height: usize,
    fov: f32,
    transform: Mat4,
    transform_inverse: Mat4,
    pixel_size: f32,
    half_width: f32,
    half_height: f32,
}

impl Camera {
    pub fn new(width: usize, height: usize, fov: f32) -> Self {
        let half_view = f32::tan(fov / 2.0);
        let aspect = width as f32 / height as f32;
        let half_width = 
            if aspect >= 1.0 { 
                half_view 
            } else { 
                half_view * aspect as f32 
            };
        let half_height = 
            if aspect >= 1.0 { 
                half_view / aspect as f32 
            } else { 
                half_view 
            };
        let pixel_size = (half_width * 2.0) / width as f32;

        Self {
            width,
            height,
            fov,
            transform: Mat4::IDENTITY,
            transform_inverse: Mat4::IDENTITY,
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;
        self.transform_inverse = transform.inverse();
        self
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);

        for y in 0..self.height-1 {
            for x in 0..self.width-1 {
                let ray = self.ray_for_pixel(x, y);
                canvas[y][x] = world.color_at(&ray);
            }
        }

        canvas
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let world_x = self.half_width - (x as f32 + 0.5) * self.pixel_size;
        let world_y = self.half_height - (y as f32 + 0.5) * self.pixel_size;
        let pixel = self.transform_inverse.transform_point3(vec3(world_x, world_y, -1.0));
        let origin = self.transform_inverse.transform_point3(Vec3::ZERO);
        let direction = (pixel - origin).normalize();
        Ray {
            origin,
            direction
        }
    }
}


#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use glam::vec3;

    use crate::{world::tests::default_world, transformations::view_transform, Color};

    use super::*;

    const EPSILON: f32 = 0.00001;

    #[test]
    fn creating_a_camera() {
        let c = Camera::new(160, 120, PI / 2.0);
        assert_eq!(c.width, 160);
        assert_eq!(c.height, 120);
        assert_eq!(c.fov, PI / 2.0);
        assert_eq!(c.transform, Mat4::IDENTITY);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert!(r.origin.abs_diff_eq(vec3(0.0, 0.0, 0.0), EPSILON));
        assert!(r.direction.abs_diff_eq(vec3(0.0, 0.0, -1.0), EPSILON));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201,101,PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert!(r.origin.abs_diff_eq(vec3(0.0, 0.0, 0.0), EPSILON));
        assert!(r.direction.abs_diff_eq(vec3(0.66519, 0.33259, -0.66851), EPSILON));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let c = Camera::new(201,101,PI / 2.0)
            .with_transform(
                Mat4::from_rotation_y(PI / 4.0) 
              * Mat4::from_translation(vec3(0.0, -2.0, 5.0))
            );
        let r = c.ray_for_pixel(100, 50);
        assert!(r.origin.abs_diff_eq(vec3(0.0, 2.0, -5.0), EPSILON));
        assert!(r.direction.abs_diff_eq(vec3(2.0_f32.sqrt() / 2.0, 0.0, -2.0_f32.sqrt() / 2.0), EPSILON));
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = default_world();
        let c = Camera::new(11,11,PI / 2.0)
            .with_transform(
                view_transform(
                    vec3(0.0, 0.0, -5.0),
                    vec3(0.0, 0.0, 0.0),
                    vec3(0.0, 1.0, 0.0)    
                )
            );
        let image = c.render(&w);
        assert_eq!(image[5][5], Color::new(0.38066, 0.47583, 0.2855));
    }
}
