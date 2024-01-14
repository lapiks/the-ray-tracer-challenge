use glam::DVec3;

use crate::{ray::Ray, intersection::Intersections, Object, bounds::Bounds};
use super::{shape::Hittable, Triangle};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SmoothTriangle {
    triangle: Triangle,
    n1: DVec3,
    n2: DVec3,
    n3: DVec3,
}

impl SmoothTriangle {
    pub fn new(p1: DVec3, p2: DVec3, p3: DVec3, n1: DVec3, n2: DVec3, n3: DVec3) -> Self {
        Self {
            triangle: Triangle::new(p1, p2, p3),
            n1,
            n2,
            n3,
        }
    }
}

impl Hittable for SmoothTriangle {
    fn intersect<'a>(&'a self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        self.triangle.intersect(ray, object)
    }

    fn normal_at(&self, _: DVec3, u: f64, v: f64) -> DVec3 {
        self.n2 * u +
        self.n3 * v +
        self.n1 * (1.0 - u - v)
    }

    fn bounds(&self) -> Bounds {
        self.triangle.bounds()
    }
}

#[cfg(test)]
mod tests {
    const EPSILON: f64 = 0.000001;

    use glam::dvec3;

    use crate::{shapes::{Triangle, Shape}, intersection::{Intersection, IntersectionInfos}};

    use super::*;

    fn default_smooth_triangle() -> SmoothTriangle {
        SmoothTriangle::new(
            dvec3(0.0, 1.0, 0.0),
            dvec3(-1.0, 0.0, 0.0),
            dvec3(1.0, 0.0, 0.0),
            dvec3(0.0, 1.0, 0.0),
            dvec3(-1.0, 0.0, 0.0),
            dvec3(1.0, 0.0, 0.0),
        )
    }

    #[test]
    fn constructing_a_smooth_triangle() {
        let t = default_smooth_triangle();
        assert_eq!(t.triangle.p1(), dvec3(0.0, 1.0, 0.0));
        assert_eq!(t.triangle.p2(), dvec3(-1.0, 0.0, 0.0));
        assert_eq!(t.triangle.p3(), dvec3(1.0, 0.0, 0.0));
        assert_eq!(t.n1, dvec3(0.0, 1.0, 0.0));
        assert_eq!(t.n2, dvec3(-1.0, 0.0, 0.0));
        assert_eq!(t.n3, dvec3(1.0, 0.0, 0.0));
    }

    #[test]
    fn an_intersection_can_encapsulate_u_and_v() {
        let t = Object::new(
            Shape::Triangle(
                Triangle::new(
                    dvec3(0.0, 1.0, 0.0), 
                    dvec3(-1.0, 0.0, 0.0),
                    dvec3(1.0, 0.0, 0.0)
                )
            )
        );

        let i = Intersection::new(
            3.5, 
            &t
        )
        .with_u_v(0.2, 0.4);

        assert_eq!(i.u(), 0.2);
        assert_eq!(i.v(), 0.4);
    }

    #[test]
    fn an_intersection_with_a_smooth_triangle_stores_u_v() {
        let t = Object::new(Shape::SmoothTriangle(default_smooth_triangle()));
        let r = Ray::new(
            dvec3(-0.2, 0.3, -2.0), 
            dvec3(0.0, 0.0, 1.0)  
        );
        let xs = t.intersect(&r);

        assert!(f64::abs(xs[0].u() - 0.45) < f64::EPSILON);
        assert!(f64::abs(xs[0].v() - 0.25) < f64::EPSILON);
    }

    #[test]
    fn a_smooth_triangle_uses_u_v_to_interpolate_the_normal() {
        let t = Object::new(Shape::SmoothTriangle(default_smooth_triangle()));
        let i = Intersection::new(
            1.0, 
            &t
        )
        .with_u_v(0.45, 0.25);

        assert!(
            t.normal_at(dvec3(0.0, 0.0, 0.0), i.u(), i.v())
            .abs_diff_eq(dvec3(-0.5547, 0.83205, 0.0), EPSILON) 
        );
    }

    #[test]
    fn preparing_the_normal_on_a_smooth_triangle() {
        let t = Object::new(Shape::SmoothTriangle(default_smooth_triangle()));
        let i = Intersection::new(
            1.0, 
            &t
        )
        .with_u_v(0.45, 0.25);
        let r = Ray::new(
            dvec3(-0.2, 0.3, -2.0), 
            dvec3(0.0, 0.0, 1.0)  
        );
        let xs = Intersections::new().with_intersections(vec![i]);
        let comps = IntersectionInfos::new(&xs, 0, &r);

        assert!(comps.normalv.abs_diff_eq(dvec3(-0.5547, 0.83205, 0.0), EPSILON));
    }
}