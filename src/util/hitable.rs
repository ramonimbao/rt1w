use std::rc::Rc;

use crate::materials::{lambertian::Lambertian, Material};
use crate::textures::constant_texture::ConstantTexture;
use crate::util::{ray::Ray, vector3::Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<Material>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::zero(),
            normal: Vec3::zero(),
            material: Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::zero())))),
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
