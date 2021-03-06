use std::sync::Arc;

use crate::textures::Texture;
use crate::util::{perlin::Perlin, vector3::Vec3};

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn create(scale: f64) -> Arc<NoiseTexture> {
        Arc::new(NoiseTexture {
            noise: Perlin::new(),
            scale,
        })
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: Vec3) -> Vec3 {
        //Vec3::unit() * self.noise.noise(self.scale * p)
        Vec3::unit() * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turbulence(p, 7)).sin())
    }
}
