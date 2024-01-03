use std::collections::HashMap;

use glam::{DVec3, DMat4};
use yaml_rust::{Yaml, yaml::Hash};

use crate::{Object, PointLight, Camera, transformations, Color, shapes::{Sphere, Plane, Cube, Group, Shape}, Material, pattern::{PatternObject, PlainPattern, StrippedPattern, RingPattern, CheckerPattern, GradientPattern}, Pattern};

extern crate yaml_rust;

pub struct YamlLoader {
    objects: Vec<Object>,
    lights: Vec<PointLight>,
    camera: Option<Camera>,
}

type Defines<'a> = HashMap<&'a str, &'a Hash>;

impl YamlLoader {
    pub fn load_from_str(source: &str) -> Self {
        let docs = yaml_rust::yaml::YamlLoader::load_from_str(source).unwrap();
        let doc = &docs[0];

        let mut camera = None;
        let mut objects = Vec::default();
        let mut lights = Vec::default();

        let mut defines: Defines = HashMap::default();

        let elems = doc.as_vec().expect("The yaml should be an array of elements to add to the scene");
        for elem in elems {
            let hash = elem.as_hash().unwrap();
            
            let _ = match Self::load_str_from_hash(hash, "define") {
                Some(define_name) => defines.insert(define_name, hash),
                None => None,
            };

            match Self::load_str_from_hash(hash, "add") {
                Some(add_value) => {
                    match add_value {
                        "camera" => {
                            camera = Some(Self::load_camera(&hash));
                        }
                        "light" => {
                            lights.push(Self::load_light(&hash));
                        }
                        "sphere" | "plane" | "cube" | "triangle" | "group" => {
                            objects.push(Self::load_object(&hash, &defines).expect("Unable to load object"));
                        }
                        &_ => {
                            panic!("Unsupported entity to add to the scene")
                        }
                    }
                },
                None => (),
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

    fn load_object(hash: &Hash, defines: &Defines) -> Option<Object> {
        let mut object = None;
        match Self::load_str_from_hash(hash, "add").expect("The shape should be a string") {
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

        let default = Object::new(Shape::Sphere(Sphere::default()));

        object
        .map(|o| {
            o
            .with_material(
                Self::load_material(hash, defines)
            )
            .with_transform(
                &Self::load_transform(hash, defines)
            )
            .with_shadow(
                Self::load_bool_from_hash(hash, "shadow").unwrap_or(default.shadow())
            )
        })
    }

    fn load_material(hash: &Hash, defines: &Defines) -> Material {
        /// Extends material hash with define's values
        fn extend_with_defines(defines: &Defines, name: &str, hash: &mut Hash) {
            defines
            .get(name)
            .map(|define_hash| {
                match define_hash.get(&Yaml::from_str("extend")) {
                    Some(extend) => match extend.as_str() {
                        Some(define_name) => {
                            extend_with_defines(defines, define_name, hash);
                        }
                        None => panic!("The extend should have a name"),
                    },
                    None => (),
                }
            
                hash.extend(define_hash.get(&Yaml::from_str("value")).unwrap().as_hash().unwrap().clone());
            });
        }

        let default = Material::default();

        match hash.get(&Yaml::from_str("material")) {
            Some(material_yaml) => {
                let mut material_hash = Hash::new();
                match material_yaml.as_str() {
                    Some(define_name) => {
                        extend_with_defines(defines, define_name, &mut material_hash);
                    },
                    None => material_hash = material_yaml.as_hash().unwrap().clone(),
                }

                Material::default()
                    .with_ambient(
                        Self::load_f64_from_hash(&material_hash, "ambient")
                        .unwrap_or(default.ambient())
                    )
                    .with_diffuse(
                        Self::load_f64_from_hash(&material_hash, "diffuse")
                        .unwrap_or(default.diffuse())
                    )
                    .with_specular(
                        Self::load_f64_from_hash(&material_hash, "specular")
                        .unwrap_or(default.specular())
                    )
                    .with_shininess(
                        Self::load_f64_from_hash(&material_hash, "shininess")
                        .unwrap_or(default.shininess()))
                    .with_reflective(
                        Self::load_f64_from_hash(&material_hash, "reflective")
                        .unwrap_or(default.reflective()))
                    .with_transparency(
                            Self::load_f64_from_hash(&material_hash, "transparency")
                            .unwrap_or(default.transparency()))
                    .with_refractive_index(
                        Self::load_f64_from_hash(&material_hash, "refractive-index")
                        .unwrap_or(default.refractive_index()))
                    .with_pattern(
                        Self::load_pattern(&material_hash, defines)
                        .unwrap_or(default.pattern().clone()))
            },
            None => default,
        }
    }

    fn load_pattern(hash: &Hash, defines: &Defines) -> Option<PatternObject> {
        // if there is a color value, its considered like a plane pattern with this color
        if let Some(color) = Self::load_color_from_hash(hash, "color") {
            return Some(PatternObject::new(
                crate::Pattern::Plain(PlainPattern::new(color))
            ));
        }

        let mut pattern_object = None;

        match Self::load_hash_from_hash(hash, "pattern") {
            Some(pattern_hash) => {
                match Self::load_str_from_hash(pattern_hash, "type").expect("The pattern type should be a string") {
                    "stripes" => {
                        let colors = Self::load_vec_from_hash(pattern_hash, "colors").expect("The pattern colors should be a vec");
                        pattern_object = Some(
                            PatternObject::new(
                                Pattern::Stripped(
                                    StrippedPattern::new(
                                        Self::load_color_from_vec(colors[0].as_vec().expect("A color should be a vec")), 
                                        Self::load_color_from_vec(colors[1].as_vec().expect("A color should be a vec"))
                                    )
                                )
                            )
                        )
                    },
                    "rings" => {
                        let colors = pattern_hash.get(&Yaml::from_str("colors")).unwrap().as_vec().unwrap();
                        pattern_object = Some(
                            PatternObject::new(
                                Pattern::Ring(
                                    RingPattern::new(
                                        Self::load_color_from_vec(colors[0].as_vec().expect("A color should be a vec")), 
                                        Self::load_color_from_vec(colors[1].as_vec().expect("A color should be a vec"))
                                    )
                                )
                            )
                        )
                    },
                    "checkers" => {
                        let colors = pattern_hash.get(&Yaml::from_str("colors")).unwrap().as_vec().unwrap();
                        pattern_object = Some(
                            PatternObject::new(
                                Pattern::Checker(
                                    CheckerPattern::new(
                                        Self::load_color_from_vec(colors[0].as_vec().expect("A color should be a vec")), 
                                        Self::load_color_from_vec(colors[1].as_vec().expect("A color should be a vec"))
                                    )
                                )
                            )
                        )
                    },
                    "gradient" => {
                        let colors = pattern_hash.get(&Yaml::from_str("colors")).unwrap().as_vec().unwrap();
                        pattern_object = Some(
                            PatternObject::new(
                                Pattern::Gradient(
                                    GradientPattern::new(
                                        Self::load_color_from_vec(colors[0].as_vec().expect("A color should be a vec")), 
                                        Self::load_color_from_vec(colors[1].as_vec().expect("A color should be a vec"))
                                    )
                                )
                            )
                        )
                    },
                    &_ => {
                        panic!("Unsupported pattern")
                    }
                }
    
                pattern_object
                .map(|o| {
                    o
                    .with_transform(
                        &Self::load_transform(pattern_hash, defines)
                    )
                })
            },
            None => pattern_object,
        }
    }

    fn load_transform(hash: &Hash, defines: &Defines) -> DMat4 {
        /// Extends transform array with define's values
        fn extend_with_defines(defines: &Defines, name: &str, vec: &mut Vec<Yaml>) {
            defines
            .get(name)
            .map(|define_hash| {
                match define_hash.get(&Yaml::from_str("extend")) {
                    Some(extend) => match extend.as_str() {
                        Some(define_name) => {
                            extend_with_defines(defines, define_name, vec);
                        }
                        None => panic!("The extend should have a name"),
                    },
                    None => (),
                }
            
                match define_hash.get(&Yaml::from_str("value")).unwrap().as_vec() {
                    Some(values) => {
                        for value in values {
                            match value.as_str() {
                                Some(define_name) => {
                                    extend_with_defines(defines, define_name, vec);
                                },
                                None => (),
                            }
                        }
                    },
                    None => (),
                }

                vec.extend(define_hash.get(&Yaml::from_str("value")).unwrap().as_vec().unwrap().clone());
            });
        }

        let mut matrix = DMat4::IDENTITY;

        match hash.get(&Yaml::from_str("transform")) {
            Some(transform_yaml) => {
                let mut transform_vec = Vec::new();

                // transform:
                //   - other-transform
                match transform_yaml.as_vec() {
                    Some(values) => {
                        for value in values {
                            match value.as_str() {
                                Some(define_name) => {
                                    extend_with_defines(defines, define_name, &mut transform_vec);
                                },
                                None => transform_vec.push(value.clone()),
                            }
                        }
                    },
                    None => (),
                }

                // transform: other-transform 
                match transform_yaml.as_str() {
                    Some(define_name) => {
                        extend_with_defines(defines, define_name, &mut transform_vec);
                    },
                    None => (),
                }

                for transformation in transform_vec.into_iter().rev() {
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
            },
            None => (),
        }
        matrix
    }

    fn load_str_from_hash<'a>(hash: &'a Hash, key: &str) -> Option<&'a str> {
        hash
        .get(&Yaml::from_str(key))
        .map(|yaml| Self::unwrap_str(yaml))
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

    fn load_bool_from_hash(hash: &Hash, key: &str) -> Option<bool> {
        hash
        .get(&Yaml::from_str(key))
        .map(|yaml| Self::unwrap_bool(yaml))
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
            Self::load_color_from_vec(yaml.as_vec().unwrap())
        })
    }

    fn load_hash_from_hash<'a>(hash: &'a Hash, key: &'a str) -> Option<&'a Hash> {
        hash
        .get(&Yaml::from_str(key))
        .map(|yaml| Self::unwrap_hash(yaml))
    }

    fn load_vec_from_hash<'a>(hash: &'a Hash, key: &'a str) -> Option<&'a Vec<Yaml>> {
        hash
        .get(&Yaml::from_str(key))
        .map(|yaml| Self::unwrap_vec(yaml))
    }

    fn load_color_from_vec(vec: &Vec<Yaml>) -> Color {
        Color::new(
            Self::unwrap_f64(&vec[0]),
            Self::unwrap_f64(&vec[1]),
            Self::unwrap_f64(&vec[2])
        )
    }

    fn unwrap_str(yaml: &Yaml) -> &str {
        match yaml.as_str() {
            Some(value) => value,
            None => panic!("Unwrapping str failed, the value is not a str"),
        }
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

    fn unwrap_bool(yaml: &Yaml) -> bool {
        match yaml.as_bool() {
            Some(value) => value,
            None => panic!("Unwrapping bool failed, the value is not a bool"),
        }
    }

    fn unwrap_vec(yaml: &Yaml) -> &Vec<Yaml> {
        match yaml.as_vec() {
            Some(value) => value,
            None => panic!("Unwrapping vec failed, the value is not a vec"),
        }
    }

    fn unwrap_hash(yaml: &Yaml) -> &Hash {
        match yaml.as_hash() {
            Some(value) => value,
            None => panic!("Unwrapping hash failed, the value is not an hash"),
        }
    }
}


#[cfg(test)]
pub mod tests {
    use glam::dvec3;

    use super::*;

    #[test]
    fn importing_a_camera_from_a_yaml_scene() {
        let source = "
            - add: camera
              width: 1920
              height: 1080
              field-of-view: 0.7854
              from: [-3, 1, 2.5]
              to: [0, 0.5, 0]
              up: [0, 1, 0]
        ";

        let loader = YamlLoader::load_from_str(source);
        let camera = loader.camera();

        assert!(camera.is_some());
        assert_eq!(camera.unwrap().width(), 1920);
        assert_eq!(camera.unwrap().height(), 1080);
        assert_eq!(camera.unwrap().field_of_view(), 0.7854);
    }
    
    #[test]
    fn importing_a_light_from_a_yaml_scene() {
        let source = "
            - add: light
              position: [-1, 2, 4]
              intensity: [1.5, 1.5, 1.5]
        ";

        let loader = YamlLoader::load_from_str(source);
        let lights = loader.lights();

        assert_eq!(lights.len(), 1);
        assert_eq!(lights[0].position(), dvec3(-1.0, 2.0, 4.0));
        assert_eq!(lights[0].intensity(), Color::new(1.5, 1.5, 1.5));
    }

    #[test]
    fn importing_a_sphere_from_a_yaml_scene() {
        let source = "
            - add: sphere
              transform:
              - [ scale, 0.33, 0.33, 0.33 ]
              - [ translate, -0.25, 0.33, 0 ]
              material:
                color: [0.5, 0.5, 1]
                ambient: 0.1
                diffuse: 0.6
                specular: 0.4
                shininess: 250
                reflective: 0.3
                transparency: 0.5
                refractive-index: 1.5
              shadow: false
        ";

        let loader = YamlLoader::load_from_str(source);
        let objects = loader.objects();

        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].material().ambient(), 0.1);
        assert_eq!(objects[0].material().diffuse(), 0.6);
        assert_eq!(objects[0].material().specular(), 0.4);
        assert_eq!(objects[0].material().shininess(), 250.0);
        assert_eq!(objects[0].material().reflective(), 0.3);
        assert_eq!(objects[0].material().transparency(), 0.5);
        assert_eq!(objects[0].material().refractive_index(), 1.5);
        assert_eq!(objects[0].shadow(), false);

        let s_r_t = objects[0].transform().to_scale_rotation_translation();
        assert_eq!(s_r_t.0, dvec3(0.33, 0.33, 0.33));
        assert_eq!(s_r_t.2, dvec3(-0.25, 0.33, 0.0));
    }


    #[test]
    fn importing_a_yaml_scene_with_material_definitions() {
        let source = "
            - define: a
              value:
                ambient: 0.6
            
            - define: b
              extend: a
              value: 
                diffuse: 0.7
            
            - add: cube
              material: a

            - add: cube
              material: b
        ";

        let loader = YamlLoader::load_from_str(source);
        let objects = loader.objects();

        assert_eq!(objects[0].material().ambient(), 0.6);
        assert_eq!(objects[0].material().diffuse(), Material::default().diffuse());
        assert_eq!(objects[1].material().ambient(), 0.6);
        assert_eq!(objects[1].material().diffuse(), 0.7);
    }

    #[test]
    fn importing_a_yaml_scene_with_transform_definitions_as_extends() {
        let source = "
            - define: a
              value:
              - [ scale, 2, 2, 2 ]
            
            - define: b
              extend: a
              value: 
              - [ scale, 2, 2, 2 ]
            
            - add: cube
              transform: a

            - add: cube
              transform: b
        ";

        let loader = YamlLoader::load_from_str(source);
        let objects = loader.objects();

        let object1_s_r_t = objects[0].transform().to_scale_rotation_translation();
        let object2_s_r_t = objects[1].transform().to_scale_rotation_translation();

        assert_eq!(object1_s_r_t.0, dvec3(2.0, 2.0, 2.0));
        assert_eq!(object2_s_r_t.0, dvec3(4.0, 4.0, 4.0));
    }

    #[test]
    fn importing_a_yaml_scene_with_transform_definitions_as_arrays() {
        let source = "
            - define: a
              value:
              - [ scale, 2, 2, 2 ]
            
            - define: b
              value: 
              - a
              - [ scale, 2, 2, 2 ]
            
            - add: cube
              transform: 
              - a

            - add: cube
              transform: 
              - b
        ";

        let loader = YamlLoader::load_from_str(source);
        let objects = loader.objects();

        let object1_s_r_t = objects[0].transform().to_scale_rotation_translation();
        let object2_s_r_t = objects[1].transform().to_scale_rotation_translation();

        assert_eq!(object1_s_r_t.0, dvec3(2.0, 2.0, 2.0));
        assert_eq!(object2_s_r_t.0, dvec3(4.0, 4.0, 4.0));
    }
}