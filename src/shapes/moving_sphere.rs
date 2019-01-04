use std::sync::Arc;

use serde_json::Value;

use crate::materials::{
    blank::Blank, dielectric, diffuse_light, isotropic, lambertian, metal, Material,
};
use crate::shapes::constant_medium::ConstantMedium;
use crate::textures::TextureType;
use crate::transform::translate::Translate;
use crate::util::{
    hitable::{HitRecord, Hitable},
    json, math,
    ray::Ray,
    vector3::Vec3,
};

pub struct MovingSphere {
    radius: f64,
    t0: f64,
    t1: f64,
    center0: Vec3,
    center1: Vec3,
    pub material: Arc<Material + Sync + Send>,
}

impl MovingSphere {
    pub fn create(
        center0: Vec3,
        center1: Vec3,
        t0: f64,
        t1: f64,
        radius: f64,
        material: Arc<Material + Sync + Send>,
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

pub fn load_from_json(values: &Value, _verbose: bool) -> Vec<Box<Hitable + Sync>> {
    let mut list: Vec<Box<Hitable + Sync>> = Vec::new();

    let id = "moving_spheres";

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

            let p0x = json::get_f64_or_rand(&values[id][i]["positions"][0]["x"]);
            let p0y = json::get_f64_or_rand(&values[id][i]["positions"][0]["y"]);
            let p0z = json::get_f64_or_rand(&values[id][i]["positions"][0]["z"]);
            let t0 = json::get_f64_or_rand(&values[id][i]["positions"][0]["t"]);

            let p1x = json::get_f64_or_rand(&values[id][i]["positions"][1]["x"]);
            let p1y = json::get_f64_or_rand(&values[id][i]["positions"][1]["y"]);
            let p1z = json::get_f64_or_rand(&values[id][i]["positions"][1]["z"]);
            let t1 = json::get_f64_or_rand(&values[id][i]["positions"][1]["t"]);

            let (p0x, p0y, p0z, p1x, p1y, p1z, t0, t1) =
                match (p0x, p0y, p0z, p1x, p1y, p1z, t0, t1) {
                    (
                        Some(x0),
                        Some(y0),
                        Some(z0),
                        Some(x1),
                        Some(y1),
                        Some(z1),
                        Some(t0),
                        Some(t1),
                    ) => (x0, y0, z0, x1, y1, z1, t0, t1),
                    (_, _, _, _, _, _, _, _) => {
                        eprintln!(
                            "ERROR: Can't get positions of moving_sphere {}! Skipping...",
                            i
                        );
                        continue;
                    }
                };
            let position_difference = Vec3::new(p1x - p0x, p1y - p0y, p1z - p0z);

            let radius = match json::get_f64_or_rand(&values[id][i]["radius"]) {
                Some(r) => r,
                _ => {
                    eprintln!(
                        "ERROR: Can't get radius of moving_sphere {}! Skipping...",
                        i
                    );
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
                    eprintln!(
                        "ERROR: Can't get material of moving_sphere {}! Skipping...",
                        i
                    );
                    continue;
                }
            };

            match density {
                Some(density) => {
                    list.push(Translate::translate(
                        ConstantMedium::create(
                            density,
                            MovingSphere::create(
                                Vec3::zero(),
                                position_difference,
                                t0,
                                t1,
                                radius,
                                Blank::create(),
                            ),
                            material,
                        ),
                        Vec3::new(p0x, p0y, p0z),
                    ));
                }
                _ => {
                    list.push(Translate::translate(
                        MovingSphere::create(
                            Vec3::zero(),
                            position_difference,
                            t0,
                            t1,
                            radius,
                            material,
                        ),
                        Vec3::new(p0x, p0y, p0z),
                    ));
                }
            }
        }
    }

    list
}
