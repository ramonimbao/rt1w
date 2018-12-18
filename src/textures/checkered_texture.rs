use std::rc::Rc;

use serde_json::Value;

use crate::materials::{lambertian::Lambertian, Material};
use crate::textures::{constant_texture::ConstantTexture, Texture};
use crate::util::vector3::Vec3;

pub struct CheckeredTexture {
    odd: Rc<Texture>,
    even: Rc<Texture>,
    scale: f64,
}

impl CheckeredTexture {
    pub fn new(odd: Rc<Texture>, even: Rc<Texture>, scale: f64) -> Rc<CheckeredTexture> {
        Rc::new(CheckeredTexture { odd, even, scale })
    }
}

impl Texture for CheckeredTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (self.scale * p.x).sin() * (self.scale * p.y).sin() * (self.scale * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
