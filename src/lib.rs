pub mod shapes {
    pub use shape::Shape;
    pub use sphere::Sphere;
    pub use plane::Plane;

    pub mod shape;
    pub mod sphere;
    pub mod plane;
    pub mod cube;
    mod test_shape;
}

pub use object::Object;
pub use world::World;
pub use light::PointLight;
pub use material::Material;
pub use camera::Camera;
pub use canvas::Canvas;
pub use color::Color;
pub use pattern::Pattern;

mod ray;
mod intersection;
pub mod transformations;
pub mod object;
pub mod light;
pub mod material;
pub mod world;
pub mod canvas;
pub mod color;
pub mod camera;
pub mod pattern;