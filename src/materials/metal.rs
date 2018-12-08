use std::rc::Rc;

use serde_json::Value;

use crate::materials::Material;
use crate::util::{hitable::HitRecord, math, ray::Ray, vector3::Vec3};

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
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
        *attenuation = self.albedo;
        math::dot(&scattered.direction, &rec.normal) > 0.0
    }
}

pub fn load_from_json(values: &Value) -> Rc<Material> {
    let r = values["material"]["albedo"]["r"].as_f64();
    let g = values["material"]["albedo"]["g"].as_f64();
    let b = values["material"]["albedo"]["b"].as_f64();
    let (r, g, b) = match (r, g, b) {
        (Some(r), Some(g), Some(b)) => (r, g, b),
        (_, _, _) => (1.0, 1.0, 1.0),
    };

    let fuzz = values["material"]["fuzz"].as_f64();
    let fuzz = match fuzz {
        Some(f) => f,
        _ => 0.0,
    };

    Rc::new(Metal::new(Vec3::new(r, g, b), fuzz))
}
