use std::sync::Arc;

use rand::Rng;
use serde_json::Value;

use crate::materials::Material;
use crate::util::{hitable::HitRecord, json, math, ray::Ray, vector3::Vec3};

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Arc<Dielectric> {
        Arc::new(Dielectric { refractive_index })
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal;
        let reflected = math::reflect(&r_in.direction, &rec.normal);
        let ni_over_nt;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted = Vec3::zero();
        let reflection_probability;
        let cosine;
        if math::dot(&r_in.direction, &rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * math::dot(&r_in.direction, &rec.normal)
                / r_in.direction.length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -math::dot(&r_in.direction, &rec.normal) / r_in.direction.length();
        }
        if math::refract(&r_in.direction, &outward_normal, ni_over_nt, &mut refracted) {
            reflection_probability = math::schlik(cosine, self.refractive_index);
        } else {
            *scattered = Ray::new(rec.p, reflected, r_in.time);
            reflection_probability = 1.0;
        }

        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < reflection_probability {
            *scattered = Ray::new(rec.p, reflected, r_in.time);
        } else {
            *scattered = Ray::new(rec.p, refracted, r_in.time);
        }
        true
    }
}

pub fn load_from_json(values: &Value) -> Arc<Material> {
    let ri = json::get_f64_or_rand(&values["material"]["refractive_index"]);
    let ri = match ri {
        Some(f) => f,
        _ => 1.0,
    };

    Dielectric::new(ri)
}
