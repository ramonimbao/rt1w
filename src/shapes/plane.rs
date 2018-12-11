use std::rc::Rc;

use serde_json::Value;

use crate::materials::{dielectric, lambertian, metal, Material};
use crate::textures::TextureType;
use crate::util::{
    hitable::{HitRecord, Hitable},
    math,
    ray::Ray,
    vector3::Vec3,
};

pub struct Plane {
    position: Vec3,
    normal: Vec3,
    material: Rc<Material>,
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3, material: Rc<Material>) -> Box<Plane> {
        Box::new(Plane {
            position,
            normal: normal,
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

pub fn load_from_json(values: &Value) -> Vec<Box<Hitable>> {
    let mut list: Vec<Box<Hitable>> = Vec::new();

    let id = "planes";

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
            (_, _, _) => continue,
        };

        let nx = values[id][i]["normal"]["x"].as_f64();
        let ny = values[id][i]["normal"]["y"].as_f64();
        let nz = values[id][i]["normal"]["z"].as_f64();
        let (nx, ny, nz) = match (nx, ny, nz) {
            (Some(x), Some(y), Some(z)) => (x, y, z),
            (_, _, _) => continue,
        };

        let material = values[id][i]["material"]["type"].as_str();
        let material: Rc<Material> = match material {
            Some("constant") => lambertian::load_from_json(&values[id][i], TextureType::Constant),
            Some("checkered") => lambertian::load_from_json(&values[id][i], TextureType::Checkered),
            Some("image") => lambertian::load_from_json(&values[id][i], TextureType::Image),
            Some("noise") => lambertian::load_from_json(&values[id][i], TextureType::Noise),
            Some("metal") => metal::load_from_json(&values[id][i]),
            Some("dielectric") => dielectric::load_from_json(&values[id][i]),
            _ => continue,
        };

        list.push(Plane::new(
            Vec3::new(px, py, pz),
            Vec3::new(nx, ny, nz),
            material,
        ));
    }

    list
}
