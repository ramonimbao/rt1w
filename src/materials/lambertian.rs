use std::rc::Rc;

use image;
use serde_json::Value;

use crate::materials::Material;
use crate::textures::{
    checkered_texture, constant_texture, image_texture, noise_texture, Texture, TextureType,
};
use crate::util::{hitable::HitRecord, math, ray::Ray, vector3::Vec3};

pub struct Lambertian {
    albedo: Rc<Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<Texture>) -> Rc<Lambertian> {
        Rc::new(Lambertian { albedo })
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
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}

pub fn load_from_json(values: &Value, texture_type: TextureType) -> Rc<Material> {
    let result = match texture_type {
        TextureType::Checkered => checkered_texture::load_from_json(&values),
        TextureType::Constant => constant_texture::load_from_json(&values),
        TextureType::Image => image_texture::load_from_json(&values),
        TextureType::Noise => noise_texture::load_from_json(&values),
    };

    result
}
