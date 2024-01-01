use std::mem::swap;

use glam::DVec3;

use crate::{ray::Ray, Object, intersection::{Intersections, Intersection}};
use super::shape::Hittable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {}

impl Hittable for Cube {
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
            let tmin_numerator = -1.0 - origin;
            let tmax_numerator = 1.0 - origin;

            let mut tmin;
            let mut tmax;
            if direction.abs() >= f64::EPSILON {
                tmin = tmin_numerator / direction;
                tmax = tmax_numerator / direction;
            } else {
                tmin = tmin_numerator * f64::INFINITY;
                tmax = tmax_numerator * f64::INFINITY;
            }

            if tmin > tmax {
                swap(&mut tmin, &mut tmax);
            }

            (tmin, tmax)
        }

        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

        let tmin = f64::max(xtmin, f64::max(ytmin, ztmin));
        let tmax = f64::min(xtmax, f64::min(ytmax, ztmax));

        if tmin > tmax {
            Intersections::new()
        } else {
            let mut xs = Intersections::new();
            xs.push(Intersection::new(tmin, object));
            xs.push(Intersection::new(tmax, object));
            xs
        }
    }

    fn normal_at(&self, point: DVec3) -> DVec3 {
        let maxc = f64::max(point.x.abs(), f64::max(point.y.abs(), point.z.abs()));
        if maxc == point.x.abs() { 
            DVec3::new(point.x, 0.0, 0.0)
        } else if maxc == point.y.abs() {
            DVec3::new(0.0, point.y, 0.0)
        } else {
            DVec3::new(0.0, 0.0, point.z)
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self {}
    }
}

impl Cube {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use glam::DVec3;
    use crate::shapes::Shape;

    use super::*;

    #[test]
    fn a_ray_intersects_a_cube() {
        let datas = vec![
            (DVec3::new(5.0, 0.5, 0.0), DVec3::new(-1.0, 0.0, 0.0), 4.0, 6.0),
            (DVec3::new(-5.0, 0.5, 0.0), DVec3::new(1.0, 0.0, 0.0), 4.0, 6.0),
            (DVec3::new(0.5, 5.0, 0.0), DVec3::new(0.0, -1.0, 0.0), 4.0, 6.0),
            (DVec3::new(0.5, -5.0, 0.0), DVec3::new(0.0, 1.0, 0.0), 4.0, 6.0),
            (DVec3::new(0.5, 0.0, 5.0), DVec3::new(0.0, 0.0, -1.0), 4.0, 6.0),
            (DVec3::new(0.5, 0.0, -5.0), DVec3::new(0.0, 0.0, 1.0), 4.0, 6.0),
            (DVec3::new(0.0, 0.5, 0.0), DVec3::new(0.0, 0.0, 1.0), -1.0, 1.0),
        ];

        let c = Object::new(Shape::Cube(Cube::default()));

        for data in datas {
            let r = Ray::new(
                data.0, 
                data.1
            );
    
            let xs = c.intersect(&r);
            assert_eq!(xs.count(), 2);
            assert_eq!(xs[0].t(), data.2);
            assert_eq!(xs[1].t(), data.3);
        }
    }

    #[test]
    fn a_ray_intersects_misses_a_cube() {
        let datas = vec![
            (DVec3::new(-2.0, 0.0, 0.0), DVec3::new(0.2673, 0.5345, 0.8018)),
            (DVec3::new(0.0, -2.0, 0.0), DVec3::new(0.8018, 0.2673, 0.5345)),
            (DVec3::new(0.0, 0.0, -2.0), DVec3::new(0.5345, 0.8018, 0.2673)),
            (DVec3::new(2.0, 0.0, 2.0), DVec3::new(0.0, 0.0, -1.0)), 
            (DVec3::new(0.0, 2.0, 2.0), DVec3::new(0.0, -1.0, 0.0)),
            (DVec3::new(2.0, 2.0, 0.0), DVec3::new(-1.0, 0.0, 0.0)), 
        ];

        let c = Object::new(Shape::Cube(Cube::default()));

        for data in datas {
            let r = Ray::new(
                data.0, 
                data.1
            );
    
            let xs = c.intersect(&r);
            assert_eq!(xs.count(), 0);
        }
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = Cube::default();
        let points_normals = vec![
            (DVec3::new(1.0, 0.5, -0.8), DVec3::new(1.0, 0.0, 0.0)), 
            (DVec3::new(-1.0, -0.2, 0.9), DVec3::new(-1.0, 0.0, 0.0)), 
            (DVec3::new(-0.4, 1.0, -0.1), DVec3::new(0.0, 1.0, 0.0)), 
            (DVec3::new(0.3, -1.0, -0.7), DVec3::new(0.0, -1.0, 0.0)), 
            (DVec3::new(-0.6, 0.3, 1.0), DVec3::new(0.0, 0.0, 1.0)), 
            (DVec3::new(0.4, 0.4, -1.0), DVec3::new(0.0, 0.0, -1.0)), 
            (DVec3::new(1.0, 1.0, 1.0), DVec3::new(1.0, 0.0, 0.0)), 
            (DVec3::new(-1.0, -1.0, -1.0), DVec3::new(-1.0, 0.0, 0.0)), 
        ];

        for point_normal in points_normals {
            assert_eq!(c.normal_at(point_normal.0), point_normal.1);
        }
    }
}