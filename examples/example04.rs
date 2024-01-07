use std::f64::consts::PI;

use glam::{dvec3, DMat4};
use ray_tracer::{Camera, World, Object, shapes::{Sphere, Shape, Plane}, Color, Material, PointLight, transformations, Pattern, pattern::{PlainPattern, PatternObject, StrippedPattern, CheckerPattern}};

fn main() {
    let l1 = PointLight::new(
        dvec3(-5.0, 5.0, -5.0), 
        Color::white() * 0.6
    );
    let l2 = PointLight::new(
        dvec3(5.0, 5.0, -5.0), 
        Color::white() * 0.4
    );

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
                .with_reflective(0.3)
                .with_pattern(
                    PatternObject::new(
                        Pattern::Checker(
                            CheckerPattern::new(Color::white(), Color::black())
                        )
                    )
                    .with_transform(
                        &DMat4::from_rotation_y(PI / 4.0)
                    )
                )
        );

    let roof = Object::new(Shape::Plane(Plane::default()))
        .with_material(wall_material.clone())
        .with_translation(0.0, 10.0, 0.0);

    let left_wall = Object::new(Shape::Plane(Plane::default()))
        .with_material(wall_material.clone())
        .with_rotation_x(PI/2.0)
        .with_rotation_y(-PI/4.0)
        .with_translation(0.0, 0.0, 5.0);

    let right_wall = Object::new(Shape::Plane(Plane::default()))
        .with_material(wall_material.clone())
        .with_rotation_x(PI/2.0)
        .with_rotation_y(PI/4.0)
        .with_translation(0.0, 0.0, 5.0);

    let back_left_wall = Object::new(Shape::Plane(Plane::default()))
        .with_material(wall_material.clone())
        .with_rotation_x(PI/2.0)
        .with_rotation_y(PI/4.0)
        .with_translation(0.0, 0.0, -15.0);

    let back_right_wall = Object::new(Shape::Plane(Plane::default()))
        .with_material(wall_material.clone())
        .with_rotation_x(PI/2.0)
        .with_rotation_y(-PI/4.0)
        .with_translation(0.0, 0.0, -15.0);

    let middle = Object::new(Shape::Sphere(Sphere::default()))
        .with_material(
            Material::default()
                .with_pattern(
                    PatternObject::new(
                        Pattern::Plain(
                            PlainPattern::new(Color::black()))
                    )
                    .with_transform(
                        &(
                            DMat4::from_rotation_z(PI / 4.0) * 
                            DMat4::from_translation(dvec3(-1.0, 0.0, 0.0)) * 
                            DMat4::from_scale(dvec3(2.0, 2.0, 2.0))
                        )
                    )
                )
                .with_diffuse(0.0)
                .with_specular(0.0)
                .with_ambient(0.0)
                .with_reflective(1.0)
        )
        .with_translation(-0.5, 1.0, 0.5);

    let right = Object::new(Shape::Sphere(Sphere::default()))
        .with_material(
            Material::default()
                .with_pattern(
                    PatternObject::new(
                        Pattern::Stripped(StrippedPattern::new(Color::new(1.0, 0.0, 0.0), Color::new(1.0, 1.0, 0.0)))
                    )
                    .with_transform(
                        &(
                            DMat4::from_rotation_y(-PI / 4.0) * 
                            DMat4::from_rotation_z(-PI / 4.0) * 
                            DMat4::from_translation(dvec3(-1.0, 0.0, 0.0)) * 
                            DMat4::from_scale(dvec3(0.2, 0.2, 0.2))
                        )
                    )
                )
                .with_diffuse(0.5)
                .with_specular(0.5)
                .with_shininess(500.0)
                .with_reflective(0.5)
        )
        .with_scale(0.5, 0.5, 0.5)
        .with_translation(1.5, 0.5, -0.5);

    let left = Object::new(Shape::Sphere(Sphere::default()))
        .with_material(
            Material::default()
                .with_pattern(
                    PatternObject::new(
                        Pattern::Plain(PlainPattern::new(Color::new(1.0, 1.0, 0.0)))
                    )
                )
                .with_diffuse(0.2)
                .with_specular(0.0)
                .with_reflective(0.8)
        )
        .with_scale(0.33, 0.33, 0.33)
        .with_translation(-1.5, 0.33, -0.75);

    let world = World::new()
        .with_objects(vec![floor, roof, left_wall, right_wall, back_left_wall, back_right_wall, middle, right, left])
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
    canvas.export("examples/example04.png").unwrap();
}
