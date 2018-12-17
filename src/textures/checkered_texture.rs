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
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}

pub fn load_from_json(values: &Value) -> Rc<Material> {
    let or = values["material"]["colors"][0]["r"].as_f64();
    let og = values["material"]["colors"][0]["g"].as_f64();
    let ob = values["material"]["colors"][0]["b"].as_f64();
    let er = values["material"]["colors"][1]["r"].as_f64();
    let eg = values["material"]["colors"][1]["g"].as_f64();
    let eb = values["material"]["colors"][1]["b"].as_f64();
    let (or, og, ob, er, eg, eb) = match (or, og, ob, er, eg, eb) {
        (Some(or), Some(og), Some(ob), Some(er), Some(eg), Some(eb)) => (or, og, ob, er, eg, eb),
        (_, _, _, _, _, _) => (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    };

    let scale = values["material"]["scale"].as_f64();
    let scale = match scale {
        Some(s) => s,
        _ => 1.0,
    };

    Lambertian::new(CheckeredTexture::new(
        ConstantTexture::new(Vec3::new(or, og, ob)),
        ConstantTexture::new(Vec3::new(er, eg, eb)),
        scale,
    ))
}
