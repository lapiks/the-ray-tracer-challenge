pub mod shapes {
    pub use shape::Shape;
    pub use sphere::Sphere;
    pub use plane::Plane;
    pub use cube::Cube;
    pub use group::Group;
    pub use triangle::Triangle;

    pub mod shape;
    pub mod sphere;
    pub mod plane;
    pub mod cube;
    pub mod triangle;
    pub mod group;
    mod test_shape;
}

pub mod lights {
    pub use light::Light;
    pub use point_light::PointLight;
    pub use area_light::AreaLight;

    pub mod light;
    pub mod point_light;
    pub mod area_light;
}

pub use object::Object;
pub use world::World;
pub use material::Material;
pub use camera::Camera;
pub use canvas::Canvas;
pub use color::Color;
pub use pattern::Pattern;
pub use yaml::YamlLoader;

mod ray;
mod intersection;
pub mod transformations;
pub mod object;
pub mod material;
pub mod world;
pub mod canvas;
pub mod color;
pub mod camera;
pub mod pattern;
mod sequence;
pub mod yaml;