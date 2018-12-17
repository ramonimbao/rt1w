use crate::util::{hitable::HitRecord, ray::Ray, vector3::Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn emit(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;
