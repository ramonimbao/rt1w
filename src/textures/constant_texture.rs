use std::rc::Rc;

use serde_json::Value;

use crate::materials::{lambertian::Lambertian, Material};
use crate::textures::Texture;
use crate::util::vector3::Vec3;

pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> Rc<ConstantTexture> {
        Rc::new(ConstantTexture { color })
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _: f64, _: f64, _: Vec3) -> Vec3 {
        self.color
    }
}

pub fn load_from_json(values: &Value) -> Rc<Material> {
    let r = values["material"]["color"]["r"].as_f64();
    let g = values["material"]["color"]["g"].as_f64();
    let b = values["material"]["color"]["b"].as_f64();
    let (r, g, b) = match (r, g, b) {
        (Some(r), Some(g), Some(b)) => (r, g, b),
        (_, _, _) => (0.0, 0.0, 0.0),
    };
    Lambertian::new(ConstantTexture::new(Vec3::new(r, g, b)))
}
