use std::sync::Arc;

use crate::materials::{blank::Blank, Material};
use crate::util::{ray::Ray, vector3::Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            u: 0.0,
            v: 0.0,
            p: Vec3::zero(),
            normal: Vec3::zero(),
            material: Blank::create(),
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
