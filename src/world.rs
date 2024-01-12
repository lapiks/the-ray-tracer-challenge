use glam::DVec3;

use crate::{object::Object, ray::Ray, Color, intersection::{Intersections, IntersectionInfos, ShadowHit, StandardHit}, lights::{light::LightSource, Light}};

pub struct World {
    objects: Vec<Object>,
    lights: Vec<Light>,
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

    pub fn with_lights(mut self, lights: Vec<Light>) -> Self {
        self.lights = lights;
        self
    }

    pub fn push_object(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn object(&self, index: usize) -> Option<&Object> {
        self.objects.get(index)
    }

    pub fn object_mut(&mut self, index: usize) -> Option<&mut Object> {
        self.objects.get_mut(index)
    }

    pub fn light(&self, index: usize) -> Option<&Light> {
        self.lights.get(index)
    }

    pub fn color_at(&self, ray: &Ray, remaining: u8) -> Option<Color> {
        let intersections = self.intersects(ray);
        match intersections.hit_index(StandardHit {}) {
            Some(index) => {
                let infos = IntersectionInfos::new(&intersections, index, &ray);
                Some(self.shade_hit(&infos, remaining))
            },
            None => None
        }
    }

    fn intersects(&self, ray: &Ray) -> Intersections {
        let mut intersections = Intersections::new();
        for object in &self.objects {
            intersections
                .append(
                    object.intersect(ray)
                );
        }

        intersections.sort()
    }

    fn shade_hit(&self, infos: &IntersectionInfos, remaining: u8) -> Color {
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
                    light.intensity_at(infos.over_point, self)
                );
        }

        let reflected = self.reflected_color(infos, remaining);
        let refracted = self.refracted_color(infos, remaining);

        let material = infos.object.material();
        if material.reflective() > 0.0 && material.transparency() > 0.0 {
            let reflectance = infos.schlick();
            color + reflected * reflectance + refracted * (1.0 - reflectance)
        } else {
            color + reflected + refracted
        }
    }

    pub fn is_shadowed(&self, world_point: DVec3, light_pos: DVec3) -> bool {
        let ray_dir = light_pos - world_point;
        let distance = ray_dir.length();
        let shadow_ray = Ray {
            origin: world_point,
            direction: ray_dir.normalize()
        };
        if let Some(hit) = self.intersects(&shadow_ray).hit(ShadowHit {}) {
            return hit.t() < distance;
        }
        false
    }

    fn reflected_color(&self, infos: &IntersectionInfos, remaining: u8) -> Color {
        let reflective = infos.object.material().reflective();
        if remaining < 1 || reflective == 0.0 {
            return Color::black();
        }

        self.color_at(
            &Ray::new(
                infos.over_point, 
                infos.reflectv
            ),
            remaining - 1
        ).unwrap_or_default() * reflective
    }

    fn refracted_color(&self, infos: &IntersectionInfos, remaining: u8) -> Color {
        let transparency = infos.object.material().transparency();
        if remaining < 1 || transparency == 0.0 {
            return Color::black();
        }

        // total internal reflection
        // Snell's Law
        let ratio = infos.n.0 / infos.n.1;
        let cos_i = infos.eyev.dot(infos.normalv);
        let sin2_t = ratio * ratio * (1.0 - cos_i * cos_i);
        if sin2_t > 1.0 {
            // total internal reflection case, light is fully reflected
            return Color::black();
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let refracted_ray = Ray::new(
            infos.under_point,
            infos.normalv * (ratio * cos_i - cos_t) - infos.eyev * ratio
        );

        self.color_at(&refracted_ray, remaining - 1).unwrap_or_default() * transparency
    }
}

#[cfg(test)]
pub mod tests {
    use glam::{DVec3, dvec3};

    use crate::{shapes::{Sphere, Shape, Plane}, Material, intersection::{Intersection, IntersectionInfos}, Pattern, pattern::{PlainPattern, PatternObject, TestPattern}, lights::PointLight};

    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::default();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    pub fn default_world() -> World {
        let l = Light::PointLight(PointLight::new(
            DVec3::new(-10.0, 10.0, -10.0), 
            Color::white()
        ));
        let m = Material::new()
            .with_pattern(
                PatternObject::new(
                    Pattern::Plain(
                        PlainPattern::new(Color::new(0.8, 1.0, 0.6))
                    )
                )
            )
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = Object::new(Shape::Sphere(Sphere::default()))
            .with_material(m);
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
            .with_scale(0.5, 0.5, 0.5)
            .transform();
        World::default()
            .with_objects(vec![s1, s2])
            .with_lights(vec![l])
    }

    #[test]
    fn the_default_world() {
        let w = default_world();

        let l = Light::PointLight(PointLight::new(
            DVec3::new(-10.0, 10.0, -10.0), 
            Color::white()
        ));

        let m = Material::new()
            .with_pattern(
                PatternObject::new(
                    Pattern::Plain(
                        PlainPattern::new(Color::new(0.8, 1.0, 0.6))  
                    )
                )
        )
            .with_diffuse(0.7)
            .with_specular(0.2);

        let s1 = Object::new(Shape::Sphere(Sphere::default()))
            .with_material(m);
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
            .with_scale(0.5, 0.5, 0.5)
            .transform();

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
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = World {
            lights: vec![
                Light::PointLight(PointLight::new(
                    dvec3(0.0, 0.25, 0.0), 
                    Color::white()
                ))
            ],
            ..default_world()
        };
        let r = Ray::new(
            dvec3(0.0, 0.0, 0.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let s = w.objects.get(1).unwrap();
        let i = Intersection::new(0.5, s);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let s1 = Object::new(Shape::Sphere(Sphere::default()));
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
            .with_translation(0.0, 0.0, 10.0)
            .transform();

        let w = World::default()
            .with_lights(
                vec![
                    Light::PointLight(PointLight::new(
                        dvec3(0.0, 0.0, -10.0), 
                        Color::white()
                    ))
                ]
            )
            .with_objects(vec![s1, s2.clone()]);

        let r = Ray::new(
            dvec3(0.0, 0.0, 5.0),
            dvec3(0.0, 0.0, 1.0)
        );

        let i = Intersection::new(4.0, &s2);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 1.0, 0.0)
        );
        let c = w.color_at(&r, 1);
        assert_eq!(c, None);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let c = w.color_at(&r, 1).unwrap_or_default();
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
        let c = w.color_at(&r, 1).unwrap_or_default();
        assert_eq!(c, PlainPattern::default().color());
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        assert_eq!(w.is_shadowed(dvec3(0.0, 10.0, 0.0), w.lights[0].positions()[0]), false);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        assert_eq!(w.is_shadowed(dvec3(10.0, -10.0, 10.0), w.lights[0].positions()[0]), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        assert_eq!(w.is_shadowed(dvec3(-20.0, 20.0, -20.0), w.lights[0].positions()[0]), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        assert_eq!(w.is_shadowed(dvec3(-2.0, 2.0, -2.0), w.lights[0].positions()[0]), false);
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let w = default_world();
        let r = Ray::new(
            DVec3::ZERO,
            DVec3::Z
        );
        let s = w.objects[1].clone()
        .with_material(
            Material::default()
                .with_ambient(1.0)   
        );
        let i = Intersection::new(1.0, &s);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert_eq!(w.reflected_color(&comps, 1), Color::black());
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = default_world();
        w.push_object(
            Object::new(Shape::Plane(Plane::default()))
                .with_material(
                    Material::default()
                        .with_reflective(0.5)
                )
                .with_translation(0.0, -1.0, 0.0)
                .transform()
        );
        let o = w.object(2).unwrap();
        let r = Ray::new(
            dvec3(0.0, 0.0, -3.0),
            dvec3(0.0, -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0)
        );
        let i = Intersection::new(2.0_f64.sqrt(), &o);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert_eq!(w.reflected_color(&comps, 5), Color::new(0.19032, 0.2379, 0.14274));
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = default_world();
        w.push_object(
            Object::new(Shape::Plane(Plane::default()))
                .with_material(
                    Material::default()
                        .with_reflective(0.5)
                )
                .with_translation(0.0, -1.0, 0.0)
                .transform()
        );
        let o = w.object(2).unwrap();
        let r = Ray::new(
            dvec3(0.0, 0.0, -3.0),
            dvec3(0.0, -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0)
        );
        let i = Intersection::new(2.0_f64.sqrt(), &o);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert_eq!(w.shade_hit(&comps, 5), Color::new(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let w = World {
            objects: vec![
                Object::new(Shape::Plane(Plane::default()))
                    .with_material(
                        Material::default()
                            .with_reflective(1.0)
                    )
                    .with_translation(0.0, 1.0, 0.0)
                    .transform(),
                Object::new(Shape::Plane(Plane::default()))
                    .with_material(
                        Material::default()
                            .with_reflective(1.0)
                    )
                    .with_translation(0.0, -1.0, 0.0)
                    .transform(),
            ],
            ..default_world()
        };

        let r = Ray::new(
            dvec3(0.0, 0.0, 0.0),
            dvec3(0.0, 1.0, 0.0)
        );

        w.color_at(&r, 5);
        // if we can arrive here, the color_at function is successful
        // without infinite recursion
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = default_world();
        w.push_object(
            Object::new(Shape::Plane(Plane::default()))
                .with_material(
                    Material::default()
                        .with_reflective(0.5)
                )
                .with_translation(0.0, -1.0, 0.0)
                .transform()
        );
        let o = w.object(2).unwrap();
        let r = Ray::new(
            dvec3(0.0, 0.0, -3.0),
            dvec3(0.0, -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0)
        );
        let i = Intersection::new(2.0_f64.sqrt(), &o);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert_eq!(w.reflected_color(&comps, 0), Color::black());
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = default_world();
        let o = w.object(0).unwrap();
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let xs = Intersections::new().with_intersections(
            vec![
                Intersection::new(4.0, &o),
                Intersection::new(6.0, &o)
            ]
        );
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert_eq!(w.refracted_color(&comps, 5), Color::black());
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let mut w = default_world();

        w.object_mut(0).unwrap()
            .material_mut()
            .set_transparency(1.0)
            .set_refractive_index(1.5);

        let o = w.object(0).unwrap();
        
        let r = Ray::new(
            dvec3(0.0, 0.0, -5.0),
            dvec3(0.0, 0.0, 1.0)
        );
        let xs = Intersections::new().with_intersections(
            vec![
                Intersection::new(4.0, &o),
                Intersection::new(6.0, &o)
            ]
        );
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert_eq!(w.refracted_color(&comps, 0), Color::black());
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut w  = default_world();
        
        w.object_mut(0).unwrap()
            .material_mut()
            .set_transparency(1.0)
            .set_refractive_index(1.5);

        let o = w.object(0).unwrap();

        let r = Ray::new(
            dvec3(0.0, 0.0, 2.0_f64.sqrt()/2.0),
            dvec3(0.0, 1.0, 0.0)
        );
        let xs = Intersections::new().with_intersections(
            vec![
                Intersection::new(-2.0_f64.sqrt()/2.0, &o),
                Intersection::new(2.0_f64.sqrt()/2.0, &o)
            ]
        );
        let comps = IntersectionInfos::new(&xs, 1, &r);
        assert_eq!(w.refracted_color(&comps, 5), Color::black());
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut w = default_world();

        w.object_mut(0).unwrap()
            .material_mut()
            .set_ambient(1.0)
            .set_pattern(
                PatternObject::new(Pattern::Test(TestPattern::new()))
            );
        
        w.object_mut(1).unwrap()
            .material_mut()
            .set_transparency(1.0)
            .set_refractive_index(1.5);

        let a = w.object(0).unwrap();
        let b = w.object(1).unwrap();

        let r = Ray::new(
            dvec3(0.0, 0.0, 0.1),
            dvec3(0.0, 1.0, 0.0)
        );
        let xs = Intersections::new().with_intersections(
            vec![
                Intersection::new(-0.9899, &a),
                Intersection::new(-0.4899, &b),
                Intersection::new(0.4899, &b),
                Intersection::new(0.9899, &a)
            ]
        );
        let comps = IntersectionInfos::new(&xs, 2, &r);
        assert_eq!(w.refracted_color(&comps, 5), Color::new(0.0, 0.99888, 0.04725));
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = default_world();
        w.push_object(
            Object::new(Shape::Plane(Plane::default()))
                .with_translation(0.0, -1.0, 0.0)
                .transform()
                .with_material(
                    Material::default()
                        .with_transparency(0.5)
                        .with_refractive_index(1.5)
                )
        );
        w.push_object(
            Object::new(Shape::Sphere(Sphere::default()))
                .with_translation(0.0, -3.5, -0.5)
                .transform()
                .with_material(
                    Material::default()
                        .with_pattern(
                            PatternObject::new(
                                Pattern::Plain(PlainPattern::new(Color::red()))
                            )
                        )
                        .with_ambient(0.5)
                )
        );
        let floor = w.object(2).unwrap();
        let r = Ray::new(
            dvec3(0.0, 0.0, -3.0),
            dvec3(0.0, -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0)
        );
        let i = Intersection::new(2.0_f64.sqrt(), floor);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert_eq!(w.shade_hit(&comps, 5), Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = default_world();
        w.push_object(
            Object::new(Shape::Plane(Plane::default()))
                .with_translation(0.0, -1.0, 0.0)
                .transform()
                .with_material(
                    Material::default()
                        .with_reflective(0.5)
                        .with_transparency(0.5)
                        .with_refractive_index(1.5)
                )
        );
        w.push_object(
            Object::new(Shape::Sphere(Sphere::default()))
                .with_translation(0.0, -3.5, -0.5)
                .transform()
                .with_material(
                    Material::default()
                        .with_pattern(
                            PatternObject::new(
                                Pattern::Plain(PlainPattern::new(Color::red()))
                            )
                        )
                        .with_ambient(0.5)
                )
        );
        let floor = w.object(2).unwrap();
        let r = Ray::new(
            dvec3(0.0, 0.0, -3.0),
            dvec3(0.0, -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0)
        );
        let i = Intersection::new(2.0_f64.sqrt(), floor);
        let xs = Intersections::new().with_intersections(vec![i.clone()]);
        let comps = IntersectionInfos::new(&xs, 0, &r);
        assert_eq!(w.shade_hit(&comps, 5), Color::new(0.93391, 0.69643, 0.69243));
    }
}