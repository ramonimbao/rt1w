use std::sync::Arc;

use serde_json::Value;

use crate::materials::{dielectric, diffuse_light, isotropic, lambertian, metal, Blank, Material};
use crate::shapes::constant_medium::ConstantMedium;
use crate::textures::TextureType;
use crate::transform::translate::Translate;
use crate::util::{
    hitable::{HitRecord, Hitable},
    json, math,
    ray::Ray,
    vector3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<Material + Sync + Send>,
}

impl Sphere {
    pub fn create(center: Vec3, radius: f64, material: Arc<Material + Sync + Send>) -> Box<Sphere> {
        Box::new(Sphere {
            center,
            radius,
            material,
        })
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = math::dot(&r.direction, &r.direction);
        let b = math::dot(&oc, &r.direction);
        let c = math::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                math::get_sphere_uv(&rec.normal, &mut rec.u, &mut rec.v);
                rec.material = self.material.clone();
                return true;
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                math::get_sphere_uv(&rec.normal, &mut rec.u, &mut rec.v);
                rec.material = self.material.clone();
                return true;
            }
        }
        false
    }
}

pub fn load_from_json(values: &Value, _verbose: bool) -> Vec<Box<Hitable + Sync>> {
    let mut list: Vec<Box<Hitable + Sync>> = Vec::new();

    let id = "spheres";

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
                    eprintln!("ERROR: Can't get position of sphere {}! Skipping...", i);
                    continue;
                }
            };

            let radius = match json::get_f64_or_rand(&values[id][i]["radius"]) {
                Some(r) => r,
                _ => {
                    eprintln!("ERROR: Can't get radius of sphere {}! Skipping...", i);
                    continue;
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
                        ConstantMedium::create(
                            density,
                            Sphere::create(Vec3::zero(), radius, Blank::create()),
                            material,
                        ),
                        Vec3::new(px, py, pz),
                    ));
                }
                None => {
                    list.push(Translate::translate(
                        Sphere::create(Vec3::zero(), radius, material),
                        Vec3::new(px, py, pz),
                    ));
                }
            }
        }
    }

    list
}
