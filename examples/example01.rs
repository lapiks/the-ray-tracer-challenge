use ray_tracer::{Camera, World, Object, shapes::{Sphere, Shape}};

fn main() {
    let camera = Camera::new(800, 600);

    let s = Sphere::default();
    let o = Object::new(Shape::Sphere(s))
        .with_scale(1.0, 0.5, 1.0)
        .with_translation(2.0, 0.0, 0.0);

    let world = World::new()
        .with_objects(vec![o]);

    let canvas = camera.render(&world);
    canvas.export("examples/example01.png").unwrap();
}
