use crate::util::{
    hitable::{HitRecord, Hitable},
    ray::Ray,
};

pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<Hitable>>) -> HitableList {
        HitableList { list }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far: f64 = t_max;
        for i in 0..self.list.len() {
            if self.list[i].hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = HitRecord {
                    t: temp_rec.t,
                    u: temp_rec.u,
                    v: temp_rec.v,
                    p: temp_rec.p,
                    normal: temp_rec.normal,
                    material: temp_rec.material.clone(),
                };
            }
        }

        hit_anything
    }
}
