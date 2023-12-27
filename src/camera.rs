use std::time::Instant;

use glam::{DVec3, DMat4, dvec3};
use rayon::prelude::*;

use crate::{Canvas, ray::Ray, World, Color};

pub struct Camera {
    width: usize,
    height: usize,
    fov: f64,
    transform: DMat4,
    transform_inverse: DMat4,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
    background: Color,
}

impl Camera {
    pub fn new(width: usize, height: usize, fov: f64) -> Self {
        let half_view = f64::tan(fov / 2.0);
        let aspect = width as f64 / height as f64;
        let half_width = 
            if aspect >= 1.0 { 
                half_view 
            } else { 
                half_view * aspect as f64 
            };
        let half_height = 
            if aspect >= 1.0 { 
                half_view / aspect as f64 
            } else { 
                half_view 
            };
        let pixel_size = (half_width * 2.0) / width as f64;

        Self {
            width,
            height,
            fov,
            transform: DMat4::IDENTITY,
            transform_inverse: DMat4::IDENTITY,
            pixel_size,
            half_width,
            half_height,
            background: Color::black(),
        }
    }

    pub fn with_transform(mut self, transform: DMat4) -> Self {
        self.transform = transform;
        self.transform_inverse = transform.inverse();
        self
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub fn render(&self, world: &World, max_recursions: u8) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);
        let now = Instant::now();

        canvas
            .pixels_mut()
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, color)| {
                let y = i / self.width;
                let x = i - y * self.width;
                let ray = self.ray_for_pixel(x, y);
                *color = world
                    .color_at(&ray, max_recursions)
                    .unwrap_or(self.background);
            });

        println!("Rendering finished in {:.2?} seconds", now.elapsed());

        canvas
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let world_x = self.half_width - (x as f64 + 0.5) * self.pixel_size;
        let world_y = self.half_height - (y as f64 + 0.5) * self.pixel_size;
        let pixel = self.transform_inverse.transform_point3(dvec3(world_x, world_y, -1.0));
        let origin = self.transform_inverse.transform_point3(DVec3::ZERO);
        let direction = (pixel - origin).normalize();
        Ray {
            origin,
            direction
        }
    }
}


#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use glam::dvec3;

    use crate::{world::tests::default_world, transformations::view_transform, Color};

    use super::*;

    const EPSILON: f64 = 0.00001;

    #[test]
    fn creating_a_camera() {
        let c = Camera::new(160, 120, PI / 2.0);
        assert_eq!(c.width, 160);
        assert_eq!(c.height, 120);
        assert_eq!(c.fov, PI / 2.0);
        assert_eq!(c.transform, DMat4::IDENTITY);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() < EPSILON);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() < EPSILON);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert!(r.origin.abs_diff_eq(dvec3(0.0, 0.0, 0.0), EPSILON));
        assert!(r.direction.abs_diff_eq(dvec3(0.0, 0.0, -1.0), EPSILON));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201,101,PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert!(r.origin.abs_diff_eq(dvec3(0.0, 0.0, 0.0), EPSILON));
        assert!(r.direction.abs_diff_eq(dvec3(0.66519, 0.33259, -0.66851), EPSILON));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let c = Camera::new(201,101,PI / 2.0)
            .with_transform(
                DMat4::from_rotation_y(PI / 4.0) 
              * DMat4::from_translation(dvec3(0.0, -2.0, 5.0))
            );
        let r = c.ray_for_pixel(100, 50);
        assert!(r.origin.abs_diff_eq(dvec3(0.0, 2.0, -5.0), EPSILON));
        assert!(r.direction.abs_diff_eq(dvec3(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0), EPSILON));
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = default_world();
        let c = Camera::new(11,11,PI / 2.0)
            .with_transform(
                view_transform(
                    dvec3(0.0, 0.0, -5.0),
                    dvec3(0.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0)    
                )
            );
        let image = c.render(&w, 1);
        assert_eq!(image[5][5], Color::new(0.38066, 0.47583, 0.2855));
    }
}
