use ray_tracer::Canvas;

fn main() {
    let canvas = Canvas::new(800, 600);
    canvas.export("examples/example01.png").unwrap();
}
