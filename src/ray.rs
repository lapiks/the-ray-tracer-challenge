use glam::{Vec3, Mat4};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn transform(&self, mat: &Mat4) -> Ray {
        Ray {
            origin:  mat.transform_point3(self.origin),
            direction: mat.transform_vector3(self.direction)
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Mat4;

    use super::*;

    // Creating and quering a ray
    #[test]
    fn new_ray() {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    // Computing a point from a distance
    #[test]
    fn ray_at() {
        let r = Ray::new(
            Vec3::new(2.0, 3.0, 4.0), 
            Vec3::new(1.0, 0.0, 0.0)
        );
        assert_eq!(r.at(0.0), Vec3::new(2.0, 3.0, 4.0));
        assert_eq!(r.at(1.0), Vec3::new(3.0, 3.0, 4.0));
        assert_eq!(r.at(-1.0), Vec3::new(1.0, 3.0, 4.0));
        assert_eq!(r.at(2.5), Vec3::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(
            Vec3::new(1.0, 2.0, 3.0), 
            Vec3::new(0.0, 1.0, 0.0)
        );
        let r2 = r.transform(&Mat4::from_translation(Vec3::new(3.0, 4.0, 5.0)));
        assert_eq!(r2.origin, Vec3::new(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(
            Vec3::new(1.0, 2.0, 3.0), 
            Vec3::new(0.0, 1.0, 0.0)
        );
        let r2 = r.transform(&Mat4::from_scale(Vec3::new(2.0, 3.0, 4.0)));
        assert_eq!(r2.origin, Vec3::new(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vec3::new(0.0, 3.0, 0.0));
    }
}