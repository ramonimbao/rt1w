use std::sync::Arc;

use crate::materials::Material;
use crate::util::{hitable::HitRecord, ray::Ray, vector3::Vec3};

pub struct Blank {}

impl Blank {
    pub fn create() -> Arc<Blank> {
        Arc::new(Blank {})
    }
}

impl Material for Blank {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &mut HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}
