use std::sync::Arc;

use crate::materials::Material;
use crate::textures::Texture;
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
