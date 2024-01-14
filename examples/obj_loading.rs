use std::f64::consts::PI;

use glam::dvec3;
use ray_tracer::{Camera, World, Color, transformations, lights::{Light, PointLight}, ObjLoader, shapes::{Shape, Plane}, Object, Material, pattern::{PatternObject, CheckerPattern, PlainPattern}, Pattern};

fn main() {
    let camera = Camera::new(1000, 800, PI / 3.0)
        .with_transform(
            transformations::view_transform(
                dvec3(0.0, 3.0, -3.0),
                dvec3(0.0, 1.0, 0.0),
                dvec3(0.0, 1.0, 0.0)
            )
        )
        .with_antialiasing(3);

    let l1 = Light::PointLight(PointLight::new(
        dvec3(-10.0, 10.0, -10.0), 
        Color::white()
    ));

    let obj_loader = ObjLoader::load_from_path("examples/obj/teapot.obj");
    let mut objects = obj_loader.objects();
    objects[0] = objects[0].clone()
    .with_scale(0.1, 0.1, 0.1)
    .with_rotation_x(-PI / 2.0)
    .transform();
    // .with_material(
    //     Material::new()
    //     .with_ambient(0.0)
    //     .with_diffuse(0.0)
    //     .with_specular(0.0)
    //     .with_transparency(1.0)
    //     .with_refractive_index(1.5)
    //     .with_pattern(
    //         PatternObject::new(
    //             Pattern::Plain(
    //                 PlainPattern::new(Color::black())
    //             )
    //         )
    //     )
    // );

    let floor = Object::new(Shape::Plane(Plane::default()))
    .with_material(
        Material::new()
        .with_pattern(
            PatternObject::new(
                Pattern::Checker(
                    CheckerPattern::new(
                        Color::new(0.9, 0.9, 0.9),
                        Color::new(0.7, 0.7, 0.7)
                    )
                )
            )
        )
    );

    let wall = floor.clone()
    .with_rotation_x(PI / 2.0)
    .with_translation(0.0, 0.0, 10.0)
    .transform();

    objects.push(floor);
    objects.push(wall);

    println!("min: {}, max: {}", objects[0].bounds().min(), objects[0].bounds().max());

    let world = World::new()
        .with_objects(objects)
        .with_lights(vec![l1]);

    let canvas = camera.render(&world, 5);
    canvas.export("examples/obj_loading.png").unwrap();
}
