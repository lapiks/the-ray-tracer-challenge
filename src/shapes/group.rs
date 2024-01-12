use glam::DVec3;

use crate::{ray::Ray, Object, intersection::Intersections};
use super::shape::Hittable;

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    objects: Vec<Object>,
}

impl Hittable for Group {
    fn intersect<'a>(&'a self, ray: &Ray, _: &'a Object) -> Intersections<'a> {
        let mut xs = Intersections::new();
        for object in &self.objects {
            xs.append(object.intersect(ray));
        }
        xs 
    }

    fn normal_at(&self, _: DVec3) -> DVec3 {
        DVec3::default()
    }
}

impl Default for Group {
    fn default() -> Self {
        Self {
            objects: Vec::default()
        }
    }
}

impl Group {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_objects(mut self, objects: Vec<Object>) -> Self {
        self.objects = objects;
        self
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use glam::{DMat4, dvec3};
    use crate::shapes::{Shape, Sphere};

    use super::*;

    #[test]
    fn creating_a_new_group() {
        let g = Group::default();
        let o = Object::new(Shape::Group(g.clone()));
        assert_eq!(o.transform().matrix, DMat4::IDENTITY);
        assert_eq!(g.objects.len(), 0);
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let s = Object::new(Shape::Sphere(Sphere::default()));
        let g = Group::new()
        .with_objects(vec![s.clone()]);
        assert_eq!(g.objects.len(), 1);
        assert_eq!(g.objects[0], s);
    }
    
    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = Object::new(Shape::Group(Group::default()));
        let r = Ray::new(
            dvec3(0.0, 0.0, 0.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let xs = g.intersect(&r);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn intersecting_a_ray_with_a_non_empty_group() {
        let s1 = Object::new(Shape::Sphere(Sphere::default()));
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
            .with_translation(0.0, 0.0, -3.0)
            .transform();
        let s3 = Object::new(Shape::Sphere(Sphere::default()))
            .with_translation(5.0, 0.0, 0.0)
            .transform();
        let g = Object::new(
            Shape::Group(
                Group::new()
                .with_objects(vec![s1.clone(), s2.clone(), s3.clone()])
            )
        );

        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let xs = g.intersect(&r).sort();
        assert_eq!(xs.count(), 4);
        assert_eq!(*xs[0].object(), s2);
        assert_eq!(*xs[1].object(), s2);
        assert_eq!(*xs[2].object(), s1);
        assert_eq!(*xs[3].object(), s1);
    }

    #[test]
    fn intersecting_a_transformed_roup() {
        let s = Object::new(Shape::Sphere(Sphere::default()))
            .with_translation(5.0, 0.0, 0.0)
            .transform();
        let g = Object::new(
            Shape::Group(
                Group::new()
                .with_objects(vec![s])
            )
        )
        .with_scale(2.0, 2.0, 2.0)
        .transform();

        let r = Ray::new(
            dvec3(10.0, 0.0, -10.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let xs = g.intersect(&r).sort();
        assert_eq!(xs.count(), 2);
    }

    #[test]
    fn convert_a_point_from_world_to_object_space() {
        let s = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(5.0, 0.0, 0.0)
        .transform();

        let g2 = Object::new(
            Shape::Group(
                Group::new()
                .with_objects(vec![s])
            )
        )
        .with_scale(2.0, 2.0, 2.0)
        .transform();

        let g1 = Object::new(
            Shape::Group(
                Group::new()
                .with_objects(vec![g2])
            )
        )
        .with_rotation_y(PI / 2.0)
        .transform();

        //assert_eq!(s.world_to_object(dvec3(-2.0, 0.0, -10.0)), dvec3(0.0, 0.0, -1.0));
    }
}