use crate::util::{
    hitable::{HitRecord, Hitable},
    ray::Ray,
    vector3::Vec3,
};

pub struct Rotate {
    object: Box<Hitable>,
}

impl Rotate {
    pub fn new(object: Box<Hitable>, angles: Vec3) -> Box<Hitable> {
        Box::new(Rotate {
            object: RotateZ::new(
                RotateY::new(RotateX::new(object, angles.x), angles.y),
                angles.z,
            ),
        })
    }
}

impl Hitable for Rotate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.object.hit(r, t_min, t_max, rec)
    }
}

struct RotateY {
    object: Box<Hitable>,
    sin_theta: f64,
    cos_theta: f64,
}

impl RotateY {
    pub fn new(object: Box<Hitable>, angle: f64) -> Box<Hitable> {
        let radians = std::f64::consts::PI / 180.0 * angle;
        Box::new(RotateY {
            object,
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
            self.sin_theta * r.origin.x + self.cos_theta * r.origin.z,
        );
        let direction = Vec3::new(
            self.cos_theta * r.direction.x - self.sin_theta * r.direction.z,
            r.direction.y,
            self.sin_theta * r.direction.x + self.cos_theta * r.direction.z,
        );
        let rotated_r = Ray::new(origin, direction, r.time);
        if self.object.hit(&rotated_r, t_min, t_max, rec) {
            let p = Vec3::new(
                self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
                rec.p.y,
                -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
            );
            let normal = Vec3::new(
                self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
                rec.normal.y,
                -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
            );
            rec.p = p;
            rec.normal = normal;
            return true;
        }

        false
    }
}

struct RotateX {
    object: Box<Hitable>,
    sin_theta: f64,
    cos_theta: f64,
}

impl RotateX {
    pub fn new(object: Box<Hitable>, angle: f64) -> Box<Hitable> {
        let radians = std::f64::consts::PI / 180.0 * angle;
        Box::new(RotateX {
            object,
            sin_theta: radians.sin(),
            cos_theta: radians.cos(),
        })
    }
}

impl Hitable for RotateX {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let origin = Vec3::new(
            r.origin.x,
            self.cos_theta * r.origin.y - self.sin_theta * r.origin.z,
            self.sin_theta * r.origin.y + self.cos_theta * r.origin.z,
        );
        let direction = Vec3::new(
            r.direction.x,
            self.cos_theta * r.direction.y - self.sin_theta * r.direction.z,
            self.sin_theta * r.direction.y + self.cos_theta * r.direction.z,
        );
        let rotated_r = Ray::new(origin, direction, r.time);
        if self.object.hit(&rotated_r, t_min, t_max, rec) {
            let p = Vec3::new(
                rec.p.x,
                self.cos_theta * rec.p.y + self.sin_theta * rec.p.z,
                -self.sin_theta * rec.p.y + self.cos_theta * rec.p.z,
            );
            let normal = Vec3::new(
                rec.normal.x,
                self.cos_theta * rec.normal.y + self.sin_theta * rec.normal.z,
                -self.sin_theta * rec.normal.y + self.cos_theta * rec.normal.z,
            );
            rec.p = p;
            rec.normal = normal;
            return true;
        }
        false
    }
}

struct RotateZ {
    object: Box<Hitable>,
    sin_theta: f64,
    cos_theta: f64,
}

impl RotateZ {
    pub fn new(object: Box<Hitable>, angle: f64) -> Box<Hitable> {
        let radians = std::f64::consts::PI / 180.0 * angle;
        Box::new(RotateZ {
            object,
            sin_theta: radians.sin(),
            cos_theta: radians.cos(),
        })
    }
}

impl Hitable for RotateZ {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let origin = Vec3::new(
            self.cos_theta * r.origin.x - self.sin_theta * r.origin.y,
            self.sin_theta * r.origin.x + self.cos_theta * r.origin.y,
            r.origin.z,
        );
        let direction = Vec3::new(
            self.cos_theta * r.direction.x - self.sin_theta * r.direction.y,
            self.sin_theta * r.direction.x + self.cos_theta * r.direction.y,
            r.direction.z,
        );
        let rotated_r = Ray::new(origin, direction, r.time);
        if self.object.hit(&rotated_r, t_min, t_max, rec) {
            let p = Vec3::new(
                self.cos_theta * rec.p.x + self.sin_theta * rec.p.y,
                -self.sin_theta * rec.p.x + self.cos_theta * rec.p.y,
                rec.p.z,
            );
            let normal = Vec3::new(
                self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.y,
                -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.y,
                rec.normal.z,
            );
            rec.p = p;
            rec.normal = normal;
            return true;
        }

        false
    }
}
