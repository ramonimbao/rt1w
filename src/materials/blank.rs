use std::rc::Rc;

use crate::materials::Material;
use crate::util::{hitable::HitRecord, math, ray::Ray, vector3::Vec3};

pub struct Blank {}

impl Blank {
    pub fn new() -> Rc<Blank> {
        Rc::new(Blank {})
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
