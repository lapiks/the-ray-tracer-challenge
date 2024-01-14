use std::{path::Path, fmt::Debug};

use glam::dvec3;

use crate::{shapes::{Group, Triangle, Shape, SmoothTriangle}, Object};

pub struct ObjLoader {
    objects: Vec<Object>,
}

impl ObjLoader {
    pub fn load_from_path<P: AsRef<Path> + Debug>(path: P) -> Self {
        let (models, _) =
            tobj::load_obj(
                &path,
                &tobj::LoadOptions {
                    triangulate: true,
                    ..Default::default()
                }
            )
            .expect("Failed to OBJ load file");

        println!("Number of models = {}", models.len());

        let mut objects = Vec::default();

        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            println!("");
            println!("model[{}].name = \'{}\'", i, m.name);

            let mut triangles = Vec::default();

            let has_normals = !mesh.normals.is_empty();

            for index in (0..mesh.indices.len()).step_by(3) {
                let vertex_index = mesh.indices[index] as usize;
                let p1 = dvec3(
                    mesh.positions[3 * vertex_index] as f64,
                    mesh.positions[3 * vertex_index + 1] as f64,
                    mesh.positions[3 * vertex_index + 2] as f64
                );
                let vertex_index = mesh.indices[index + 1] as usize;
                let p2 = dvec3(
                    mesh.positions[3 * vertex_index] as f64,
                    mesh.positions[3 * vertex_index + 1] as f64,
                    mesh.positions[3 * vertex_index + 2] as f64
                );
                let vertex_index = mesh.indices[index + 2] as usize;
                let p3 = dvec3(
                    mesh.positions[3 * vertex_index] as f64,
                    mesh.positions[3 * vertex_index + 1] as f64,
                    mesh.positions[3 * vertex_index + 2] as f64
                );
                
                match has_normals {
                    true => {
                        let vertex_index = mesh.indices[index] as usize;
                        let n1 = dvec3(
                            mesh.normals[3 * vertex_index] as f64,
                            mesh.normals[3 * vertex_index + 1] as f64,
                            mesh.normals[3 * vertex_index + 2] as f64
                        );
                        let vertex_index = mesh.indices[index + 1] as usize;
                        let n2 = dvec3(
                            mesh.normals[3 * vertex_index] as f64,
                            mesh.normals[3 * vertex_index + 1] as f64,
                            mesh.normals[3 * vertex_index + 2] as f64
                        );
                        let vertex_index = mesh.indices[index + 2] as usize;
                        let n3 = dvec3(
                            mesh.normals[3 * vertex_index] as f64,
                            mesh.normals[3 * vertex_index + 1] as f64,
                            mesh.normals[3 * vertex_index + 2] as f64
                        );

                        triangles.push(
                            Object::new(
                                Shape::SmoothTriangle(
                                    SmoothTriangle::new(p1, p2, p3, n1, n2, n3)
                                )
                            )
                        );
                    },
                    false => {
                        triangles.push(
                            Object::new(
                                Shape::Triangle(
                                    Triangle::new(p1, p2, p3)
                                )
                            )
                        );
                    },
                }                
            }

            objects.push(
                Object::new(
                    Shape::Group(
                        Group::default()
                        .with_objects(triangles)
                    )
                )
            );
        }

        Self {
            objects,
        }
    }

    pub fn objects(self) -> Vec<Object> {
        self.objects
    }
}