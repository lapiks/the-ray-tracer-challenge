use glam::{DVec3, DMat4, dvec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
    min: DVec3,
    max: DVec3,
}

impl Default for Bounds {
    fn default() -> Self {
        Self { 
            min: DVec3::splat(f64::INFINITY), 
            max: DVec3::splat(f64::NEG_INFINITY)
        }
    }
}

impl Bounds {
    pub fn new(min: DVec3, max: DVec3) -> Self{
        Self {
            min, max
        }
    }

    pub fn min(&self) -> DVec3 {
        self.min
    }

    pub fn max(&self) -> DVec3 {
        self.max
    }

    pub fn expand(&mut self, other: &Bounds) {
        self.add_point(other.min);
        self.add_point(other.max);
    }

    pub fn add_point(mut self, new_point: DVec3) -> Self {
        self.min = dvec3(
            f64::min(self.min.x, new_point.x),
            f64::min(self.min.y, new_point.y),
            f64::min(self.min.z, new_point.z),
        );
        self.max = dvec3(
            f64::max(self.max.x, new_point.x),
            f64::max(self.max.y, new_point.y),
            f64::max(self.max.z, new_point.z),
        );
        self
    }

    pub fn transform(self, matrix: &DMat4) -> Self {
        let p0 = self.min;
        let p1 = dvec3(self.max.x, self.min.y, self.min.z);
        let p2 = dvec3(self.min.x, self.max.y, self.min.z);
        let p3 = dvec3(self.max.x, self.max.y, self.min.z);
        let p4 = dvec3(self.min.x, self.min.y, self.max.z);
        let p5 = dvec3(self.max.x, self.min.y, self.max.z);
        let p6 = dvec3(self.min.x, self.max.y, self.max.z);
        let p7 = self.max;

        Self::default()
        .add_point(matrix.transform_point3(p0))
        .add_point(matrix.transform_point3(p1))
        .add_point(matrix.transform_point3(p2))
        .add_point(matrix.transform_point3(p3))
        .add_point(matrix.transform_point3(p4))
        .add_point(matrix.transform_point3(p5))
        .add_point(matrix.transform_point3(p6))
        .add_point(matrix.transform_point3(p7))
    }
}


#[cfg(test)]
mod tests {
    use glam::dvec3;

    use crate::{Object, shapes::{Sphere, Shape}};

    use super::*;

    #[test]
    fn creating_a_new_bounds() {
        let bb = Bounds::new(
            dvec3(1.0, 0.0, 0.0), 
            dvec3(0.0, 1.0, 0.0), 
        );

        assert_eq!(bb.min(), dvec3(1.0, 0.0, 0.0));
        assert_eq!(bb.max(), dvec3(0.0, 1.0, 0.0));
    }

    #[test]
    fn bounds_with_an_untransformed_sphere() {
        let s = Object::new(Shape::Sphere(Sphere::default()));

        assert_eq!(s.bounds().min, dvec3(-1.0, -1.0, -1.0));
        assert_eq!(s.bounds().max, dvec3(1.0, 1.0, 1.0));
    }

    #[test]
    fn bounds_with_a_transformed_sphere() {
        let s = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(5.0, 0.0, 0.0)
        .transform();

        assert_eq!(s.bounds().min, dvec3(4.0, -1.0, -1.0));
        assert_eq!(s.bounds().max, dvec3(6.0, 1.0, 1.0));
    }
}