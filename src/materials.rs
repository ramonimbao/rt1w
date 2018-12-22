use std::sync::Arc;

use serde_json::Value;

use crate::materials::{lambertian::Lambertian, metal::Metal};
use crate::textures::{
    checkered_texture::CheckeredTexture, constant_texture::ConstantTexture,
    image_texture::ImageTexture, noise_texture::NoiseTexture, Texture, TextureType,
};
use crate::util::{hitable::HitRecord, json, ray::Ray, vector3::Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn emit(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

pub enum MaterialType {
    Lambertian,
    Metal(f64),
}

// Yay, got rid of the repeating code in lambertian.rs and metal.rs, but I don't know how
// redundant things will be now that I have an enum for MaterialType. I'll still look around
// if I can use it anywhere else.
pub fn create_material(
    values: &Value,
    texture_type: &TextureType,
    material_type: MaterialType,
) -> Arc<Material + Sync + Send> {
    match *texture_type {
        TextureType::Checkered => {
            let or = json::get_f64_or_rand(&values["material"]["colors"][0]["r"]);
            let og = json::get_f64_or_rand(&values["material"]["colors"][0]["g"]);
            let ob = json::get_f64_or_rand(&values["material"]["colors"][0]["b"]);
            let er = json::get_f64_or_rand(&values["material"]["colors"][1]["r"]);
            let eg = json::get_f64_or_rand(&values["material"]["colors"][1]["g"]);
            let eb = json::get_f64_or_rand(&values["material"]["colors"][1]["b"]);
            let (or, og, ob, er, eg, eb) = match (or, og, ob, er, eg, eb) {
                (Some(or), Some(og), Some(ob), Some(er), Some(eg), Some(eb)) => {
                    (or, og, ob, er, eg, eb)
                }
                (_, _, _, _, _, _) => (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            };

            let scale = json::get_f64_or_rand(&values["material"]["scale"]);
            let scale = match scale {
                Some(s) => s,
                _ => 1.0,
            };

            match material_type {
                MaterialType::Lambertian => Lambertian::new(CheckeredTexture::new(
                    ConstantTexture::new(Vec3::new(or, og, ob)),
                    ConstantTexture::new(Vec3::new(er, eg, eb)),
                    scale,
                )),
                MaterialType::Metal(fuzz) => Metal::new(
                    CheckeredTexture::new(
                        ConstantTexture::new(Vec3::new(or, og, ob)),
                        ConstantTexture::new(Vec3::new(er, eg, eb)),
                        scale,
                    ),
                    fuzz,
                ),
            }
        }

        TextureType::Constant => {
            let r = json::get_f64_or_rand(&values["material"]["color"]["r"]);
            let g = json::get_f64_or_rand(&values["material"]["color"]["g"]);
            let b = json::get_f64_or_rand(&values["material"]["color"]["b"]);
            let (r, g, b) = match (r, g, b) {
                (Some(r), Some(g), Some(b)) => (r, g, b),
                (_, _, _) => (0.0, 0.0, 0.0),
            };

            match material_type {
                MaterialType::Lambertian => {
                    Lambertian::new(ConstantTexture::new(Vec3::new(r, g, b)))
                }
                MaterialType::Metal(fuzz) => {
                    Metal::new(ConstantTexture::new(Vec3::new(r, g, b)), fuzz)
                }
            }
        }

        TextureType::Image => {
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

            let scale = json::get_f64_or_rand(&values["material"]["scale"]);
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

            match material_type {
                MaterialType::Lambertian => Lambertian::new(ImageTexture::new(&image_file, scale)),
                MaterialType::Metal(fuzz) => {
                    Metal::new(ImageTexture::new(&image_file, scale), fuzz)
                }
            }
        }

        TextureType::Noise => {
            let scale = json::get_f64_or_rand(&values["material"]["scale"]);
            let scale = match scale {
                Some(s) => s,
                _ => 1.0,
            };

            match material_type {
                MaterialType::Lambertian => Lambertian::new(NoiseTexture::new(scale)),
                MaterialType::Metal(fuzz) => Metal::new(NoiseTexture::new(scale), fuzz),
            }
        }
    }
}

pub mod blank;
pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;
