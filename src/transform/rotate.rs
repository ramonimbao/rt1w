use crate::util::{
    hitable::{HitRecord, Hitable},
    ray::Ray,
    vector3::Vec3,
};

pub struct RotateY {
    object: Box<Hitable>,
    angle: f64,
    sin_theta: f64,
    cos_theta: f64,
}

impl RotateY {
    pub fn new(object: Box<Hitable>, angle: f64) -> Box<Hitable> {
        let radians = std::f64::consts::PI / 180.0 * angle;
        Box::new(RotateY {
            object,
            angle,
            sin_theta: radians.sin(),
            cos_theta: radians.cos(),
        })
    }
}

impl Hitable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let origin = Vec3::new(
            self.cos_theta * r.origin.x - self.sin_theta * r.origin.z,
            r.origin.y,
            self.sin_theta * r.origin.x - self.cos_theta * r.origin.z,
        );
        let direction = Vec3::new(
            self.cos_theta * r.direction.x - self.sin_theta * r.direction.z,
            r.direction.y,
            self.sin_theta * r.direction.x - self.cos_theta * r.direction.z,
        );
        let rotated_r = Ray::new(origin, direction, r.time);
        if self.object.hit(&rotated_r, t_min, t_max, rec) {
            let p = Vec3::new(
                self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
                rec.p.y,
                self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
            );
            let normal = Vec3::new(
                self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
                rec.normal.y,
                self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
            );
            rec.p = p;
            rec.normal = normal;
            return true;
        }

        false
    }
}
