use glam::{DVec3, DMat4};
use yaml_rust::{Yaml, yaml::Hash};

use crate::{Object, PointLight, Camera, transformations, Color, shapes::{Sphere, Plane, Cube, Group, Shape}, Material, pattern::{PatternObject, PlainPattern}};

extern crate yaml_rust;

pub struct YamlLoader {
    objects: Vec<Object>,
    lights: Vec<PointLight>,
    camera: Option<Camera>,
}

impl YamlLoader {
    pub fn load_from_str(source: &str) -> Self {
        let docs = yaml_rust::yaml::YamlLoader::load_from_str(source).unwrap();
        let doc = &docs[0];

        let mut camera = None;
        let mut objects = Vec::default();
        let mut lights = Vec::default();

        let elems = doc.as_vec().expect("The yaml should be an array of elements to add to the scene");
        for elem in elems {
            let hash = elem.as_hash().unwrap();
            let add = hash.get(&Yaml::from_str("add")).expect("Each element of the hash array should contains a add key");

            match add.as_str().unwrap() {
                "camera" => {
                    camera = Some(Self::load_camera(&hash));
                }
                "light" => {
                    lights.push(Self::load_light(&hash));
                }
                "sphere" | "plane" | "cube" | "triangle" | "group" => {
                    objects.push(Self::load_object(&hash).expect("Unable to load object"));
                }
                &_ => {
                    panic!("Unsupported entity to add to the scene")
                }
            }
        }
        
        YamlLoader {
            objects,
            lights,
            camera
        }
    }

    pub fn objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn lights(&self) -> &Vec<PointLight> {
        &self.lights
    }

    pub fn camera(&self) -> Option<&Camera> {
        self.camera.as_ref()
    }

    fn load_camera(hash: &Hash) -> Camera {
        Camera::new(
            Self::load_i64_from_hash(hash, "width").expect("Camera is missing the width parameter") as usize,
            Self::load_i64_from_hash(hash, "height").expect("Camera is missing the height parameter") as usize,
            Self::load_f64_from_hash(hash, "field-of-view").expect("Camera is missing the field-of-view parameter"),
        )
        .with_transform(
            transformations::view_transform(
                Self::load_dvec3_from_hash(hash, "from").expect("Camera is missing the from parameter"),
                Self::load_dvec3_from_hash(hash, "to").expect("Camera is missing the to parameter"),
                Self::load_dvec3_from_hash(hash, "up").expect("Camera is missing the up parameter"),
            )
        )
    }

    fn load_light(hash: &Hash) -> PointLight {
        PointLight::new(
            Self::load_dvec3_from_hash(hash, "position").expect("The light is missing the position parameter"), 
            Self::load_color_from_hash(hash, "intensity").expect("The light is missing the intensity parameter")
        )
    }

    fn load_object(hash: &Hash) -> Option<Object> {
        let mut object = None;
        match hash.get(&Yaml::from_str("add")).unwrap().as_str().expect("The shape should be a string") {
            "sphere" => {
                object = Some(Object::new(Shape::Sphere(Sphere::default())));
            }
            "plane" => {
                object = Some(Object::new(Shape::Plane(Plane::default())));
            }
            "cube" => {
                object = Some(Object::new(Shape::Cube(Cube::default())));
            }
            "triangle" => {
                //object = Object::new(Shape::Triangle(Triangle::default()));
            }
            "group" => {
                object = Some(Object::new(Shape::Group(Group::default())));
            }
            &_ => {
                panic!("Unsupported shape")
            }
        }

        object
        .map(|o| {
            o
            .with_material(
                Self::load_material(hash.get(&Yaml::from_str("material")).unwrap().as_hash().unwrap())
            )
            .with_transform(
                &Self::load_transform(hash)
            )
        })
    }

    fn load_material(hash: &Hash) -> Material {
        let default = Material::default();

        Material::default()
            .with_ambient(Self::load_f64_from_hash(hash, "ambient").unwrap_or(default.ambient()))
            .with_diffuse(Self::load_f64_from_hash(hash, "diffuse").unwrap_or(default.diffuse()))
            .with_specular(Self::load_f64_from_hash(hash, "specular").unwrap_or(default.specular()))
            .with_shininess(Self::load_f64_from_hash(hash, "shininess").unwrap_or(default.shininess()))
            .with_reflective(Self::load_f64_from_hash(hash, "reflective").unwrap_or(default.reflective()))
            .with_refractive_index(Self::load_f64_from_hash(hash, "refractive_index").unwrap_or(default.refractive_index()))
            .with_transparency(Self::load_f64_from_hash(hash, "transparency").unwrap_or(default.transparency()))
            .with_pattern(Self::load_pattern(hash).unwrap_or(default.pattern().clone()))
    }

    fn load_pattern(hash: &Hash) -> Option<PatternObject> {
        if let Some(color) = Self::load_color_from_hash(hash, "color") {
            return Some(PatternObject::new(
                crate::Pattern::Plain(PlainPattern::new(color))
            ));
        }

        None
    }

    fn load_transform(hash: &Hash) -> DMat4 {
        let mut matrix = DMat4::IDENTITY;
        if let Some(transform) = hash.get(&Yaml::from_str("transform")) {
            for transformation in transform.as_vec().unwrap().into_iter().rev() {
                if let Some(values) = transformation.as_vec() {
                    let operation = &values[0];
                    matrix *= match operation.as_str().unwrap() {
                        "translate" => {
                            DMat4::from_translation(
                                DVec3::new(
                                    Self::unwrap_f64(&values[1]),
                                    Self::unwrap_f64(&values[2]),
                                    Self::unwrap_f64(&values[3])
                                )
                            )
                        }
                        "scale" => {
                            DMat4::from_scale(
                                DVec3::new(
                                    Self::unwrap_f64(&values[1]),
                                    Self::unwrap_f64(&values[2]),
                                    Self::unwrap_f64(&values[3])
                                )
                            )
                        }
                        "rotate-x" => {
                            DMat4::from_rotation_x(
                                Self::unwrap_f64(&values[1])
                            )
                        }
                        "rotate-y" => {
                            DMat4::from_rotation_y(
                                Self::unwrap_f64(&values[1])
                            )
                        }
                        "rotate-z" => {
                            DMat4::from_rotation_z(
                                Self::unwrap_f64(&values[1])
                            )
                        }
                        &_ => {
                            panic!("Unsupported transform operation")
                        }
                    };
                }
            }
        }

        matrix
    }

    fn load_i64_from_hash(hash: &Hash, key: &str) -> Option<i64> {
        hash
        .get(&Yaml::from_str(key))
        .map(|yaml| Self::unwrap_i64(yaml))
    }

    fn load_f64_from_hash(hash: &Hash, key: &str) -> Option<f64> {
        hash
        .get(&Yaml::from_str(key))
        .map(|yaml| Self::unwrap_f64(yaml))
    }

    fn load_dvec3_from_hash(hash: &Hash, key: &str) -> Option<DVec3> {
        hash.get(&Yaml::from_str(key)).map(|yaml| {
            let vec = yaml.as_vec().unwrap();
            DVec3::new(
                Self::unwrap_f64(&vec[0]),
                Self::unwrap_f64(&vec[1]),
                Self::unwrap_f64(&vec[2])
            )
        })
    }

    fn load_color_from_hash(hash: &Hash, key: &str) -> Option<Color> {
        hash.get(&Yaml::from_str(key)).map(|yaml| {
            let vec = yaml.as_vec().unwrap();
            Color::new(
                Self::unwrap_f64(&vec[0]),
                Self::unwrap_f64(&vec[1]),
                Self::unwrap_f64(&vec[2])
            )
        })
    }

    fn unwrap_i64(yaml: &Yaml) -> i64 {
        match yaml.as_i64() {
            Some(value) => value,
            None => panic!("Unwrapping i64 failed, the value is not a i64"),
        }
    }

    fn unwrap_f64(yaml: &Yaml) -> f64 {
        match yaml.as_f64() {
            Some(value) => value,
            None => match yaml.as_i64() {
                Some(value) => value as f64,
                None => panic!("Unwrapping f64 failed, the value is not a f64 or i64"),
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use glam::dvec3;

    use super::*;

    #[test]
    fn importing_a_yaml_scene() {
        let source = "
            - add: camera
              width: 1920
              height: 1080
              field-of-view: 0.7854
              from: [-3, 1, 2.5]
              to: [0, 0.5, 0]
              up: [0, 1, 0]

            - add: light
              position: [-1, 2, 4]
              intensity: [1.5, 1.5, 1.5]

            - add: sphere
              transform:
              - [ scale, 0.33, 0.33, 0.33 ]
              - [ translate, -0.25, 0.33, 0 ]
              material:
               color: [0.5, 0.5, 1]
               ambient: 0.1
               diffuse: 0.6
               specular: 0.4
               reflective: 0.3
               transparency: 0.5
               refractive_index: 1.5
        ";

        let loader = YamlLoader::load_from_str(source);
        let objects = loader.objects();
        let lights = loader.lights();
        let camera = loader.camera();

        assert!(camera.is_some());
        assert_eq!(camera.unwrap().width(), 1920);
        assert_eq!(camera.unwrap().height(), 1080);

        assert_eq!(lights.len(), 1);
        assert_eq!(lights[0].position(), dvec3(-1.0, 2.0, 4.0));
        assert_eq!(lights[0].intensity(), Color::new(1.5, 1.5, 1.5));

        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].material().ambient(), 0.1);
        assert_eq!(objects[0].material().diffuse(), 0.6);
        assert_eq!(objects[0].material().specular(), 0.4);
        assert_eq!(objects[0].material().reflective(), 0.3);
        assert_eq!(objects[0].material().transparency(), 0.5);
        assert_eq!(objects[0].material().refractive_index(), 1.5);

        let s_r_t = objects[0].transform().to_scale_rotation_translation();
        assert_eq!(s_r_t.0, dvec3(0.33, 0.33, 0.33));
        assert_eq!(s_r_t.2, dvec3(-0.25, 0.33, 0.0));

    }
}