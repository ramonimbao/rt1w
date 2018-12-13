use std::rc::Rc;

use serde_json::Value;

use crate::materials::{dielectric, diffuse_light, lambertian, metal, Material};
use crate::textures::TextureType;
use crate::util::{
    hitable::{HitRecord, Hitable},
    hitable_list::HitableList,
    math,
    ray::Ray,
    vector3::Vec3,
};

pub struct Cuboid {
    object: HitableList,
}

// Derp. I forgot there's a `hitable_list` which implements exactly what I wanted already.
impl Cuboid {
    pub fn new(origin: Vec3, size: Vec3, material: Rc<Material>) -> Box<Hitable> {
        let mut components: Vec<Box<Hitable>> = Vec::new();

        let min = origin;
        let max = origin + size;

        components.push(RectXY::new(
            Vec3::new(min.x, min.y, 0.0),
            Vec3::new(max.x, max.y, 0.0),
            min.z,
            true,
            material.clone(),
        ));
        components.push(RectXY::new(
            Vec3::new(min.x, min.y, 0.0),
            Vec3::new(max.x, max.y, 0.0),
            max.z,
            false,
            material.clone(),
        ));
        components.push(RectXZ::new(
            Vec3::new(min.x, 0.0, min.z),
            Vec3::new(max.x, 0.0, max.z),
            min.y,
            true,
            material.clone(),
        ));
        components.push(RectXZ::new(
            Vec3::new(min.x, 0.0, min.z),
            Vec3::new(max.x, 0.0, max.z),
            max.y,
            false,
            material.clone(),
        ));
        components.push(RectYZ::new(
            Vec3::new(0.0, min.y, min.z),
            Vec3::new(0.0, max.y, max.z),
            min.x,
            true,
            material.clone(),
        ));
        components.push(RectYZ::new(
            Vec3::new(0.0, min.y, min.z),
            Vec3::new(0.0, max.y, max.z),
            max.x,
            false,
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

pub fn load_from_json(values: &Value) -> Vec<Box<Hitable>> {
    let mut list: Vec<Box<Hitable>> = Vec::new();

    let id = "cuboids";

    let length = match values[id].as_array() {
        Some(n) => n.len(),
        _ => 0,
    };

    for i in 0..length {
        // Get the parameters
        let px = values[id][i]["position"]["x"].as_f64();
        let py = values[id][i]["position"]["y"].as_f64();
        let pz = values[id][i]["position"]["z"].as_f64();
        let sx = values[id][i]["size"]["x"].as_f64();
        let sy = values[id][i]["size"]["y"].as_f64();
        let sz = values[id][i]["size"]["z"].as_f64();
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
                eprintln!("ERROR: Can't get material of cuboid {}! Skipping...", i);
                continue;
            }
        };

        list.push(Cuboid::new(
            Vec3::new(px, py, pz),
            Vec3::new(sx, sy, sz),
            material,
        ));
    }

    list
}

// Eww, lotsa repeatin' code here...

struct RectXY {
    min: Vec3,
    max: Vec3,
    // Z for min and max will be disregarded
    z: f64,
    normal_flip: bool,
    material: Rc<Material>,
}

impl RectXY {
    pub fn new(
        min: Vec3,
        max: Vec3,
        z: f64,
        normal_flip: bool,
        material: Rc<Material>,
    ) -> Box<RectXY> {
        Box::new(RectXY {
            min,
            max,
            z,
            normal_flip,
            material,
        })
    }
}

impl Hitable for RectXY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.z - r.origin.z) / r.direction.z;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;
        if x < self.min.x || x > self.max.x || y < self.min.y || y > self.max.y {
            return false;
        }
        rec.u = (x - self.min.x) / (self.max.x - self.min.x)
            * if self.normal_flip { -1.0 } else { 1.0 };
        rec.v = (self.min.y - y) / (self.max.y - self.min.y);
        rec.t = t;
        rec.material = self.material.clone();
        rec.p = r.point_at_parameter(t);
        rec.normal = if self.normal_flip {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        };

        true
    }
}

struct RectXZ {
    min: Vec3,
    max: Vec3,
    // Z for min and max will be disregarded
    y: f64,
    normal_flip: bool,
    material: Rc<Material>,
}

impl RectXZ {
    pub fn new(
        min: Vec3,
        max: Vec3,
        y: f64,
        normal_flip: bool,
        material: Rc<Material>,
    ) -> Box<RectXZ> {
        Box::new(RectXZ {
            min,
            max,
            y,
            normal_flip,
            material,
        })
    }
}

impl Hitable for RectXZ {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.y - r.origin.y) / r.direction.y;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;
        if x < self.min.x || x > self.max.x || z < self.min.z || z > self.max.z {
            return false;
        }
        rec.u = (x - self.min.x) / (self.max.x - self.min.x)
            * if self.normal_flip { -1.0 } else { 1.0 };
        rec.v = (z - self.min.z) / (self.max.z - self.min.z);
        rec.t = t;
        rec.material = self.material.clone();
        rec.p = r.point_at_parameter(t);
        rec.normal = if self.normal_flip {
            Vec3::new(0.0, -1.0, 0.0)
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };

        true
    }
}

struct RectYZ {
    min: Vec3,
    max: Vec3,
    // Z for min and max will be disregarded
    x: f64,
    normal_flip: bool,
    material: Rc<Material>,
}

impl RectYZ {
    pub fn new(
        min: Vec3,
        max: Vec3,
        x: f64,
        normal_flip: bool,
        material: Rc<Material>,
    ) -> Box<RectYZ> {
        Box::new(RectYZ {
            min,
            max,
            x,
            normal_flip,
            material,
        })
    }
}

impl Hitable for RectYZ {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.x - r.origin.x) / r.direction.x;
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.origin.y + t * r.direction.y;
        let z = r.origin.z + t * r.direction.z;
        if y < self.min.y || y > self.max.y || z < self.min.z || z > self.max.z {
            return false;
        }
        rec.u = (z - self.min.z) / (self.max.z - self.min.z)
            * if self.normal_flip { 1.0 } else { -1.0 };
        rec.v = (self.min.y - y) / (self.max.y - self.min.y);
        rec.t = t;
        rec.material = self.material.clone();
        rec.p = r.point_at_parameter(t);
        rec.normal = if self.normal_flip {
            Vec3::new(-1.0, 0.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        true
    }
}
