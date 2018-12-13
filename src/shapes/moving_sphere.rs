use std::rc::Rc;

use serde_json::Value;

use crate::materials::{dielectric, diffuse_light, lambertian, metal, Material};
use crate::textures::TextureType;
use crate::transform::{rotate::RotateY, translate::Translate};
use crate::util::{
    hitable::{HitRecord, Hitable},
    math,
    ray::Ray,
    vector3::Vec3,
};

pub struct MovingSphere {
    radius: f64,
    t0: f64,
    t1: f64,
    center0: Vec3,
    center1: Vec3,
    material: Rc<Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        t0: f64,
        t1: f64,
        radius: f64,
        material: Rc<Material>,
    ) -> Box<MovingSphere> {
        Box::new(MovingSphere {
            radius,
            t0,
            t1,
            center0,
            center1,
            material,
        })
    }

    pub fn get_center(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.t0) / (self.t1 - self.t0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.get_center(r.time);
        let a = math::dot(&r.direction, &r.direction);
        let b = math::dot(&oc, &r.direction);
        let c = math::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.get_center(r.time)) / self.radius;
                math::get_sphere_uv(&rec.normal, &mut rec.u, &mut rec.v);
                rec.material = self.material.clone();
                return true;
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.get_center(r.time)) / self.radius;
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

    let id = "moving_spheres";

    let length = match values[id].as_array() {
        Some(n) => n.len(),
        _ => 0,
    };

    for i in 0..length {
        // Get the parameters
        let p0x = values[id][i]["positions"][0]["x"].as_f64();
        let p0y = values[id][i]["positions"][0]["y"].as_f64();
        let p0z = values[id][i]["positions"][0]["z"].as_f64();
        let t0 = values[id][i]["positions"][0]["t"].as_f64();
        let p1x = values[id][i]["positions"][1]["x"].as_f64();
        let p1y = values[id][i]["positions"][1]["y"].as_f64();
        let p1z = values[id][i]["positions"][1]["z"].as_f64();
        let t1 = values[id][i]["positions"][1]["t"].as_f64();
        let (p0x, p0y, p0z, p1x, p1y, p1z, t0, t1) = match (p0x, p0y, p0z, p1x, p1y, p1z, t0, t1) {
            (Some(x0), Some(y0), Some(z0), Some(x1), Some(y1), Some(z1), Some(t0), Some(t1)) => {
                (x0, y0, z0, x1, y1, z1, t0, t1)
            }
            (_, _, _, _, _, _, _, _) => {
                eprintln!(
                    "ERROR: Can't get position of moving_sphere {}! Skipping...",
                    i
                );
                continue;
            }
        };
        let position_difference = Vec3::new(p1x - p0x, p1y - p0y, p1z - p0z);

        let radius = values[id][i]["radius"].as_f64();
        let radius = match radius {
            Some(r) => r,
            _ => {
                eprintln!(
                    "ERROR: Can't get radius of moving_sphere {}! Skipping...",
                    i
                );
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
                eprintln!(
                    "ERROR: Can't get material of moving_sphere {}! Skipping...",
                    i
                );
                continue;
            }
        };

        list.push(Translate::new(
            MovingSphere::new(Vec3::zero(), position_difference, t0, t1, radius, material),
            Vec3::new(p0x, p0y, p0z),
        ));
    }

    list
}
