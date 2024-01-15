use glam::{DVec3, dvec3};

use crate::{ray::Ray, intersection::{Intersections, Intersection}, Object, bounds::BoundingBox};
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

    pub fn p1(&self) -> DVec3 {
        self.p1
    }

    pub fn p2(&self) -> DVec3 {
        self.p2
    }

    pub fn p3(&self) -> DVec3 {
        self.p3
    }
}

impl Hittable for Triangle {
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        let mut xs = Intersections::new();
        
        let dir_cross_e2 = ray.direction.cross(self.e2);
        let det = self.e1.dot(dir_cross_e2);
        if det.abs() < f64::EPSILON { return xs }

        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = f * p1_to_origin.dot(dir_cross_e2);
        if u < 0.0 || u > 1.0 { return xs }

        let origin_cross_e1 = p1_to_origin.cross(self.e1);
        let v = f * ray.direction.dot(origin_cross_e1);
        if v < 0.0 || u + v > 1.0 { return xs }

        xs.push(
            Intersection::new(
                f * self.e2.dot(origin_cross_e1), 
                object
            )
            .with_u_v(u, v)
        );
        xs
    }

    fn normal_at(&self, _: DVec3, _: f64, _: f64) -> DVec3 {
        self.normal
    }

    fn bounds(&self) -> BoundingBox {
        BoundingBox::new(
            dvec3(
                f64::min(f64::min(self.p1.x, self.p2.x), self.p3.x),
                f64::min(f64::min(self.p1.y, self.p2.y), self.p3.y),
                f64::min(f64::min(self.p1.z, self.p2.z), self.p3.z)
            ),
            dvec3(
                f64::max(f64::max(self.p1.x, self.p2.x), self.p3.x),
                f64::max(f64::max(self.p1.y, self.p2.y), self.p3.y),
                f64::max(f64::max(self.p1.z, self.p2.z), self.p3.z)
            ),            
        )
    }
}

#[cfg(test)]
mod tests {
    use glam::dvec3;
    use crate::shapes::Shape;

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
        let n1 = t.normal_at(dvec3(0.0, 0.5, 0.0), 0.0, 0.0);
        let n2 = t.normal_at(dvec3(-0.5, 0.75, 0.0), 0.0, 0.0);
        let n3 = t.normal_at(dvec3(0.5, 0.25, 0.0), 0.0, 0.0);
        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    #[test]
    fn inersecting_a_ray_parallel_to_the_triangle() {
        let t = Triangle::new(
            dvec3(0.0, 1.0, 0.0),
            dvec3(-1.0, 0.0, 0.0),
            dvec3(1.0, 0.0, 0.0)
        );
        let o = Object::new(Shape::Triangle(t));
        let r = Ray::new(
            dvec3(0.0, -1.0, -2.0),
            dvec3(0.0, 1.0, 0.0)
        );
        let xs = o.intersect(&r);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn a_ray_misses_the_p1_p3_edge() {
        let t = Triangle::new(
            dvec3(0.0, 1.0, 0.0),
            dvec3(-1.0, 0.0, 0.0),
            dvec3(1.0, 0.0, 0.0)
        );
        let o = Object::new(Shape::Triangle(t));
        let r = Ray::new(
            dvec3(1.0, 1.0, -2.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let xs = o.intersect(&r);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let t = Triangle::new(
            dvec3(0.0, 1.0, 0.0),
            dvec3(-1.0, 0.0, 0.0),
            dvec3(1.0, 0.0, 0.0)
        );
        let o = Object::new(Shape::Triangle(t));
        let r = Ray::new(
            dvec3(-1.0, 1.0, -2.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let xs = o.intersect(&r);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn a_ray_misses_the_p2_p3_edge() {
        let t = Triangle::new(
            dvec3(0.0, 1.0, 0.0),
            dvec3(-1.0, 0.0, 0.0),
            dvec3(1.0, 0.0, 0.0)
        );
        let o = Object::new(Shape::Triangle(t));
        let r = Ray::new(
            dvec3(0.0, -1.0, -2.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let xs = o.intersect(&r);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = Triangle::new(
            dvec3(0.0, 1.0, 0.0),
            dvec3(-1.0, 0.0, 0.0),
            dvec3(1.0, 0.0, 0.0)
        );
        let o = Object::new(Shape::Triangle(t));
        let r = Ray::new(
            dvec3(0.0, 0.5, -2.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let xs = o.intersect(&r);
        assert_eq!(xs.count(), 1);
        assert_eq!(xs[0].t(), 2.0);
    }

    #[test]
    fn a_triangle_has_a_bounding_box() {
        let t = Triangle::new(
            dvec3(-3.0, 7.0, 2.0),
            dvec3(6.0, 2.0, -4.0),
            dvec3(2.0, -1.0, -1.0)
        );
        let bb = t.bounds();
        assert_eq!(bb.min(), dvec3(-3.0, -1.0, -4.0));
        assert_eq!(bb.max(), dvec3(6.0, 7.0, 2.0));
    }
}