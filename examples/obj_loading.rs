use std::f64::consts::PI;

use glam::dvec3;
use ray_tracer::{Camera, World, Color, transformations, lights::{Light, PointLight}, ObjLoader};

fn main() {
    let camera = Camera::new(100, 80, PI / 3.0)
        .with_transform(
            transformations::view_transform(
                dvec3(0.0, 4.0, -8.0),
                dvec3(0.0, 2.0, 0.0),
                dvec3(0.0, 1.0, 0.0)
            )
        )
        .with_antialiasing(3);

    let l1 = Light::PointLight(PointLight::new(
        dvec3(-10.0, 10.0, -10.0), 
        Color::white()
    ));

    let obj_loader = ObjLoader::load_from_path("examples/obj/teapot.obj");

    let world = World::new()
        .with_objects(obj_loader.objects().to_owned())
        .with_lights(vec![l1]);

    let canvas = camera.render(&world, 5);
    canvas.export("examples/obj_loading.png").unwrap();
}
