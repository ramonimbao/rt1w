use std::rc::Rc;

use crate::textures::Texture;
use crate::util::vector3::Vec3;

pub struct CheckeredTexture {
    odd: Rc<Texture>,
    even: Rc<Texture>,
}

impl CheckeredTexture {
    pub fn new(odd: Rc<Texture>, even: Rc<Texture>) -> CheckeredTexture {
        CheckeredTexture { odd, even }
    }
}

impl Texture for CheckeredTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let scale = 10.0;
        let sines = (scale * p.x).sin() * (scale * p.y).sin() * (scale * p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}
