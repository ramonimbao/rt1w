use std::sync::Arc;

use serde_json::Value;

use crate::materials::{blank::Blank, dielectric, diffuse_light, lambertian, metal, Material};
use crate::shapes::constant_medium::ConstantMedium;
use crate::textures::TextureType;
use crate::transform::{rotate::Rotate, translate::Translate};
use crate::util::{
    hitable::{HitRecord, Hitable},
    hitable_list::HitableList,
    json,
    ray::Ray,
    vector3::Vec3,
};

pub struct Cuboid {
    object: HitableList,
}

// Derp. I forgot there's a `hitable_list` which implements exactly what I wanted already.
impl Cuboid {
    pub fn new(
        origin: Vec3,
        size: Vec3,
        material: Arc<Material + Sync + Send>,
    ) -> Box<Hitable + Sync> {
        let mut components: Vec<Box<Hitable + Sync>> = Vec::new();

        let min = origin - (size / 2.0);
        let max = origin + (size / 2.0);

        components.push(Rect::new(
            RectType::RectXY,
            Vec3::new(min.x, min.y, 0.0),
            Vec3::new(max.x, max.y, 0.0),
            min.z,
            NormalType::Flipped,
            material.clone(),
        ));
        components.push(Rect::new(
            RectType::RectXY,
            Vec3::new(min.x, min.y, 0.0),
            Vec3::new(max.x, max.y, 0.0),
            max.z,
            NormalType::NotFlipped,
            material.clone(),
        ));
        components.push(Rect::new(
            RectType::RectXZ,
            Vec3::new(min.x, 0.0, min.z),
            Vec3::new(max.x, 0.0, max.z),
            min.y,
            NormalType::Flipped,
            material.clone(),
        ));
        components.push(Rect::new(
            RectType::RectXZ,
            Vec3::new(min.x, 0.0, min.z),
            Vec3::new(max.x, 0.0, max.z),
            max.y,
            NormalType::NotFlipped,
            material.clone(),
        ));
        components.push(Rect::new(
            RectType::RectYZ,
            Vec3::new(0.0, min.y, min.z),
            Vec3::new(0.0, max.y, max.z),
            min.x,
            NormalType::Flipped,
            material.clone(),
        ));
        components.push(Rect::new(
            RectType::RectYZ,
            Vec3::new(0.0, min.y, min.z),
            Vec3::new(0.0, max.y, max.z),
            max.x,
            NormalType::NotFlipped,
            material.clone(),
        ));
        Box::new(Cuboid {
            object: HitableList::new(components),
        })
    }
}

impl Hitable for Cuboid {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.object.hit(r, t_min, t_max, rec)
    }
}

pub fn load_from_json(values: &Value) -> Vec<Box<Hitable + Sync>> {
    let mut list: Vec<Box<Hitable + Sync>> = Vec::new();

    let id = "cuboids";

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
            let sx = json::get_f64_or_rand(&values[id][i]["size"]["x"]);
            let sy = json::get_f64_or_rand(&values[id][i]["size"]["y"]);
            let sz = json::get_f64_or_rand(&values[id][i]["size"]["z"]);
            let (px, py, pz, sx, sy, sz) = match (px, py, pz, sx, sy, sz) {
                (Some(px), Some(py), Some(pz), Some(sx), Some(sy), Some(sz)) => {
                    (px, py, pz, sx, sy, sz)
                }
                (_, _, _, _, _, _) => {
                    eprintln!(
                        "ERROR: Can't get position and size of cuboid {}! Skipping...",
                        i
                    );
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
                        "ERROR: Can't get rotation of cuboid {}! Defaulting to (0,0,0)...",
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
                    eprintln!("ERROR: Can't get material of cuboid {}! Skipping...", i);
                    continue;
                }
            };

            match density {
                Some(density) => {
                    list.push(Translate::new(
                        Rotate::new(
                            ConstantMedium::new(
                                density,
                                Cuboid::new(Vec3::zero(), Vec3::new(sx, sy, sz), Blank::new()),
                                material,
                            ),
                            Vec3::new(rx, ry, rz),
                        ),
                        Vec3::new(px, py, pz),
                    ));
                }
                None => {
                    list.push(Translate::new(
                        Rotate::new(
                            Cuboid::new(Vec3::zero(), Vec3::new(sx, sy, sz), material),
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

// Eww, lotsa repeatin' code here...

pub enum RectType {
    RectXY,
    RectXZ,
    RectYZ,
}

#[derive(PartialEq)]
pub enum NormalType {
    Flipped,
    NotFlipped,
}

pub struct Rect {
    rect_type: RectType,
    min: Vec3,
    max: Vec3,
    offset: f64,
    normal_flip: NormalType,
    material: Arc<Material + Sync + Send>,
}

impl Rect {
    pub fn new(
        rect_type: RectType,
        min: Vec3,
        max: Vec3,
        offset: f64,
        normal_flip: NormalType,
        material: Arc<Material + Sync + Send>,
    ) -> Box<Rect> {
        Box::new(Rect {
            rect_type,
            min,
            max,
            offset,
            normal_flip,
            material,
        })
    }
}

impl Hitable for Rect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        match self.rect_type {
            RectType::RectXY => {
                let t = (self.offset - r.origin.z) / r.direction.z;
                if t < t_min || t > t_max {
                    return false;
                }
                let x = r.origin.x + t * r.direction.x;
                let y = r.origin.y + t * r.direction.y;
                if x < self.min.x || x > self.max.x || y < self.min.y || y > self.max.y {
                    return false;
                }
                rec.u = (x - self.min.x) / (self.max.x - self.min.x)
                    * if self.normal_flip == NormalType::Flipped {
                        -1.0
                    } else {
                        1.0
                    };
                rec.v = (self.min.y - y) / (self.max.y - self.min.y);
                rec.t = t;
                rec.p = r.point_at_parameter(t);
                rec.normal = if self.normal_flip == NormalType::Flipped {
                    Vec3::new(0.0, 0.0, -1.0)
                } else {
                    Vec3::new(0.0, 0.0, 1.0)
                };
            }

            RectType::RectXZ => {
                let t = (self.offset - r.origin.y) / r.direction.y;
                if t < t_min || t > t_max {
                    return false;
                }
                let x = r.origin.x + t * r.direction.x;
                let z = r.origin.z + t * r.direction.z;
                if x < self.min.x || x > self.max.x || z < self.min.z || z > self.max.z {
                    return false;
                }
                rec.u = (x - self.min.x) / (self.max.x - self.min.x)
                    * if self.normal_flip == NormalType::Flipped {
                        -1.0
                    } else {
                        1.0
                    };
                rec.v = (z - self.min.z) / (self.max.z - self.min.z);
                rec.t = t;
                rec.p = r.point_at_parameter(t);
                rec.normal = if self.normal_flip == NormalType::Flipped {
                    Vec3::new(0.0, -1.0, 0.0)
                } else {
                    Vec3::new(0.0, 1.0, 0.0)
                };
            }

            RectType::RectYZ => {
                let t = (self.offset - r.origin.x) / r.direction.x;
                if t < t_min || t > t_max {
                    return false;
                }
                let y = r.origin.y + t * r.direction.y;
                let z = r.origin.z + t * r.direction.z;
                if y < self.min.y || y > self.max.y || z < self.min.z || z > self.max.z {
                    return false;
                }
                rec.u = (z - self.min.z) / (self.max.z - self.min.z)
                    * if self.normal_flip == NormalType::Flipped {
                        1.0
                    } else {
                        -1.0
                    };
                rec.v = (self.min.y - y) / (self.max.y - self.min.y);
                rec.t = t;
                rec.p = r.point_at_parameter(t);
                rec.normal = if self.normal_flip == NormalType::Flipped {
                    Vec3::new(-1.0, 0.0, 0.0)
                } else {
                    Vec3::new(1.0, 0.0, 0.0)
                };
            }
        }

        rec.material = self.material.clone();

        true
    }
}
