use glam::DVec3;

use crate::{ray::Ray, Object, intersection::Intersections, bounds::BoundingBox};
use super::{shape::Hittable, Shape};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Group {
    objects: Vec<Object>,
}

impl Hittable for Group {
    fn intersect<'a>(&'a self, ray: &Ray, this: &'a Object) -> Intersections<'a> {
        let mut xs = Intersections::new();
        if this.bounding_box().intersects(ray) {
            for object in &self.objects {
                xs.append(object.intersect(ray));
            }
        }
        xs
    }

    fn normal_at(&self, _: DVec3, _: f64, _: f64) -> DVec3 {
        DVec3::default()
    }

    fn bounds(&self) -> BoundingBox {
        self.objects
            .iter()
            .fold(BoundingBox::default(), |bounds, object| {
                bounds.merge(object.bounding_box())
            })
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

    pub fn objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut Vec<Object> {
        &mut self.objects
    }

    fn partition_children(&mut self) -> (Group, Group) {
        let (left, right) = self.bounds().split();

        let mut left_objects = Vec::default();
        let mut right_objects = Vec::default();
        let mut middle_objects = Vec::default();

        for object in &self.objects {
            if left.contains_box(object.bounding_box()) {
                left_objects.push(object.clone());
            } else if right.contains_box(object.bounding_box()) {
                right_objects.push(object.clone())
            } else {
                middle_objects.push(object.clone());
            }
        }

        self.objects = middle_objects;

        (
            Group::new().with_objects(left_objects),
            Group::new().with_objects(right_objects)
        )
    }

    fn make_subgroup(&mut self, objects: Vec<Object>) {
        self.objects.push(
            Object::new(
                Shape::Group(
                    Group::default()
                    .with_objects(objects)
                )
            )
        )
    }

    pub fn divide(&mut self, threshold: usize) {
        if threshold <= self.objects.len() {
            let (left, right) = self.partition_children();
            if !left.objects.is_empty() {
                self.make_subgroup(left.objects);
            }
            if !right.objects.is_empty() {
                self.make_subgroup(right.objects);
            }
        }
        for child in &mut self.objects {
            *child = child.clone().divide(threshold); //todo: improve
        }
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
    fn intersecting_a_transformed_group() {
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
    fn partitionning_a_group_s_children() {
        let s1 = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(-2.0, 0.0, 0.0)
        .transform();
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(2.0, 0.0, 0.0)
        .transform();
        let s3 = Object::new(Shape::Sphere(Sphere::default()));

        let mut g = Group::default().with_objects(vec![s1.clone(), s2.clone(), s3.clone()]);
        let (left, right) = g.partition_children();
        assert_eq!(g.objects[0], s3);
        assert_eq!(left.objects, vec![s1]);
        assert_eq!(right.objects, vec![s2]);
    }

    #[test]
    fn creating_a_sub_group_from_a_list_of_children() {
        let s1 = Object::new(Shape::Sphere(Sphere::default()));
        let s2 = Object::new(Shape::Sphere(Sphere::default()));
        let mut g = Group::default();
        g.make_subgroup(vec![s1.clone(), s2.clone()]);

        assert_eq!(g.objects.len(), 1);
        assert_eq!(*g.objects[0].shape().as_group().unwrap().objects(), vec![s1, s2]);
    }

    #[test]
    fn subdividing_a_primitive_does_nothing() {
        let shape = Object::new(
            Shape::Sphere(
                Sphere::default()
            )
        )
        .divide(1);
        assert_eq!(*shape.shape(), Shape::Sphere(Sphere::default()));
    }


    #[test]
    fn subdividing_a_group_partitions_its_children() {
        let s1 = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(-2.0, -2.0, 0.0)
        .transform();
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(-2.0, 2.0, 0.0)
        .transform();
        let s3 =  Object::new(Shape::Sphere(Sphere::default()))
        .with_scale(4.0, 4.0, 4.0)
        .transform();
        let mut g = Group::default().with_objects(vec![s1.clone(), s2.clone(), s3.clone()]);
        g.divide(1);

        assert_eq!(g.objects[0], s3);
        let subgroup = g.objects[1].shape().as_group();
        assert!(subgroup.is_some());
        assert_eq!(subgroup.unwrap().objects.len(), 2);
        let subsubgroup1 = subgroup.unwrap().objects[0].shape().as_group();
        assert!(subsubgroup1.is_some());
        assert_eq!(subsubgroup1.unwrap().objects, vec![s1]);
        let subsubgroup2 = subgroup.unwrap().objects[1].shape().as_group();
        assert!(subsubgroup2.is_some());
        assert_eq!(subsubgroup2.unwrap().objects, vec![s2]);
    }

    #[test]
    fn subdividing_a_group_with_too_few_children() {
        let s1 = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(-2.0, 0.0, 0.0)
        .transform();
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(2.0, 1.0, 0.0)
        .transform();
        let s3 =  Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(2.0, -1.0, 0.0)
        .transform();
        let subgroup = Object::new(Shape::Group(Group::default().with_objects(vec![s1.clone(), s2.clone(), s3.clone()])));

        let s4 = Object::new(Shape::Sphere(Sphere::default()));
        let mut g = Group::default().with_objects(vec![subgroup.clone(), s4.clone()]);
    
        g.divide(3);

        let subgroup = g.objects[0].shape().as_group();
        assert_eq!(subgroup.is_some(), true);
        assert_eq!(g.objects[1], s4);
        assert_eq!(subgroup.unwrap().objects.len(), 2);
        let subgroup0 = subgroup.unwrap().objects[0].shape().as_group();
        assert_eq!(subgroup0.is_some(), true);
        assert_eq!(subgroup0.unwrap().objects, vec![s1]);
        let subgroup1 = subgroup.unwrap().objects[1].shape().as_group();
        assert_eq!(subgroup1.is_some(), true);
        assert_eq!(subgroup1.unwrap().objects, vec![s2, s3]);
    }
}