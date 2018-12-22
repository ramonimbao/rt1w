use std::sync::Arc;

use serde_json::Value;

use crate::materials::{blank::Blank, dielectric, diffuse_light, lambertian, metal, Material};
use crate::shapes::constant_medium::ConstantMedium;
use crate::textures::TextureType;
use crate::transform::{rotate::Rotate, translate::Translate};
use crate::util::{
    hitable::{HitRecord, Hitable},
    json, math,
    ray::Ray,
    vector3::Vec3,
};

pub struct Plane {
    position: Vec3,
    normal: Vec3,
    pub material: Arc<Material + Sync + Send>,
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3, material: Arc<Material + Sync + Send>) -> Box<Plane> {
        Box::new(Plane {
            position,
            normal,
            material,
        })
    }
}

impl Hitable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let denominator = math::dot(&r.direction, &-self.normal);
        if denominator > 1.0e-6 {
            let t = math::dot(&(r.origin - self.position), &self.normal) / denominator;
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = self.normal;
                math::get_plane_uv(
                    &math::cross(&(r.origin - self.position), &rec.p),
                    &rec.normal,
                    &mut rec.u,
                    &mut rec.v,
                );
                rec.material = self.material.clone();
                return true;
            }
        }
        false
    }
}

pub fn load_from_json(values: &Value) -> Vec<Box<Hitable + Sync>> {
    let mut list: Vec<Box<Hitable + Sync>> = Vec::new();

    let id = "planes";

    let length = match values[id].as_array() {
        Some(n) => n.len(),
        _ => 0,
    };

    for i in 0..length {
        // Get the parameters
        let copies = match json::get_f64_or_rand(&values[id][i]["copies"]) {
            Some(n) => n,
            _ => 1.0,
        } as usize;

        for _ in 0..copies {
            let density = json::get_f64_or_rand(&values[id][i]["density"]);

            let px = json::get_f64_or_rand(&values[id][i]["position"]["x"]);
            let py = json::get_f64_or_rand(&values[id][i]["position"]["y"]);
            let pz = json::get_f64_or_rand(&values[id][i]["position"]["z"]);
            let (px, py, pz) = match (px, py, pz) {
                (Some(x), Some(y), Some(z)) => (x, y, z),
                (_, _, _) => {
                    eprintln!("ERROR: Can't get position of plane {}! Skipping...", i);
                    continue;
                }
            };

            let nx = json::get_f64_or_rand(&values[id][i]["normal"]["x"]);
            let ny = json::get_f64_or_rand(&values[id][i]["normal"]["y"]);
            let nz = json::get_f64_or_rand(&values[id][i]["normal"]["z"]);
            let (nx, ny, nz) = match (nx, ny, nz) {
                (Some(x), Some(y), Some(z)) => (x, y, z),
                (_, _, _) => {
                    eprintln!("ERROR: Can't get normal of plane {}! Skipping...", i);
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
                        "ERROR: Can't get rotation of plane {}! Defaulting to (0,0,0)...",
                        i
                    );
                    (0.0, 0.0, 0.0)
                }
            };

            let material: Arc<Material + Sync + Send> = match values[id][i]["material"]["type"]
                .as_str()
            {
                Some("matte/constant") => {
                    lambertian::load_from_json(&values[id][i], &TextureType::Constant)
                }
                Some("matte/checkered") => {
                    lambertian::load_from_json(&values[id][i], &TextureType::Checkered)
                }
                Some("matte/image") => {
                    lambertian::load_from_json(&values[id][i], &TextureType::Image)
                }
                Some("matte/noise") => {
                    lambertian::load_from_json(&values[id][i], &TextureType::Noise)
                }
                Some("metal/constant") => {
                    metal::load_from_json(&values[id][i], &TextureType::Constant)
                }
                Some("metal/checkered") => {
                    metal::load_from_json(&values[id][i], &TextureType::Checkered)
                }
                Some("metal/image") => metal::load_from_json(&values[id][i], &TextureType::Image),
                Some("metal/noise") => metal::load_from_json(&values[id][i], &TextureType::Noise),
                Some("dielectric") => dielectric::load_from_json(&values[id][i]),
                Some("light") => diffuse_light::load_from_json(&values[id][i]),
                _ => {
                    eprintln!("ERRPR: Can't get material of plane {}! Skipping...", i);
                    continue;
                }
            };

            match density {
                Some(density) => {
                    list.push(Translate::new(
                        Rotate::new(
                            ConstantMedium::new(
                                density,
                                Plane::new(Vec3::zero(), Vec3::new(nx, ny, nz), Blank::new()),
                                material,
                            ),
                            Vec3::new(rx, ry, rz),
                        ),
                        Vec3::new(px, py, pz),
                    ));
                }
                _ => {
                    list.push(Translate::new(
                        Rotate::new(
                            Plane::new(Vec3::zero(), Vec3::new(nx, ny, nz), material),
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
