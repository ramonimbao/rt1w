use std::rc::Rc;

use serde_json::Value;

use crate::materials::{dielectric, lambertian, metal, Material};
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
    pub fn new(position: Vec3, normal: Vec3, material: Rc<Material>) -> Plane {
        Plane {
            position,
            normal: normal,
            material,
        }
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
                rec.material = self.material.clone();
                return true;
            }
        }
        false
    }
}

pub fn load_from_json(values: &Value) -> Vec<Box<Hitable>> {
    let mut list: Vec<Box<Hitable>> = Vec::new();

    let length = match values["planes"].as_array() {
        Some(n) => n.len(),
        _ => 0,
    };

    for i in 0..length {
        if !values["planes"][i].is_object() {
            break;
        }

        // Get the parameters
        let px = values["planes"][i]["position"]["x"].as_f64();
        let py = values["planes"][i]["position"]["y"].as_f64();
        let pz = values["planes"][i]["position"]["z"].as_f64();
        let (px, py, pz) = match (px, py, pz) {
            (Some(x), Some(y), Some(z)) => (x, y, z),
            (_, _, _) => continue,
        };

        let nx = values["planes"][i]["normal"]["x"].as_f64();
        let ny = values["planes"][i]["normal"]["y"].as_f64();
        let nz = values["planes"][i]["normal"]["z"].as_f64();
        let (nx, ny, nz) = match (nx, ny, nz) {
            (Some(x), Some(y), Some(z)) => (x, y, z),
            (_, _, _) => continue,
        };

        let material = values["planes"][i]["material"]["type"].as_str();
        let material: Rc<Material> = match material {
            Some("lambertian") => lambertian::load_from_json(&values["planes"][i]),
            Some("metal") => metal::load_from_json(&values["planes"][i]),
            Some("dielectric") => dielectric::load_from_json(&values["planes"][i]),
            _ => continue,
        };

        list.push(Box::new(Plane::new(
            Vec3::new(px, py, pz),
            Vec3::new(nx, ny, nz),
            material,
        )));
    }

    list
}
