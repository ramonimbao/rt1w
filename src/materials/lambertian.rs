use std::sync::Arc;

use image;
use serde_json::Value;

use crate::materials::Material;
use crate::textures::{
    checkered_texture::CheckeredTexture, constant_texture::ConstantTexture,
    image_texture::ImageTexture, noise_texture::NoiseTexture, Texture, TextureType,
};
use crate::util::{hitable::HitRecord, json, math, ray::Ray, vector3::Vec3};

pub struct Lambertian {
    albedo: Arc<Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<Texture>) -> Arc<Lambertian> {
        Arc::new(Lambertian { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p + rec.normal + math::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p, r_in.time);
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}

pub fn load_from_json(values: &Value, texture_type: &TextureType) -> Arc<Material> {
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

            Lambertian::new(CheckeredTexture::new(
                ConstantTexture::new(Vec3::new(or, og, ob)),
                ConstantTexture::new(Vec3::new(er, eg, eb)),
                scale,
            ))
        }

        TextureType::Constant => {
            let r = json::get_f64_or_rand(&values["material"]["color"]["r"]);
            let g = json::get_f64_or_rand(&values["material"]["color"]["g"]);
            let b = json::get_f64_or_rand(&values["material"]["color"]["b"]);
            let (r, g, b) = match (r, g, b) {
                (Some(r), Some(g), Some(b)) => (r, g, b),
                (_, _, _) => (0.0, 0.0, 0.0),
            };
            Lambertian::new(ConstantTexture::new(Vec3::new(r, g, b)))
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

            Lambertian::new(ImageTexture::new(&image_file, scale))
        }

        TextureType::Noise => {
            let scale = json::get_f64_or_rand(&values["material"]["scale"]);
            let scale = match scale {
                Some(s) => s,
                _ => 1.0,
            };

            Lambertian::new(NoiseTexture::new(scale))
        }
    }
}
