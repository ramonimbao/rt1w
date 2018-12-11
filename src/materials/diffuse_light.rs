use std::rc::Rc;

use serde_json::Value;

use crate::materials::Material;
use crate::textures::{constant_texture::ConstantTexture, Texture};
use crate::util::{hitable::HitRecord, ray::Ray, vector3::Vec3};

pub struct DiffuseLight {
    emitter: Rc<Texture>,
}

impl DiffuseLight {
    pub fn new(emitter: Rc<Texture>) -> Rc<DiffuseLight> {
        Rc::new(DiffuseLight { emitter })
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emit(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emitter.value(u, v, *p)
    }
}

pub fn load_from_json(values: &Value) -> Rc<Material> {
    let r = values["material"]["color"]["r"].as_f64();
    let g = values["material"]["color"]["g"].as_f64();
    let b = values["material"]["color"]["b"].as_f64();
    let (r, g, b) = match (r, g, b) {
        (Some(r), Some(g), Some(b)) => (r, g, b),
        (_, _, _) => (1.0, 1.0, 1.0),
    };

    let fuzz = values["material"]["fuzz"].as_f64();
    let fuzz = match fuzz {
        Some(f) => f,
        _ => 0.0,
    };

    DiffuseLight::new(ConstantTexture::new(Vec3::new(r, g, b)))
}
