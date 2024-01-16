use std::mem::swap;

use glam::{DVec3, DMat4, dvec3};

use crate::ray::Ray;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    min: DVec3,
    max: DVec3,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self { 
            min: DVec3::splat(f64::INFINITY), 
            max: DVec3::splat(f64::NEG_INFINITY)
        }
    }
}

impl BoundingBox {
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

    pub fn merge(self, other: &BoundingBox) -> Self {
        self
        .add_point(other.min)
        .add_point(other.max)
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

    pub fn contains_point(&self, point: DVec3) -> bool {
        point.x >= self.min.x && point.y >= self.min.y && point.z >= self.min.z &&
        point.x <= self.max.x && point.y <= self.max.y && point.z <= self.max.z
    }

    pub fn contains_box(&self, other: &BoundingBox) -> bool {
        self.contains_point(other.min) && self.contains_point(other.max)
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

    pub fn intersects<'a>(&self, ray: &Ray) -> bool {
        fn check_axis(origin: f64, direction: f64, min: f64, max: f64) -> (f64, f64) {
            let tmin_numerator = min - origin;
            let tmax_numerator = max - origin;

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

        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x, self.min.x, self.max.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y, self.min.y, self.max.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z, self.min.z, self.max.z);

        let tmin = f64::max(xtmin, f64::max(ytmin, ztmin));
        let tmax = f64::min(xtmax, f64::min(ytmax, ztmax));

        tmin < tmax
    }
}


#[cfg(test)]
mod tests {
    use std::f64::{INFINITY, NEG_INFINITY, consts::PI};

    const EPSILON: f64 = 0.0001;

    use glam::dvec3;

    use crate::{Object, shapes::{Sphere, Shape, Group, test_shape::TestShape}, transformations::Transform};

    use super::*;
    
    #[test]
    fn creating_an_empty_bounding_box() {
        let bb = BoundingBox::default();

        assert_eq!(bb.min(), dvec3(INFINITY, INFINITY, INFINITY));
        assert_eq!(bb.max(), dvec3(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY));
    }

    #[test]
    fn creating_a_bounding_box_with_volume() {
        let bb = BoundingBox::new(
            dvec3(-1.0, -2.0, -3.0), 
            dvec3(3.0, 2.0, 1.0), 
        );

        assert_eq!(bb.min(), dvec3(-1.0, -2.0, -3.0));
        assert_eq!(bb.max(), dvec3(3.0, 2.0, 1.0));
    }

    #[test]
    fn adding_points_to_an_empty_bounding_box() {
        let bb = BoundingBox::default()
            .add_point(dvec3(-5.0, 2.0, 0.0))
            .add_point(dvec3(7.0, 0.0, -3.0));
        assert_eq!(bb.min(), dvec3(-5.0, 0.0, -3.0));
        assert_eq!(bb.max(), dvec3(7.0, 2.0, 0.0));
    }

    #[test]
    fn adding_one_bounding_box_to_another() {
        let bb = BoundingBox::default()
            .add_point(dvec3(-5.0, 2.0, 0.0))
            .add_point(dvec3(7.0, 4.0, 4.0))
            .merge(&BoundingBox::default()
                .add_point(dvec3(8.0, -7.0, -2.0))
                .add_point(dvec3(14.0, 2.0, 8.0))
            );

        assert_eq!(bb.min(), dvec3(-5.0, -7.0, -2.0));
        assert_eq!(bb.max(), dvec3(14.0, 4.0, 8.0));
    }

    #[test]
    fn checking_if_a_box_contains_a_given_point() {
        let bb = BoundingBox::default()
            .add_point(dvec3(5.0, -2.0, 0.0))
            .add_point(dvec3(11.0, 4.0, 7.0));

        assert_eq!(bb.contains_point(dvec3(5.0, -2.0, 0.0)), true);
        assert_eq!(bb.contains_point(dvec3(11.0, 4.0, 0.0)), true);
        assert_eq!(bb.contains_point(dvec3(8.0, 1.0, 3.0)), true);
        assert_eq!(bb.contains_point(dvec3(3.0, 0.0, 3.0)), false);
        assert_eq!(bb.contains_point(dvec3(8.0, -4.0, 3.0)), false);
        assert_eq!(bb.contains_point(dvec3(8.0, 1.0, -1.0)), false);
        assert_eq!(bb.contains_point(dvec3(13.0, 1.0, 3.0)), false);
        assert_eq!(bb.contains_point(dvec3(8.0, 5.0, 3.0)), false);
        assert_eq!(bb.contains_point(dvec3(8.0, 1.0, 8.0)), false);
    }

    #[test]
    fn checking_if_a_box_contains_a_given_box() {
        let bb = BoundingBox::default()
            .add_point(dvec3(5.0, -2.0, 0.0))
            .add_point(dvec3(11.0, 4.0, 7.0));

        let datas = vec![
            (dvec3(5.0, -2.0, 0.0), dvec3(11.0, 4.0, 7.0), true),
            (dvec3(6.0, -1.0, 1.0), dvec3(10.0, 3.0, 6.0), true),
            (dvec3(4.0, -3.0, -1.0), dvec3(10.0, 3.0, 6.0), false),
            (dvec3(6.0, -1.0, 1.0), dvec3(12.0, 5.0, 8.0), false),
        ];

        for data in datas {
            assert_eq!(
                bb.contains_box(
                    &BoundingBox::new(data.0, data.1)
                ), 
                data.2
            );
        }
    }

    #[test]
    fn transforming_a_bounding_box() {
        let bb = BoundingBox::new(
            dvec3(-1.0, -1.0, -1.0),
            dvec3(1.0, 1.0, 1.0)
        )
        .transform(
            &Transform::new()
            .with_rotation_y(PI / 4.0)
            .with_rotation_x(PI / 4.0)
            .matrix
        );

        assert!(bb.min.abs_diff_eq(dvec3(-1.4142, -1.7071, -1.7071), EPSILON));
        assert!(bb.max.abs_diff_eq(dvec3(1.4142, 1.7071, 1.7071), EPSILON));
    }

    #[test]
    fn querying_a_shape_bounding_box_in_its_parent_space() {
        let s = Object::new(Shape::Sphere(Sphere::default()))
        .with_scale(0.5, 2.0, 4.0)
        .with_translation(1.0, -3.0, 5.0)
        .transform();

        assert_eq!(s.bounding_box().min, dvec3(0.5, -5.0, 1.0));
        assert_eq!(s.bounding_box().max, dvec3(1.5, -1.0, 9.0));
    }
    
    #[test]
    fn a_group_has_a_bounding_box_that_contains_its_children() {
        let s1 = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(5.0, 5.0, 5.0)
        .transform();

        let s2 = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(-5.0, -5.0, -5.0)
        .transform();
    
        let g = Object::new(Shape::Group(Group::default().with_objects(vec![s1, s2])));

        assert_eq!(g.bounding_box().min, dvec3(-6.0, -6.0, -6.0));
        assert_eq!(g.bounding_box().max, dvec3(6.0, 6.0, 6.0));
    }

    #[test]
    fn a_transformed_group_has_a_bounding_box_that_contains_its_children() {
        let s1 = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(5.0, 5.0, 5.0)
        .transform();

        let s2 = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(-5.0, -5.0, -5.0)
        .transform();
    
        let g = Object::new(Shape::Group(Group::default().with_objects(vec![s1, s2])))
        .with_translation(1.0, 1.0, 1.0)
        .transform();

        assert_eq!(g.bounding_box().min, dvec3(-5.0, -5.0, -5.0));
        assert_eq!(g.bounding_box().max, dvec3(7.0, 7.0, 7.0));
    }

    #[test]
    fn intersecting_a_ray_with_a_bounding_box_at_the_origin() {
        let bb = BoundingBox::new(
            dvec3(-1.0, -1.0, -1.0),
            dvec3(1.0, 1.0, 1.0)
        );

        let datas = vec![
            (dvec3(5.0, 0.5, 0.0), dvec3(-1.0, 0.0, 0.0), true),
            (dvec3(-5.0, 0.5, 0.0), dvec3(1.0, 0.0, 0.0), true),
            (dvec3(0.5, 5.0, 0.0), dvec3(0.0, -1.0, 0.0), true),
            (dvec3(0.5, -5.0, 0.0), dvec3(0.0, 1.0, 0.0), true),
            (dvec3(0.5, 0.0, 5.0), dvec3(0.0, 0.0, -1.0), true),
            (dvec3(0.5, 0.0, -5.0), dvec3(0.0, 0.0, 1.0), true),
            (dvec3(0.0, 0.5, 0.0), dvec3(0.0, 0.0, 1.0), true),
            (dvec3(-2.0, 0.0, 0.0), dvec3(2.0, 4.0, 6.0), false),
            (dvec3(0.0, -2.0, 0.0), dvec3(6.0, 2.0, 4.0), false),
            (dvec3(0.0, 0.0, -2.0), dvec3(4.0, 6.0, 2.0), false),
            (dvec3(2.0, 0.0, 2.0), dvec3(0.0, 0.0, -1.0), false),
            (dvec3(0.0, 2.0, 2.0), dvec3(0.0, -1.0, 0.0), false),
            (dvec3(2.0, 2.0, 0.0), dvec3(-1.0, 0.0, 0.0), false),
        ];
        
        for data in datas {
            let r = Ray::new(data.0, data.1.normalize());
            assert_eq!(bb.intersects(&r), data.2);
        }
    }

    #[test]
    fn intersecting_a_ray_with_a_non_cubic_bounding_box() {
        let bb = BoundingBox::new(
            dvec3(5.0, -2.0, 0.0),
            dvec3(11.0, 4.0, 7.0)
        );

        let datas = vec![
            (dvec3(15.0, 1.0, 2.0), dvec3(-1.0, 0.0, 0.0), true),
            (dvec3(-5.0, -1.0, 4.0), dvec3(1.0, 0.0, 0.0), true),
            (dvec3(7.0, 6.0, 5.0), dvec3(0.0, -1.0, 0.0), true),
            (dvec3(9.0, -5.0, 6.0), dvec3(0.0, 1.0, 0.0), true),
            (dvec3(8.0, 2.0, 12.0), dvec3(0.0, 0.0, -1.0), true),
            (dvec3(6.0, 0.0, -5.0), dvec3(0.0, 0.0, 1.0), true),
            (dvec3(8.0, 1.0, 3.5), dvec3(0.0, 0.0, 1.0), true),
            (dvec3(9.0, -1.0, -8.0), dvec3(2.0, 4.0, 6.0), false),
            (dvec3(8.0, 3.0, -4.0), dvec3(6.0, 2.0, 4.0), false),
            (dvec3(9.0, -1.0, -2.0), dvec3(4.0, 6.0, 2.0), false),
            (dvec3(4.0, 0.0, 9.0), dvec3(0.0, 0.0, -1.0), false),
            (dvec3(8.0, 6.0, -1.0), dvec3(0.0, -1.0, 0.0), false),
            (dvec3(12.0, 5.0, 4.0), dvec3(-1.0, 0.0, 0.0), false),
        ];
        
        for data in datas {
            let r = Ray::new(data.0, data.1.normalize());
            assert_eq!(bb.intersects(&r), data.2);
        }
    }

    #[test]
    fn intersecting_ray_on_group_doenst_test_children_if_boxed_is_missed() {
        let child = Object::new(Shape::TestShape(TestShape::default()));
        let object = Object::new(Shape::Group(Group::default().with_objects(vec![child])));
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 1.0, 0.0)  
        );
        object.intersect(&r);
        assert!(
            match object
            .shape()
            .as_group()
            .unwrap()
            .objects()[0]
            .shape() {
                Shape::TestShape(s) => {
                    s.saved_ray().is_none()
                }
                _ => false
            }
        )
    }

    #[test]
    fn intersecting_ray_on_group_tests_children_if_boxed_is_hit() {
        let child = Object::new(Shape::TestShape(TestShape::default()));
        let object = Object::new(Shape::Group(Group::default().with_objects(vec![child])));
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 0.0, 1.0)  
        );
        object.intersect(&r);
        assert!(
            match object
            .shape()
            .as_group()
            .unwrap()
            .objects()[0]
            .shape() {
                Shape::TestShape(s) => {
                    s.saved_ray().is_some()
                }
                _ => false
            }
        )
    }
}