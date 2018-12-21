use std::sync::Arc;

use image;
use serde_json::Value;

use crate::materials::Material;
use crate::textures::{constant_texture::ConstantTexture, Texture};
use crate::util::{hitable::HitRecord, json, ray::Ray, vector3::Vec3};

pub struct DiffuseLight {
    emitter: Arc<Texture + Sync + Send>,
}

impl DiffuseLight {
    pub fn new(emitter: Arc<Texture + Sync + Send>) -> Arc<DiffuseLight> {
        Arc::new(DiffuseLight { emitter })
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &mut HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emit(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emitter.value(u, v, *p)
    }
}

pub fn load_from_json(values: &Value) -> Arc<Material + Sync + Send> {
    let r = json::get_f64_or_rand(&values["material"]["color"]["r"]);
    let g = json::get_f64_or_rand(&values["material"]["color"]["g"]);
    let b = json::get_f64_or_rand(&values["material"]["color"]["b"]);
    let (r, g, b) = match (r, g, b) {
        (Some(r), Some(g), Some(b)) => (r, g, b),
        (_, _, _) => (1.0, 1.0, 1.0),
    };

    DiffuseLight::new(ConstantTexture::new(Vec3::new(r, g, b)))
}
