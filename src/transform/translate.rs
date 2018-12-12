use crate::util::{
    hitable::{HitRecord, Hitable},
    ray::Ray,
    vector3::Vec3,
};

pub struct Translate {
    object: Box<Hitable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(object: Box<Hitable>, offset: Vec3) -> Box<Hitable> {
        Box::new(Translate { object, offset })
    }
}

impl Hitable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        if self.object.hit(&moved_r, t_min, t_max, rec) {
            rec.p += self.offset;
            return true;
        }

        false
    }
}
