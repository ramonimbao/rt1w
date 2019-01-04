use std::sync::Arc;

use image::DynamicImage;

use crate::textures::Texture;
use crate::util::vector3::Vec3;

pub struct ImageTexture {
    pixels: Vec<Vec3>,
    width: u32,
    height: u32,
    scale: f64,
}

impl ImageTexture {
    pub fn create(image: &DynamicImage, scale: f64) -> Arc<ImageTexture> {
        let image = image.to_rgb();
        let (width, height) = image.dimensions();
        let pixels: Vec<Vec3> = image
            .pixels()
            .map(|rgb| {
                Vec3::new(
                    f64::from(rgb[0]) / 255.0,
                    f64::from(rgb[1]) / 255.0,
                    f64::from(rgb[2]) / 255.0,
                )
            })
            .collect();

        Arc::new(ImageTexture {
            pixels,
            width,
            height,
            scale,
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: Vec3) -> Vec3 {
        let i = ((u * f64::from(self.width) / self.scale) as u32 % self.width) as usize;
        let j = ((v * f64::from(self.height) / self.scale) as u32 % self.height) as usize;

        let red = self.pixels[i + self.width as usize * j].x;
        let green = self.pixels[i + self.width as usize * j].y;
        let blue = self.pixels[i + self.width as usize * j].z;

        Vec3::new(red, green, blue)
    }
}
