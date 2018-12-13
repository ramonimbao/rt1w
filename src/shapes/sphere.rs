use std::rc::Rc;

use serde_json::Value;

use crate::materials::{dielectric, diffuse_light, lambertian, metal, Material};
use crate::textures::TextureType;
use crate::transform::{rotate::Rotate, translate::Translate};
use crate::util::{
    hitable::{HitRecord, Hitable},
    math,
    ray::Ray,
    vector3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<Material>) -> Box<Sphere> {
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

pub fn load_from_json(values: &Value) -> Vec<Box<Hitable>> {
    let mut list: Vec<Box<Hitable>> = Vec::new();

    let id = "spheres";

    let length = match values[id].as_array() {
        Some(n) => n.len(),
        _ => 0,
    };

    for i in 0..length {
        // Get the parameters
        let px = values[id][i]["position"]["x"].as_f64();
        let py = values[id][i]["position"]["y"].as_f64();
        let pz = values[id][i]["position"]["z"].as_f64();
        let (px, py, pz) = match (px, py, pz) {
            (Some(x), Some(y), Some(z)) => (x, y, z),
            (_, _, _) => {
                eprintln!("ERROR: Can't get position of sphere {}! Skipping...", i);
                continue;
            }
        };

        let radius = values[id][i]["radius"].as_f64();
        let radius = match radius {
            Some(r) => r,
            _ => {
                eprintln!("ERROR: Can't get radius of sphere {}! Skipping...", i);
                continue;
            }
        };

        let material = values[id][i]["material"]["type"].as_str();
        let material: Rc<Material> = match material {
            Some("constant") => lambertian::load_from_json(&values[id][i], TextureType::Constant),
            Some("checkered") => lambertian::load_from_json(&values[id][i], TextureType::Checkered),
            Some("image") => lambertian::load_from_json(&values[id][i], TextureType::Image),
            Some("noise") => lambertian::load_from_json(&values[id][i], TextureType::Noise),
            Some("metal") => metal::load_from_json(&values[id][i]),
            Some("dielectric") => dielectric::load_from_json(&values[id][i]),
            Some("light") => diffuse_light::load_from_json(&values[id][i]),
            _ => {
                eprintln!("ERROR: Can't get material of sphere {}! Skipping...", i);
                continue;
            }
        };

        list.push(Translate::new(
            Sphere::new(Vec3::zero(), radius, material),
            Vec3::new(px, py, pz),
        ));
    }

    list
}
