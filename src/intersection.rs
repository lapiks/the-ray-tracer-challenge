use std::cmp::Ordering;

use crate::object::Object;


#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<'a> {
    t: f32,
    object: &'a Object
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a Object) -> Self {
        Self {
            t,
            object
        }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn object(&self) -> &Object {
        &self.object
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t < other.t {
            return Ordering::Less;
        }
        else if self.t > other.t {
            return Ordering::Greater;
        }
        else {
            return Ordering::Equal;
        }
    }
}

impl<'a> Eq for Intersection<'a> {}

pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new_empty() -> Self {
        Self {
            intersections: Vec::default()
        }
    }

    pub fn new(intersections: Vec<Intersection<'a>>) -> Self {
        Self {  
            intersections: intersections
        }
    }

    pub fn with_capacity(size: usize) -> Self {
        Self {
            intersections: Vec::with_capacity(size)
        }
    }

    pub fn push(&mut self, intersection: &Intersection<'a>) {
        self.intersections.push(intersection.clone());
    }

    pub fn sort(mut self) -> Self {
        self.intersections.sort();
        self
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections.iter().find(|i| i.t >= 0.0)
    }

    pub fn count(&self) -> usize {
        self.intersections.len()
    }

    pub fn get(&self, index: usize) -> Option<&Intersection> {
        self.intersections.get(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::{sphere::Sphere, shape::Shape};

    use super::*;

    // An intersection encapsulate t and object
    #[test]
    fn new_intersection() {
        let s = Sphere::default();
        let o = Object::new(Shape::Sphere(s));
        let i = Intersection::new(3.5, &o);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &o);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();
        let o = Object::new(Shape::Sphere(s));
        let i1 = Intersection::new(1.0, &o);
        let i2 = Intersection::new(2.0, &o);
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0), Some(&i1));
        assert_eq!(xs.get(1), Some(&i2));
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let o = Object::new(Shape::Sphere(s));
        let i1 = Intersection::new(1.0, &o);
        let i2 = Intersection::new(2.0, &o);
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]).sort();
        let i = xs.hit();
        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let o = Object::new(Shape::Sphere(s));
        let i1 = Intersection::new(-1.0, &o);
        let i2 = Intersection::new(1.0, &o);
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]).sort();
        let i = xs.hit();
        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let o = Object::new(Shape::Sphere(s));
        let i1 = Intersection::new(-2.0, &o);
        let i2 = Intersection::new(-1.0, &o);
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]).sort();
        let i = xs.hit();
        assert_eq!(i, None);
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let o = Object::new(Shape::Sphere(s));
        let i1 = Intersection::new(5.0, &o);
        let i2 = Intersection::new(7.0, &o);
        let i3 = Intersection::new(-3.0, &o);
        let i4 = Intersection::new(2.0, &o);
        let xs = Intersections::new(vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()]).sort();
        let i = xs.hit();
        assert_eq!(i, Some(&i4));
    }
}