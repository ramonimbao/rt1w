use std::rc::Rc;

use crate::materials::Material;
use crate::util::{
    hitable::{HitRecord, Hitable},
    math,
    ray::Ray,
    vector3::Vec3,
};

pub struct Plane {
    position: Vec3,
    normal: Vec3,
    material: Rc<Material>,
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3, material: Rc<Material>) -> Plane {
        Plane {
            position,
            normal: normal,
            material,
        }
    }
}

impl Hitable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let denominator = math::dot(&r.direction, &-self.normal);
        if denominator > 1.0e-6 {
            let t = math::dot(&(r.origin - self.position), &self.normal) / denominator;
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = self.normal;
                rec.material = self.material.clone();
                return true;
            }
        }
        false
    }
}
