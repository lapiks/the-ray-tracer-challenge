use glam::Vec3;
use ray_tracer::{Camera, World, Object, shapes::{Sphere, Shape}, Color, Material, PointLight};

fn main() {
    let camera = Camera::new(1920, 1080)
        .with_translation(0.0, 0.0, -2.0);

    let l = PointLight::new(&Vec3::new(-10.0, 10.0, -10.0), &Color::white());

    let m = Material::default()
        .with_color(&Color::new(1.0, 0.2, 1.0));

    let s = Sphere::default();
    let o = Object::new(Shape::Sphere(s))
        //.with_scale(1.0, 0.5, 1.0)
        //.with_translation(2.0, 0.0, 0.0)
        .with_material(m);

    let world = World::new()
        .with_objects(vec![o])
        .with_lights(vec![l]);

    let canvas = camera.render(&world);
    canvas.export("examples/example01.png").unwrap();
}
