use std::sync::Arc;

use rand::Rng;

use crate::materials::Material;
use crate::util::{
    hitable::{HitRecord, Hitable},
    ray::Ray,
    vector3::Vec3,
};

pub struct ConstantMedium {
    density: f64,
    object: Box<Hitable + Sync>,
    material: Arc<Material + Sync + Send>,
}

impl ConstantMedium {
    pub fn new(
        density: f64,
        object: Box<Hitable + Sync>,
        material: Arc<Material + Sync + Send>,
    ) -> Box<ConstantMedium> {
        Box::new(ConstantMedium {
            density,
            object,
            material,
        })
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let (mut rec1, mut rec2) = (HitRecord::new(), HitRecord::new());
        let mut rng = rand::thread_rng();
        if self.object.hit(r, std::f64::MIN, std::f64::MAX, &mut rec1) {
            if self
                .object
                .hit(r, rec1.t + 0.0001, std::f64::MAX, &mut rec2)
            {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return false;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let distance_inside_boundary = (rec2.t - rec1.t) * r.direction.length();
                let hit_distance = -(1.0 / self.density) * rng.gen::<f64>().log10();
                if hit_distance < distance_inside_boundary {
                    rec.t = rec1.t + hit_distance / r.direction.length();
                    rec.p = r.point_at_parameter(rec.t);
                    rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
                    rec.material = self.material.clone();
                    return true;
                }
            }
        }
        false
    }
}
