use std::f64::consts::PI;

use glam::dvec3;
use ray_tracer::{Camera, World, Object, shapes::{Sphere, Shape, Plane}, Color, Material, transformations, Pattern, pattern::{PlainPattern, PatternObject, StrippedPattern, GradientPattern, CheckerPattern}, lights::{Light, PointLight}};

fn main() {
    let l1 = Light::PointLight(PointLight::new(
        dvec3(-10.0, 10.0, -10.0), 
        Color::white() * 0.6
    ));
    let l2 = Light::PointLight(PointLight::new(
        dvec3(10.0, 10.0, -10.0), 
        Color::white() * 0.4
    ));

    let wall_material = Material::default()
        .with_pattern(
            PatternObject::new(
                Pattern::Plain(
                    PlainPattern::new(Color::white()))
                )
            )
        .with_specular(0.0);
    
    let floor = Object::new(Shape::Plane(Plane::default()))
        .with_material(
            Material::default()
                .with_pattern(
                    PatternObject::new(
                        Pattern::Checker(
                            CheckerPattern::new(Color::white(), Color::black())
                        )
                    )
                    .with_rotation_y(PI / 4.0)
                    .transform()
                )
        );

    let left_wall = Object::new(Shape::Plane(Plane::default()))
        .with_material(wall_material.clone())
        .with_rotation_x(PI/2.0)
        .with_rotation_y(-PI/4.0)
        .with_translation(0.0, 0.0, 5.0)
        .transform();

    let right_wall = Object::new(Shape::Plane(Plane::default()))
        .with_material(wall_material.clone())
        .with_rotation_x(PI/2.0)
        .with_rotation_y(PI/4.0)
        .with_translation(0.0, 0.0, 5.0)
        .transform();

    let middle = Object::new(Shape::Sphere(Sphere::default()))
        .with_material(
            Material::default()
                .with_pattern(
                    PatternObject::new(
                        Pattern::Gradient(
                            GradientPattern::new(Color::new(0.1, 1.0, 0.5), Color::new(0.5, 0.5, 1.0)))
                    )
                    .with_scale(2.0, 2.0, 2.0)
                    .with_translation(-1.0, 0.0, 0.0)
                    .with_rotation_z(PI / 4.0)
                    .transform()
                )
                .with_diffuse(1.0)
                .with_specular(0.3)
        )
        .with_translation(-0.5, 1.0, 0.5)
        .transform();

    let right = Object::new(Shape::Sphere(Sphere::default()))
        .with_material(
            Material::default()
                .with_pattern(
                    PatternObject::new(
                        Pattern::Stripped(StrippedPattern::new(Color::new(1.0, 0.0, 0.0), Color::new(1.0, 1.0, 0.0)))
                    )
                    .with_scale(0.2, 0.2, 0.2)
                    .with_translation(-1.0, 0.0, 0.0)
                    .with_rotation_z(-PI / 4.0)
                    .with_rotation_y(-PI / 4.0)
                    .transform()
                )
                .with_diffuse(1.0)
                .with_specular(0.3)
        )
        .with_scale(0.5, 0.5, 0.5)
        .with_translation(1.5, 0.5, -0.5)
        .transform();

    let left = Object::new(Shape::Sphere(Sphere::default()))
        .with_material(
            Material::default()
                .with_pattern(
                    PatternObject::new(
                        Pattern::Plain(PlainPattern::new(Color::new(1.0, 0.8, 0.1)))
                    )
                )
                .with_diffuse(1.0)
                .with_specular(0.3)
        )
        .with_scale(0.33, 0.33, 0.33)
        .with_translation(-1.5, 0.33, -0.75)
        .transform();

    let world = World::new()
        .with_objects(vec![floor, left_wall, right_wall, middle, right, left])
        .with_lights(vec![l1, l2]);

    let camera = Camera::new(1920, 1080, PI / 3.0)
        .with_transform(
            transformations::view_transform(
                dvec3(0.0, 1.5, -5.0),
                dvec3(0.0, 1.0, 0.0),
                dvec3(0.0, 1.0, 0.0)
            )
        )
        .with_antialiasing(3);

    let canvas = camera.render(&world, 5);
    canvas.export("examples/example03.png").unwrap();
}
