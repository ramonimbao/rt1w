use std::fs::OpenOptions;
use std::sync::Arc;

use serde_json::Value;

use crate::materials::{
    blank::Blank, dielectric, diffuse_light, isotropic, lambertian, metal, Material,
};
use crate::shapes::{
    constant_medium::ConstantMedium,
    triangle::{Triangle, Vertex},
};
use crate::textures::TextureType;
use crate::transform::{rotate::Rotate, translate::Translate};
use crate::util::{
    hitable::{HitRecord, Hitable},
    hitable_list::HitableList,
    json, math,
    ray::Ray,
    vector3::Vec3,
};

pub struct Mesh {
    triangles: HitableList,
}

impl Mesh {
    pub fn create(
        filename: &str,
        material: Arc<Material + Sync + Send>,
        scale: f64,
    ) -> Box<Hitable + Sync> {
        let mut triangles: Vec<Box<Hitable + Sync>> = Vec::new();

        // TODO: Handle this gracefully.
        let mut file = OpenOptions::new().read(true).open(filename).unwrap();
        let stl = stl_io::read_stl(&mut file).unwrap();

        let mut vertices: Vec<Vertex> = Vec::new();

        for v in &stl.vertices {
            vertices.push(Vertex::new(
                Vec3::new(
                    f64::from(v[0]) * scale,
                    f64::from(v[1]) * scale,
                    f64::from(v[2]) * scale,
                ),
                Vec3::zero(),
            ));
        }

        for indices in &stl.faces {
            let (v0, v1, v2) = (
                vertices[indices.vertices[0] as usize].position,
                vertices[indices.vertices[1] as usize].position,
                vertices[indices.vertices[2] as usize].position,
            );

            vertices[indices.vertices[0] as usize].normal += math::cross(&(v1 - v0), &(v2 - v0));
            vertices[indices.vertices[1] as usize].normal += math::cross(&(v1 - v0), &(v2 - v0));
            vertices[indices.vertices[2] as usize].normal += math::cross(&(v1 - v0), &(v2 - v0));
        }

        for v in vertices.iter_mut() {
            v.normal = math::unit_vector(&v.normal);
        }

        for indices in &stl.faces {
            triangles.push(Triangle::create(
                [
                    vertices[indices.vertices[0] as usize],
                    vertices[indices.vertices[1] as usize],
                    vertices[indices.vertices[2] as usize],
                ],
                material.clone(),
            ));
        }

        Box::new(Mesh {
            triangles: HitableList::new(triangles),
        })
    }
}

impl Hitable for Mesh {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.triangles.hit(r, t_min, t_max, rec)
    }
}

pub fn load_from_json(values: &Value, verbose: bool) -> Vec<Box<Hitable + Sync>> {
    let mut list: Vec<Box<Hitable + Sync>> = Vec::new();

    let id = "meshes";

    let length = match values[id].as_array() {
        Some(n) => n.len(),
        _ => 0,
    };

    for i in 0..length {
        //Get the parameters
        let copies = match json::get_f64_or_rand(&values[id][i]["copies"]) {
            Some(n) => n,
            _ => 1.0,
        } as usize;

        for _ in 0..copies {
            let density = json::get_f64_or_rand(&values[id][i]["density"]);

            let scale = match json::get_f64_or_rand(&values[id][i]["scale"]) {
                Some(s) => s,
                _ => 1.0,
            };

            let filename = match values[id][i]["filename"].as_str() {
                Some(filename) => filename,
                _ => continue,
            };

            let px = json::get_f64_or_rand(&values[id][i]["position"]["x"]);
            let py = json::get_f64_or_rand(&values[id][i]["position"]["y"]);
            let pz = json::get_f64_or_rand(&values[id][i]["position"]["z"]);
            let (px, py, pz) = match (px, py, pz) {
                (Some(px), Some(py), Some(pz)) => (px, py, pz),
                (_, _, _) => {
                    eprintln!("ERROR: Can't get position of mesh {}! Skipping...", i);
                    continue;
                }
            };

            let rx = json::get_f64_or_rand(&values[id][i]["rotation"]["x"]);
            let ry = json::get_f64_or_rand(&values[id][i]["rotation"]["y"]);
            let rz = json::get_f64_or_rand(&values[id][i]["rotation"]["z"]);
            let (rx, ry, rz) = match (rx, ry, rz) {
                (Some(rx), Some(ry), Some(rz)) => (rx, ry, rz),
                (_, _, _) => {
                    if verbose {
                        eprintln!(
                            "ERROR: Can't get rotation of mesh {}! Defaulting to (0,0,0)...",
                            i
                        );
                    }
                    (0.0, 0.0, 0.0)
                }
            };

            let material: Arc<Material + Sync + Send> = match values[id][i]["material"]["type"]
                .as_str()
            {
                Some("matte/constant") => {
                    lambertian::load_from_json(&values[id][i], TextureType::Constant)
                }
                Some("matte/checkered") => {
                    lambertian::load_from_json(&values[id][i], TextureType::Checkered)
                }
                Some("matte/image") => {
                    lambertian::load_from_json(&values[id][i], TextureType::Image)
                }
                Some("matte/noise") => {
                    lambertian::load_from_json(&values[id][i], TextureType::Noise)
                }
                Some("metal/constant") => {
                    metal::load_from_json(&values[id][i], TextureType::Constant)
                }
                Some("metal/checkered") => {
                    metal::load_from_json(&values[id][i], TextureType::Checkered)
                }
                Some("metal/image") => metal::load_from_json(&values[id][i], TextureType::Image),
                Some("metal/noise") => metal::load_from_json(&values[id][i], TextureType::Noise),
                Some("isotropic/constant") => {
                    isotropic::load_from_json(&values[id][i], TextureType::Constant)
                }
                Some("isotropic/checkered") => {
                    isotropic::load_from_json(&values[id][i], TextureType::Checkered)
                }
                Some("isotropic/image") => {
                    isotropic::load_from_json(&values[id][i], TextureType::Image)
                }
                Some("isotropic/noise") => {
                    isotropic::load_from_json(&values[id][i], TextureType::Noise)
                }
                Some("dielectric") => dielectric::load_from_json(&values[id][i]),
                Some("light") => diffuse_light::load_from_json(&values[id][i]),
                _ => {
                    eprintln!("ERROR: Can't get material of sphere {}! Skipping...", i);
                    continue;
                }
            };

            match density {
                Some(density) => {
                    list.push(Translate::translate(
                        Rotate::rotate(
                            ConstantMedium::create(
                                density,
                                Mesh::create(filename, Blank::create(), scale),
                                material,
                            ),
                            Vec3::new(rx, ry, rz),
                        ),
                        Vec3::new(px, py, pz),
                    ));
                }
                None => {
                    list.push(Translate::translate(
                        Rotate::rotate(
                            Mesh::create(filename, material, scale),
                            Vec3::new(rx, ry, rz),
                        ),
                        Vec3::new(px, py, pz),
                    ));
                }
            }
        }
    }

    list
}
