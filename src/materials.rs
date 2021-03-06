use std::sync::Arc;

use serde_json::Value;

pub mod blank;
pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub use crate::materials::blank::Blank;
pub use crate::materials::dielectric::Dielectric;
pub use crate::materials::diffuse_light::DiffuseLight;
pub use crate::materials::isotropic::Isotropic;
pub use crate::materials::lambertian::Lambertian;
pub use crate::materials::metal::Metal;

use crate::textures::{CheckeredTexture, ConstantTexture, ImageTexture, NoiseTexture, TextureType};
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
    Isotropic,
    Metal(f64),
}

// Yay, got rid of the repeating code in lambertian.rs and metal.rs, but I don't know how
// redundant things will be now that I have an enum for MaterialType. I'll still look around
// if I can use it anywhere else.
pub fn create_material(
    values: &Value,
    texture_type: TextureType,
    material_type: MaterialType,
) -> Arc<Material + Sync + Send> {
    match texture_type {
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

            let texture = CheckeredTexture::create(
                ConstantTexture::create(Vec3::new(or, og, ob)),
                ConstantTexture::create(Vec3::new(er, eg, eb)),
                scale,
            );
            match material_type {
                MaterialType::Lambertian => Lambertian::create(texture),
                MaterialType::Metal(fuzz) => Metal::create(texture, fuzz),
                MaterialType::Isotropic => Isotropic::create(texture),
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

            let texture = ConstantTexture::create(Vec3::new(r, g, b));
            match material_type {
                MaterialType::Lambertian => Lambertian::create(texture),
                MaterialType::Metal(fuzz) => Metal::create(texture, fuzz),
                MaterialType::Isotropic => Isotropic::create(texture),
            }
        }

        TextureType::Image => {
            let filename = values["material"]["filename"].as_str();
            let filename = match filename {
                Some(filename) => filename,
                _ => {
                    return Lambertian::create(CheckeredTexture::create(
                        ConstantTexture::create(Vec3::new(0.0, 0.0, 0.0)),
                        ConstantTexture::create(Vec3::new(1.0, 0.0, 1.0)),
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
                    return Lambertian::create(CheckeredTexture::create(
                        ConstantTexture::create(Vec3::new(0.0, 0.0, 0.0)),
                        ConstantTexture::create(Vec3::new(1.0, 0.0, 1.0)),
                        10.0,
                    ));
                }
            };

            let texture = ImageTexture::create(&image_file, scale);
            match material_type {
                MaterialType::Lambertian => Lambertian::create(texture),
                MaterialType::Metal(fuzz) => Metal::create(texture, fuzz),
                MaterialType::Isotropic => Isotropic::create(texture),
            }
        }

        TextureType::Noise => {
            let scale = json::get_f64_or_rand(&values["material"]["scale"]);
            let scale = match scale {
                Some(s) => s,
                _ => 1.0,
            };

            let texture = NoiseTexture::create(scale);
            match material_type {
                MaterialType::Lambertian => Lambertian::create(texture),
                MaterialType::Metal(fuzz) => Metal::create(texture, fuzz),
                MaterialType::Isotropic => Isotropic::create(texture),
            }
        }
    }
}
