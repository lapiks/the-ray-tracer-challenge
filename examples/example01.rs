use ray_tracer::Camera;

fn main() {
    let camera = Camera::new(800, 600);
    let canvas = camera.render();
    canvas.export("examples/example01.png").unwrap();
}
