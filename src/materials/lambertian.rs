use std::sync::Arc;

use image;
use serde_json::Value;

use crate::materials::{create_material, Material, MaterialType};
use crate::textures::{Texture, TextureType};
use crate::util::{hitable::HitRecord, json, math, ray::Ray, vector3::Vec3};

pub struct Lambertian {
    albedo: Arc<Texture + Sync + Send>,
}

impl Lambertian {
    pub fn new(albedo: Arc<Texture + Sync + Send>) -> Arc<Lambertian> {
        Arc::new(Lambertian { albedo })
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

pub fn load_from_json(values: &Value, texture_type: &TextureType) -> Arc<Material + Sync + Send> {
    create_material(values, texture_type, MaterialType::Lambertian)
}
