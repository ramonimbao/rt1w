use std::fs::OpenOptions;
use std::sync::Arc;

use serde_json::Value;

use crate::materials::{
    blank::Blank,
    dielectric, diffuse_light,
    lambertian::{self, Lambertian},
    metal, Material,
};
use crate::shapes::{constant_medium::ConstantMedium, triangle::Triangle};
use crate::textures::{
    checkered_texture::CheckeredTexture, constant_texture::ConstantTexture, TextureType,
};
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
    pub fn new(
        filename: &str,
        material: Arc<Material + Sync + Send>,
        scale: f64,
    ) -> Box<Hitable + Sync> {
        let mut triangles: Vec<Box<Hitable + Sync>> = Vec::new();

        let mut file = OpenOptions::new().read(true).open(filename).unwrap();
        let stl = stl_io::read_stl(&mut file).unwrap();

        let mut vertices: Vec<Vec3> = Vec::new();

        for v in &stl.vertices {
            vertices.push(Vec3::new(
                f64::from(v[0]) * scale,
                f64::from(v[1]) * scale,
                f64::from(v[2]) * scale,
            ));
        }

        for indices in &stl.faces {
            triangles.push(Triangle::new(
                vec![
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

pub fn load_from_json(values: &Value) -> Vec<Box<Hitable + Sync>> {
    let mut list: Vec<Box<Hitable + Sync>> = Vec::new();

    let id = "meshes";

    let length = match values[id].as_array() {
        Some(n) => n.len(),
        _ => 0,
    };

    for i in 0..length {
        //Get the parameters
        let density = json::get_f64_or_rand(&values[id][i]["density"]);

        let scale = json::get_f64_or_rand(&values[id][i]["scale"]);
        let scale = match scale {
            Some(s) => s,
            _ => 1.0,
        };

        let filename = values[id][i]["filename"].as_str();
        let filename = match filename {
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
                eprintln!(
                    "ERROR: Can't get rotation of mesh {}! Defaulting to (0,0,0)...",
                    i
                );
                (0.0, 0.0, 0.0)
            }
        };

        let material = values[id][i]["material"]["type"].as_str();
        let material: Arc<Material + Sync + Send> = match material {
            Some("matte/constant") => {
                lambertian::load_from_json(&values[id][i], &TextureType::Constant)
            }
            Some("matte/checkered") => {
                lambertian::load_from_json(&values[id][i], &TextureType::Checkered)
            }
            Some("matte/image") => lambertian::load_from_json(&values[id][i], &TextureType::Image),
            Some("matte/noise") => lambertian::load_from_json(&values[id][i], &TextureType::Noise),
            Some("metal/constant") => metal::load_from_json(&values[id][i], &TextureType::Constant),
            Some("metal/checkered") => {
                metal::load_from_json(&values[id][i], &TextureType::Checkered)
            }
            Some("metal/image") => metal::load_from_json(&values[id][i], &TextureType::Image),
            Some("metal/noise") => metal::load_from_json(&values[id][i], &TextureType::Noise),
            Some("dielectric") => dielectric::load_from_json(&values[id][i]),
            Some("light") => diffuse_light::load_from_json(&values[id][i]),
            _ => {
                eprintln!("ERROR: Can't get material of sphere {}! Skipping...", i);
                continue;
            }
        };

        match density {
            Some(density) => {
                list.push(Translate::new(
                    Rotate::new(
                        ConstantMedium::new(
                            density,
                            Mesh::new(filename, Blank::new(), scale),
                            material,
                        ),
                        Vec3::new(rx, ry, rz),
                    ),
                    Vec3::new(px, py, pz),
                ));
            }
            None => {
                list.push(Translate::new(
                    Rotate::new(Mesh::new(filename, material, scale), Vec3::new(rx, ry, rz)),
                    Vec3::new(px, py, pz),
                ));
            }
        }
    }

    list
}
