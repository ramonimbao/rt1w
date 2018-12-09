use std::rc::Rc;

use crate::materials::Material;
use crate::textures::Texture;
use crate::util::{hitable::HitRecord, ray::Ray, vector3::Vec3};

pub struct DiffuseLight {
    emitter: Rc<Texture>,
}

impl DiffuseLight {
    pub fn new(emitter: Rc<Texture>) -> Rc<DiffuseLight> {
        Rc::new(DiffuseLight { emitter })
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emit(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emitter.value(u, v, *p)
    }
}
