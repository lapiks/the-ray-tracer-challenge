use std::f64::consts::PI;

use glam::dvec3;
use ray_tracer::{Camera, World, Object, shapes::{Sphere, Shape}, Color, Material, transformations, Pattern, pattern::{PlainPattern, PatternObject}, lights::{Light, PointLight}};

fn main() {
    let camera = Camera::new(1920, 1080, PI / 3.0)
        .with_transform(
            transformations::view_transform(
                dvec3(0.0, 1.5, -5.0),
                dvec3(0.0, 0.0, 0.0),
                dvec3(0.0, 1.0, 0.0)
            )
        )
        .with_antialiasing(3);

    let l1 = Light::PointLight(PointLight::new(
        dvec3(-10.0, 10.0, -10.0), 
        Color::white()
    ));

    let l2 = Light::PointLight(PointLight::new(
        dvec3(10.0, 10.0, 10.0), 
        Color::white()
    ));

    let s = Sphere::default();
    let o1 = Object::new(Shape::Sphere(s))
        .with_scale(0.75, 0.75, 0.75)
        .with_translation(1.0, 0.0, 0.0)
        .with_material(
            Material::default()
                .with_pattern(
                    PatternObject::new(
                        Pattern::Plain(PlainPattern::new(Color::new(1.0, 0.2, 1.0)))
                    )
                )
        );

    let o2 = Object::new(Shape::Sphere(s))
        .with_translation(-1.0, 0.0, 0.0)
        .with_material(
            Material::default()
                .with_pattern(
                    PatternObject::new(
                        Pattern::Plain(PlainPattern::new(Color::new(0.2, 1.0, 1.0)))
                    )
                )
        );

    let world = World::new()
        .with_objects(vec![o1, o2])
        .with_lights(vec![l1, l2]);

    let canvas = camera.render(&world, 5);
    canvas.export("examples/example01.png").unwrap();
}
