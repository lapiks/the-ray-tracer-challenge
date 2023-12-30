use glam::DVec3;

use crate::ray::Ray;
use super::shape::Hittable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    p1: DVec3,
    p2: DVec3,
    p3: DVec3,
    e1: DVec3,
    e2: DVec3,
    normal: DVec3,
}

impl Triangle {
    pub fn new(p1: DVec3, p2: DVec3, p3: DVec3) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = e2.cross(e1).normalize();
        Self {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal, 
        }
    }
}

impl Hittable for Triangle {
    fn intersect(&self, _: &Ray) -> Vec<f64> {
        vec![]
    }

    fn normal_at(&self, _: DVec3) -> DVec3 {
        self.normal
    }
}

#[cfg(test)]
mod tests {
    use glam::dvec3;
    use super::*;

    #[test]
    fn constructing_a_triangle() {
        let p1 = dvec3(0.0, 1.0, 0.0);
        let p2 = dvec3(-1.0, 0.0, 0.0);
        let p3 = dvec3(1.0, 0.0, 0.0);
        let t = Triangle::new(p1, p2, p3);
        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.e1, dvec3(-1.0, -1.0, 0.0));
        assert_eq!(t.e2, dvec3(1.0, -1.0, 0.0));
        assert_eq!(t.normal, dvec3(0.0, 0.0, -1.0));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = Triangle::new(
            dvec3(0.0, 1.0, 0.0),
            dvec3(-1.0, 0.0, 0.0),
            dvec3(1.0, 0.0, 0.0)
        );
        let n1 = t.normal_at(dvec3(0.0, 0.5, 0.0));
        let n2 = t.normal_at(dvec3(-0.5, 0.75, 0.0));
        let n3 = t.normal_at(dvec3(0.5, 0.25, 0.0));
        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }
}