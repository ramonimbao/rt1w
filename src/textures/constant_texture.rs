use std::sync::Arc;

use crate::textures::Texture;
use crate::util::vector3::Vec3;

pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    pub fn create(color: Vec3) -> Arc<ConstantTexture> {
        Arc::new(ConstantTexture { color })
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _: f64, _: f64, _: Vec3) -> Vec3 {
        self.color
    }
}
