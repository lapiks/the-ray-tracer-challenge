use std::f32::consts::PI;

use glam::{Vec3, Mat4, vec3};
use ray_tracer::{Camera, World, Object, shapes::{Sphere, Shape}, Color, Material, PointLight};

fn main() {
    let camera = Camera::new(1920, 1080, PI / 2.0)
        .with_transform(
            Mat4::from_translation(
                vec3(0.0, 0.0, -5.0)
            )
        );

    let l1 = PointLight::new(
        Vec3::new(-10.0, 10.0, -10.0), 
        Color::white()
    );

    let l2 = PointLight::new(
        Vec3::new(10.0, 10.0, 10.0), 
        Color::white()
    );

    let s = Sphere::default();
    let o1 = Object::new(Shape::Sphere(s))
        .with_scale(0.75, 0.75, 0.75)
        .with_translation(1.0, 0.0, 0.0)
        .with_material(
            Material::default()
                .with_color(Color::new(1.0, 0.2, 1.0))
        );

    let o2 = Object::new(Shape::Sphere(s))
        .with_translation(-1.0, 0.0, 0.0)
        .with_material(
            Material::default()
                .with_color(Color::new(0.2, 1.0, 1.0)));

    let world = World::new()
        .with_objects(vec![o1, o2])
        .with_lights(vec![l1, l2]);

    let canvas = camera.render(&world);
    canvas.export("examples/example01.png").unwrap();
}
