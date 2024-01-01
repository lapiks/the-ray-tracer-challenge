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

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}

#[cfg(test)]
mod tests {
    use glam::{DMat4, dvec3};
    use crate::shapes::{Shape, Sphere};

    use super::*;

    #[test]
    fn creating_a_new_group() {
        let g = Group::default();
        let o = Object::new(Shape::Group(g.clone()));
        assert_eq!(*o.transform(), DMat4::IDENTITY);
        assert_eq!(g.objects.len(), 0);
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let mut g = Group::default();
        let s = Object::new(Shape::Sphere(Sphere::default()));
        g.add_object(s.clone());
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
        let mut g = Group::default();
        let s1 = Object::new(Shape::Sphere(Sphere::default()));
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
            .with_translation(0.0, 0.0, -3.0);
        let s3 = Object::new(Shape::Sphere(Sphere::default()))
            .with_translation(5.0, 0.0, 0.0);
        g.add_object(s1.clone());
        g.add_object(s2.clone());
        g.add_object(s3.clone());
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let o = Object::new(Shape::Group(g));
        let xs = o.intersect(&r).sort();
        assert_eq!(xs.count(), 4);
        assert_eq!(*xs[0].object(), s2);
        assert_eq!(*xs[1].object(), s2);
        assert_eq!(*xs[2].object(), s1);
        assert_eq!(*xs[3].object(), s1);
    }
}