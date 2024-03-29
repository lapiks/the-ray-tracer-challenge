use std::cmp::Ordering;

use glam::DVec3;

use crate::{object::Object, ray::Ray};

const EPSILON: f64 = 0.00001;

pub trait HitPredicate {
    fn hit_predicate(&self) -> Box<dyn FnMut(&&Intersection<'_>) -> bool>;
    fn hit_index_predicate(&self) -> Box<dyn FnMut(&Intersection<'_>) -> bool>;
}

pub struct StandardHit {}

impl HitPredicate for StandardHit {
    fn hit_predicate(&self) -> Box<dyn FnMut(&&Intersection<'_>) -> bool> {
       Box::new(|i| i.t >= 0.0)
    }

    fn hit_index_predicate(&self) -> Box<dyn FnMut(&Intersection<'_>) -> bool> {
        Box::new(|i| i.t >= 0.0)
    }
}

pub struct ShadowHit {}

impl HitPredicate for ShadowHit {
    fn hit_predicate(&self) -> Box<dyn FnMut(&&Intersection<'_>) -> bool> {
        Box::new(|i| i.object.shadow() && i.t >= 0.0)
    }

    fn hit_index_predicate(&self) -> Box<dyn FnMut(&Intersection<'_>) -> bool> {
        Box::new(|i| i.object.shadow() && i.t >= 0.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Object,
    u: f64,
    v: f64,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Object) -> Self {
        Self {
            t,
            object,
            u: 0.0, 
            v: 0.0,
        }
    }

    pub fn with_u_v(mut self, u: f64, v: f64) -> Self {
        self.u = u;
        self.v = v;
        self
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Object {
        &self.object
    }

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn v(&self) -> f64 {
        self.v
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
    pub fn new() -> Self {
        Self {  
            intersections: Vec::default()
        }
    }

    pub fn from_capacity(size: usize) -> Self {
        Self {
            intersections: Vec::with_capacity(size)
        }
    }

    pub fn with_intersections(mut self, intersections: Vec<Intersection<'a>>) -> Self {
        self.intersections = intersections;
        self
    }

    pub fn push(&mut self, intersection: Intersection<'a>) {
        self.intersections.push(intersection);
    }

    pub fn append(&mut self, other: Intersections<'a>) {
        self.intersections.append(&mut other.move_all());
    }

    pub fn sort(mut self) -> Self {
        self.intersections.sort();
        self
    }

    pub fn hit(&self, predicate: impl HitPredicate) -> Option<&Intersection> {
        self.intersections.iter().find(predicate.hit_predicate())
    }

    pub fn hit_index(&self, predicate: impl HitPredicate) -> Option<usize> {
        self.intersections.iter().position(predicate.hit_index_predicate())
    }

    pub fn count(&self) -> usize {
        self.intersections.len()
    }

    pub fn get(&self, index: usize) -> Option<&Intersection> {
        self.intersections.get(index)
    }

    pub fn move_all(self) -> Vec<Intersection<'a>> {
        self.intersections
    }

    pub fn get_all(&self) -> &Vec<Intersection<'a>> {
        &self.intersections
    }
}

impl<'a> std::ops::Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    // row major
    fn index(&self, index: usize) -> &Intersection<'a> {
        &self.intersections[index]
    }
}

pub struct IntersectionInfos<'a> {
    pub t: f64,
    pub object: &'a Object,
    pub point: DVec3,
    pub over_point: DVec3,
    pub under_point: DVec3,
    pub eyev: DVec3,
    pub normalv: DVec3,
    pub reflectv: DVec3,
    pub inside: bool,
    pub n: (f64, f64)
}

impl<'a> IntersectionInfos<'a> {
    pub fn new(intersections: &'a Intersections<'a>, intersection_index: usize, ray: &Ray) -> Self {
        let intersection = intersections.get(intersection_index).unwrap();

        let point = ray.at(intersection.t);
        let object = intersection.object;
        let eyev = -ray.direction;
        let mut normalv = object.normal_at(point, intersection.u, intersection.v);
        let mut inside = false;
        if normalv.dot(eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }
        let over_point = point + normalv * EPSILON;
        let under_point = point - normalv * EPSILON;
        let reflectv = ray.direction - normalv * 2.0 * ray.direction.dot(normalv);

        // find n1 n2
        let (mut n1, mut n2) = (0.0, 0.0);
        let mut containers: Vec<&Object> = Vec::default();
        for (current_index, i) in intersections.get_all().iter().enumerate() {
            if current_index == intersection_index {
                if containers.is_empty() {
                    n1 = 1.0;
                } else {
                    n1 = containers.last().unwrap().material().refractive_index();
                }
            }

            match containers
                .iter()
                .position(|&object| object == i.object) {
                    Some(pos) => {
                        containers.remove(pos);
                    }
                    None => containers.push(i.object),
            }

            if current_index == intersection_index {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().material().refractive_index();
                }
            }
        }

        Self {
            t: intersection.t,
            object,
            point,
            over_point,
            under_point,
            eyev,
            normalv,
            reflectv,
            inside,
            n: (n1, n2)
        }
    }

    pub fn schlick(&self) -> f64 {
        let mut cos = self.eyev.dot(self.normalv);
        if self.n.0 > self.n.1 {
            let n = self.n.0 / self.n.1;
            let sin2_t = n * n * (1.0 - cos * cos);
            if sin2_t > 1.0 {
                return 1.0;
            }

            cos = (1.0 - sin2_t).sqrt(); 
        }

        let r0 = ((self.n.0 - self.n.1) / (self.n.0 + self.n.1)).powi(2);

        return r0 + (1.0 - r0) * (1.0 - cos).powi(5);
    }
}

#[cfg(test)]
mod tests {
    use glam::dvec3;

    use crate::{shapes::{sphere::Sphere, shape::Shape, Plane}, ray::Ray, Material};

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
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2.clone()]);
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
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2.clone()]).sort();
        let i = xs.hit(StandardHit {});
        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let o = Object::new(Shape::Sphere(s));
        let i1 = Intersection::new(-1.0, &o);
        let i2 = Intersection::new(1.0, &o);
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2.clone()]).sort();
        let i = xs.hit(StandardHit {});
        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let o = Object::new(Shape::Sphere(s));
        let i1 = Intersection::new(-2.0, &o);
        let i2 = Intersection::new(-1.0, &o);
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2.clone()]).sort();
        let i = xs.hit(StandardHit {});
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
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()]).sort();
        let i = xs.hit(StandardHit {});
        assert_eq!(i, Some(&i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let o = Object::new(Shape::Sphere(Sphere::default()));
        let i = Intersection::new(4.0,&o);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, dvec3(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, dvec3(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, dvec3(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outsize() {
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let o = Object::new(Shape::Sphere(Sphere::default()));
        let i = Intersection::new(4.0,&o);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(
            dvec3(0.0, 0.0, 0.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let o = Object::new(Shape::Sphere(Sphere::default()));
        let i = Intersection::new(1.0,&o);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, dvec3(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, dvec3(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, dvec3(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let o = Object::new(Shape::Sphere(Sphere::default()))
            .with_translation(0.0, 0.0, 1.0)
            .transform();
        let i = Intersection::new(5.0,&o);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert!(comps.over_point.z < -EPSILON/2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    
    #[test]
    fn precomputing_the_reflection_vector() {
        let o = Object::new(Shape::Plane(Plane::new()));
        let r = Ray::new(
            dvec3(0.0, 1.0, -1.0), 
            dvec3(0.0, -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0)
        );
        let i = Intersection::new(2.0_f64.sqrt(), &o);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert_eq!(comps.reflectv, dvec3(0.0, 2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0));
    }

    fn glass_sphere() -> Object {
        Object::new(Shape::Sphere(Sphere::default()))
            .with_material(
                Material::default()
                .with_transparency(1.0)
                .with_refractive_index(1.5)
            )
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = glass_sphere()
            .with_scale(2.0, 2.0, 2.0)
            .transform();
        let b = glass_sphere()
            .with_translation(0.0, 0.0, -0.25)
            .transform()
            .with_material(
                a.material().clone()
                    .with_refractive_index(2.0)
            );
        let c = glass_sphere()
            .with_translation(0.0, 0.0, 0.25)
            .transform()
            .with_material(
                a.material().clone()
                    .with_refractive_index(2.5)
            );
        let r = Ray::new(
            dvec3(0.0, 0.0, -4.0),
            dvec3(0.0, 0.0, 1.0)
        );

        let xs = Intersections::new()
            .with_intersections(
                vec![
                    Intersection::new(2.0, &a),
                    Intersection::new(2.75, &b),
                    Intersection::new(3.25, &c),
                    Intersection::new(4.75, &b),
                    Intersection::new(5.25, &c),
                    Intersection::new(6.0, &a),
                ]
            );

        assert_eq!(IntersectionInfos::new(&xs, 0, &r).n, (1.0, 1.5));
        assert_eq!(IntersectionInfos::new(&xs, 1, &r).n, (1.5, 2.0));
        assert_eq!(IntersectionInfos::new(&xs, 2, &r).n, (2.0, 2.5));
        assert_eq!(IntersectionInfos::new(&xs, 3, &r).n, (2.5, 2.5));
        assert_eq!(IntersectionInfos::new(&xs, 4, &r).n, (2.5, 1.5));
        assert_eq!(IntersectionInfos::new(&xs, 5, &r).n, (1.5, 1.0));
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0), 
            dvec3(0.0, 0.0, 1.0)
        );
        let s = glass_sphere()
            .with_translation(0.0, 0.0, 1.0)
            .transform();
        let xs = Intersections::new().with_intersections(vec![Intersection::new(5.0, &s)]);
        let infos = IntersectionInfos::new(&xs, 0, &r);
        assert!(infos.under_point.z > EPSILON / 2.0);
        assert!(infos.point.z < infos.under_point.z);
    }

    #[test]
    fn the_schlick_approximation_user_total_internal_reflection() {
        let s = glass_sphere();

        let r = Ray::new(
            dvec3(0.0, 0.0, 2.0_f64.sqrt()/2.0), 
            dvec3(0.0, 1.0, 0.0)
        );
        
        let xs = Intersections::new().with_intersections(
            vec![
                Intersection::new(-2.0_f64.sqrt()/2.0, &s),
                Intersection::new(2.0_f64.sqrt()/2.0, &s)
            ]
        );
        let infos = IntersectionInfos::new(&xs, 1, &r);

        assert_eq!(infos.schlick(), 1.0);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let s = glass_sphere();

        let r = Ray::new(
            dvec3(0.0, 0.0, 0.0), 
            dvec3(0.0, 1.0, 0.0)
        );
        
        let xs = Intersections::new().with_intersections(
            vec![
                Intersection::new(-1.0, &s),
                Intersection::new(1.0, &s)
            ]
        );
        let infos = IntersectionInfos::new(&xs, 1, &r);

        assert!((infos.schlick() - 0.04).abs() < EPSILON);
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let s = glass_sphere();

        let r = Ray::new(
            dvec3(0.0, 0.99, -2.0), 
            dvec3(0.0, 0.0, 1.0)
        );
        
        let xs = Intersections::new().with_intersections(
            vec![
                Intersection::new(1.8589, &s)
            ]
        );
        let infos = IntersectionInfos::new(&xs, 0, &r);

        assert!((infos.schlick() - 0.48873).abs() < EPSILON);
    }
}