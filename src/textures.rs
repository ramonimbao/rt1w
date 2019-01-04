use crate::util::vector3::Vec3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

// Used for loading into scene via JSON
pub enum TextureType {
    Checkered,
    Constant,
    Image,
    Noise,
}

pub mod checkered_texture;
pub mod constant_texture;
pub mod image_texture;
pub mod noise_texture;

pub use crate::textures::checkered_texture::CheckeredTexture;
pub use crate::textures::constant_texture::ConstantTexture;
pub use crate::textures::image_texture::ImageTexture;
pub use crate::textures::noise_texture::NoiseTexture;
