use std::rc::Rc;

use image::DynamicImage;

use crate::textures::Texture;
use crate::util::vector3::Vec3;

pub struct ImageTexture {
    pixels: Vec<Vec3>,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new(image: DynamicImage) -> Rc<ImageTexture> {
        let image = image.to_rgb();
        let (width, height) = image.dimensions();
        let pixels: Vec<Vec3> = image
            .pixels()
            .map(|rgb| {
                Vec3::new(
                    rgb[0] as f64 / 255.0,
                    rgb[1] as f64 / 255.0,
                    rgb[2] as f64 / 255.0,
                )
            })
            .collect();

        Rc::new(ImageTexture {
            pixels,
            width,
            height,
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let mut i = (u * self.width as f64) as i64;
        let mut j = ((1.0 - v) * self.height as f64) as i64;

        if i < 0 {
            i = 0;
        }
        if j < 0 {
            j = 0;
        }
        if i > self.width as i64 - 1 {
            i = self.width as i64 - 1;
        }
        if j > self.height as i64 - 1 {
            j = self.height as i64 - 1;
        }
        let i = i as usize;
        let j = j as usize;

        let r = self.pixels[i + self.width as usize * j].x as f64;
        let g = self.pixels[i + self.width as usize * j].y as f64;
        let b = self.pixels[i + self.width as usize * j].z as f64;

        Vec3::new(r, g, b)
    }
}
