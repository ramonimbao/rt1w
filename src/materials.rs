use crate::util::{hitable::HitRecord, ray::Ray, vector3::Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub mod dielectric;
pub mod lambertian;
pub mod metal;
