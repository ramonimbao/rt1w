use crate::util::vector3::Vec3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub mod checkered_texture;
pub mod constant_texture;
pub mod image_texture;
pub mod noise_texture;
