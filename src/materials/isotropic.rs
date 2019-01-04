use std::sync::Arc;

use serde_json::Value;

use crate::materials::{create_material, Material, MaterialType};
use crate::textures::{Texture, TextureType};
use crate::util::{hitable::HitRecord, math, ray::Ray, vector3::Vec3};

pub struct Isotropic {
    texture: Arc<Texture + Sync + Send>,
}

impl Isotropic {
    pub fn create(texture: Arc<Texture + Sync + Send>) -> Arc<Isotropic> {
        Arc::new(Isotropic { texture })
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, math::random_in_unit_sphere(), r_in.time);
        *attenuation = self.texture.value(rec.u, rec.v, rec.p);
        true
    }
}

pub fn load_from_json(values: &Value, texture_type: TextureType) -> Arc<Material + Sync + Send> {
    create_material(values, texture_type, MaterialType::Isotropic)
}
