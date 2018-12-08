use std::rc::Rc;

use serde_json::Value;

use crate::materials::Material;
use crate::textures::{constant_texture::ConstantTexture, Texture};
use crate::util::{hitable::HitRecord, math, ray::Ray, vector3::Vec3};

pub struct Lambertian {
    albedo: Rc<Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<Texture>) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p + rec.normal + math::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p, r_in.time);
        *attenuation = self.albedo.value(0.0, 0.0, rec.p);
        true
    }
}

// TODO: Implement loading checkerboard pattern from JSON files
pub fn load_from_json(values: &Value) -> Rc<Material> {
    let r = values["material"]["color"]["r"].as_f64();
    let g = values["material"]["color"]["g"].as_f64();
    let b = values["material"]["color"]["b"].as_f64();
    let (r, g, b) = match (r, g, b) {
        (Some(r), Some(g), Some(b)) => (r, g, b),
        (_, _, _) => (1.0, 0.0, 0.0),
    };

    Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        r, g, b,
    )))))
}
