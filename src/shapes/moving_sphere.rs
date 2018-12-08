use std::rc::Rc;

use crate::materials::Material;
use crate::util::{
    hitable::{HitRecord, Hitable},
    math,
    ray::Ray,
    vector3::Vec3,
};

pub struct MovingSphere {
    radius: f64,
    t0: f64,
    t1: f64,
    center0: Vec3,
    center1: Vec3,
    material: Rc<Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        t0: f64,
        t1: f64,
        radius: f64,
        material: Rc<Material>,
    ) -> Box<MovingSphere> {
        Box::new(MovingSphere {
            radius,
            t0,
            t1,
            center0,
            center1,
            material,
        })
    }

    pub fn get_center(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.t0) / (self.t1 - self.t0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.get_center(r.time);
        let a = math::dot(&r.direction, &r.direction);
        let b = math::dot(&oc, &r.direction);
        let c = math::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.get_center(r.time)) / self.radius;
                rec.material = self.material.clone();
                return true;
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.get_center(r.time)) / self.radius;
                rec.material = self.material.clone();
                return true;
            }
        }
        false
    }
}

// TODO: Support loading of moving spheres from JSON
pub fn load_from_json() {}
