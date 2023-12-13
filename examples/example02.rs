use std::f32::consts::PI;

use glam::{Vec3, vec3};
use ray_tracer::{Camera, World, Object, shapes::{Sphere, Shape}, Color, Material, PointLight, transformations};

fn main() {
    let l = PointLight::new(
        Vec3::new(-10.0, 10.0, -10.0), 
        Color::white()
    );

    let m =  Material::default()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0);

    let floor = Object::new(Shape::Sphere(Sphere::default()))
        .with_material(m.clone())
        .with_scale(10.0, 0.01, 10.0);

    let left_wall = Object::new(Shape::Sphere(Sphere::default()))
        .with_material(m.clone())
        .with_scale(10.0, 0.01, 10.0)
        .with_rotation_x(PI/2.0)
        .with_rotation_y(-PI/4.0)
        .with_translation(0.0, 0.0, 5.0);

    let right_wall = Object::new(Shape::Sphere(Sphere::default()))
        .with_material(m.clone())
        .with_scale(10.0, 0.01, 10.0)
        .with_rotation_x(PI/2.0)
        .with_rotation_y(PI/4.0)
        .with_translation(0.0, 0.0, 5.0);

    let middle =  Object::new(Shape::Sphere(Sphere::default()))
        .with_material(
            Material::default()
                .with_color(Color::new(0.1, 1.0, 0.5))
                .with_diffuse(0.7)
                .with_specular(0.3)
        )
        .with_translation(-0.5, 1.0, 0.5);

    let right =  Object::new(Shape::Sphere(Sphere::default()))
        .with_material(
            Material::default()
                .with_color(Color::new(0.5, 1.0, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3)
        )
        .with_scale(0.5, 0.5, 0.5)
        .with_translation(1.5, 0.5, -0.5);

    let left =  Object::new(Shape::Sphere(Sphere::default()))
        .with_material(
            Material::default()
                .with_color(Color::new(1.0, 0.8, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3)
        )
        .with_scale(0.33, 0.33, 0.33)
        .with_translation(-1.5, 0.33, -0.75);

    let world = World::new()
        .with_objects(vec![floor, left_wall, right_wall, middle, right, left])
        .with_lights(vec![l]);

    let camera = Camera::new(1920, 1080, PI / 3.0)
        .with_transform(
            transformations::view_transform(
                vec3(0.0, 1.5, -5.0),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 1.0, 0.0)
            )
        );

    let canvas = camera.render(&world);
    canvas.export("examples/example02.png").unwrap();
}
