use std::mem::swap;

use glam::DVec3;

use crate::ray::Ray;
use super::shape::Hittable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {}

impl Hittable for Cube {
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
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
            Vec::default()
        } else {
            vec![tmin, tmax]
        }

        
    }

    fn normal_at(&self, point: DVec3) -> DVec3 {
        // for a unit sphere at 0,0,0
        point.normalize()
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
    use super::*;

    const EPSILON: f64 = 0.00001;

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

        let c = Cube::default();

        for data in datas {
            let r = Ray::new(
                data.0, 
                data.1
            );
    
            let xs = c.intersect(&r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0], data.2);
            assert_eq!(xs[1], data.3);
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

        let c = Cube::default();

        for data in datas {
            let r = Ray::new(
                data.0, 
                data.1
            );
    
            let xs = c.intersect(&r);
            assert_eq!(xs.len(), 0);
        }
    }
}