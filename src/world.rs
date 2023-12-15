use glam::DVec3;

use crate::{object::Object, ray::Ray, Color, PointLight, intersection::{Intersections, IntersectionInfos}};

pub struct World {
    objects: Vec<Object>,
    lights: Vec<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        Self { 
            objects: Vec::default(), 
            lights: Vec::default() 
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_objects(mut self, objects: Vec<Object>) -> Self {
        self.objects = objects;
        self
    }

    pub fn with_lights(mut self, lights: Vec<PointLight>) -> Self {
        self.lights = lights;
        self
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        if let Some(intersection) = self
            .intersects(ray)
            .hit() {
                let infos = IntersectionInfos::new(&intersection, &ray);
                return self.shade_hit(&infos);
            }

        Color::black()
    }

    fn intersects(&self, ray: &Ray) -> Intersections{
        let mut intersections = Intersections::new_empty();
        for object in &self.objects {
            intersections
                .append(
                    object.intersect(ray)
                );
                
        }

        intersections.sort()
    }

    fn shade_hit(&self, infos: &IntersectionInfos) -> Color {
        let mut color = Color::black();
        for light in &self.lights {
            color += infos.object
                .material()
                .lighting(
                    infos.object,
                    &light, 
                    infos.over_point, 
                    infos.eyev, 
                    infos.normalv,
                    self.is_shadowed(infos.over_point, light.position())
                );
        }

        color
    }

    fn is_shadowed(&self, world_point: DVec3, light_pos: DVec3) -> bool {
        let ray_dir = light_pos - world_point;
        let distance = ray_dir.length();
        let shadow_ray = Ray {
            origin: world_point,
            direction: ray_dir.normalize()
        };
        if let Some(hit) = self.intersects(&shadow_ray).hit() {
            return hit.t() < distance;
        }
        false
    }
}

#[cfg(test)]
pub mod tests {
    use glam::{DVec3, dvec3};

    use crate::{shapes::{Sphere, Shape}, Material, intersection::{Intersection, IntersectionInfos}, Pattern, pattern::{PlainPattern, PatternObject}};

    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::default();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    pub fn default_world() -> World {
        let l = PointLight::new(
            DVec3::new(-10.0, 10.0, -10.0), 
            Color::white()
        );
        let m = Material::new()
            .with_pattern(
                PatternObject::new(
                    Pattern::PlainPattern(
                        PlainPattern::new(Color::new(0.8, 1.0, 0.6))
                    )
                )
            )
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = Object::new(Shape::Sphere(Sphere::default()))
            .with_material(m);
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
            .with_scale(0.5, 0.5, 0.5);
        World::default()
            .with_objects(vec![s1, s2])
            .with_lights(vec![l])
    }

    #[test]
    fn the_default_world() {
        let w = default_world();

        let l = PointLight::new(
            DVec3::new(-10.0, 10.0, -10.0), 
            Color::white()
        );
        
        let m = Material::new()
            .with_pattern(
                PatternObject::new(
                    Pattern::PlainPattern(
                        PlainPattern::new(Color::new(0.8, 1.0, 0.6))  
                    )
                )
        )
            .with_diffuse(0.7)
            .with_specular(0.2);

        let s1 = Object::new(Shape::Sphere(Sphere::default()))
            .with_material(m);
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
            .with_scale(0.5, 0.5, 0.5);

        assert_eq!(w.objects.len(), 2);
        assert_eq!(w.lights.len(), 1);
        assert_eq!(w.lights[0], l);
        assert_eq!(w.objects[0], s1);
        assert_eq!(w.objects[1], s2);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let xs = w.intersects(&r);
        assert_eq!(xs.count(), 4);
        assert_eq!(xs.get(0).unwrap().t(), 4.0);
        assert_eq!(xs.get(1).unwrap().t(), 4.5);
        assert_eq!(xs.get(2).unwrap().t(), 5.5);
        assert_eq!(xs.get(3).unwrap().t(), 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let s = w.objects.get(0).unwrap();
        let i = Intersection::new(4.0, s);
        let comps = IntersectionInfos::new(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = World {
            lights: vec![
                PointLight::new(
                    dvec3(0.0, 0.25, 0.0), 
                    Color::white()
                )
            ],
            ..default_world()
        };
        let r = Ray::new(
            dvec3(0.0, 0.0, 0.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let s = w.objects.get(1).unwrap();
        let i = Intersection::new(0.5, s);
        let comps = IntersectionInfos::new(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let s1 = Object::new(Shape::Sphere(Sphere::default()));
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
            .with_translation(0.0, 0.0, 10.0);

        let w = World::default()
            .with_lights(
                vec![
                    PointLight::new(
                        dvec3(0.0, 0.0, -10.0), 
                        Color::white()
                    )
                ]
            )
            .with_objects(vec![s1, s2.clone()]);

        let r = Ray::new(
            dvec3(0.0, 0.0, 5.0),
            dvec3(0.0, 0.0, 1.0)
        );

        let i = Intersection::new(4.0, &s2);
        let comps = IntersectionInfos::new(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 1.0, 0.0)
        );
        let c = w.color_at(&r);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let outer = Object::new(Shape::Sphere(Sphere::default()))
            .with_material(
                Material::new()
                    .with_ambient(1.0)
            );
        let inner = Object::new(Shape::Sphere(Sphere::default()))
            .with_material(
                Material::new()
                    .with_ambient(1.0)
            );
        let w = World {
            objects: vec![inner.clone(), outer],
            ..default_world()  
        };
        let r = Ray::new(
            dvec3(0.0, 0.0, 0.75),
            dvec3(0.0, 0.0, -1.0)
        );
        let c = w.color_at(&r);
        assert_eq!(c, PlainPattern::default().color());
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        assert_eq!(w.is_shadowed(dvec3(0.0, 10.0, 0.0), w.lights[0].position()), false);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        assert_eq!(w.is_shadowed(dvec3(10.0, -10.0, 10.0), w.lights[0].position()), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        assert_eq!(w.is_shadowed(dvec3(-20.0, 20.0, -20.0), w.lights[0].position()), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        assert_eq!(w.is_shadowed(dvec3(-2.0, 2.0, -2.0), w.lights[0].position()), false);
    }
}