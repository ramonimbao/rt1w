use std::rc::Rc;

use image::DynamicImage;
use serde_json::Value;

use crate::materials::{lambertian::Lambertian, Material};
use crate::textures::{
    checkered_texture::CheckeredTexture, constant_texture::ConstantTexture, Texture,
};
use crate::util::vector3::Vec3;

pub struct ImageTexture {
    pixels: Vec<Vec3>,
    width: u32,
    height: u32,
    scale: f64,
}

impl ImageTexture {
    pub fn new(image: DynamicImage, scale: f64) -> Rc<ImageTexture> {
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
            scale,
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: Vec3) -> Vec3 {
        let i = ((u * self.width as f64 / self.scale) as u32 % self.width) as usize;
        let j = ((v * self.height as f64 / self.scale) as u32 % self.height) as usize;

        let r = self.pixels[i + self.width as usize * j].x as f64;
        let g = self.pixels[i + self.width as usize * j].y as f64;
        let b = self.pixels[i + self.width as usize * j].z as f64;

        Vec3::new(r, g, b)
    }
}

pub fn load_from_json(values: &Value) -> Rc<Material> {
    let filename = values["material"]["filename"].as_str();
    let filename = match filename {
        Some(filename) => filename,
        _ => {
            return Lambertian::new(CheckeredTexture::new(
                ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0)),
                ConstantTexture::new(Vec3::new(1.0, 0.0, 1.0)),
                10.0,
            ));
        }
    };

    let scale = values["material"]["scale"].as_f64();
    let scale = match scale {
        Some(s) => s,
        _ => 1.0,
    };

    let image_file = image::open(filename);
    let image_file = match image_file {
        Ok(image_file) => image_file,
        Err(e) => {
            eprintln!("ERROR [{}]: {}", filename, e);
            return Lambertian::new(CheckeredTexture::new(
                ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0)),
                ConstantTexture::new(Vec3::new(1.0, 0.0, 1.0)),
                10.0,
            ));
        }
    };

    Lambertian::new(ImageTexture::new(image_file, scale))
}
