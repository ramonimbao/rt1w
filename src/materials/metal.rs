use std::sync::Arc;

use serde_json::Value;

use crate::materials::{create_material, Material, MaterialType};
use crate::textures::{Texture, TextureType};
use crate::util::{hitable::HitRecord, json, math, ray::Ray, vector3::Vec3};

pub struct Metal {
    albedo: Arc<Texture + Sync + Send>,
    fuzz: f64,
}

impl Metal {
    pub fn create(albedo: Arc<Texture + Sync + Send>, fuzz: f64) -> Arc<Metal> {
        Arc::new(Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        })
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = math::reflect(
            &math::unit_vector(&r_in.direction),
            &math::unit_vector(&rec.normal),
        );
        *scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * math::random_in_unit_sphere(),
            r_in.time,
        );
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        math::dot(&scattered.direction, &rec.normal) > 0.0
    }
}

// The code here and in the lambert repeat. Maybe there's a way to generalize it?
pub fn load_from_json(values: &Value, texture_type: &TextureType) -> Arc<Material + Sync + Send> {
    let fuzz = json::get_f64_or_rand(&values["material"]["fuzz"]);
    let fuzz = match fuzz {
        Some(f) => f,
        _ => 0.0,
    };

    create_material(values, texture_type, &MaterialType::Metal(fuzz))
}
