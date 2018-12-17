use std::rc::Rc;

use serde_json::Value;

use crate::materials::Material;
use crate::textures::{
    checkered_texture::CheckeredTexture, constant_texture::ConstantTexture,
    image_texture::ImageTexture, noise_texture::NoiseTexture, Texture, TextureType,
};
use crate::util::{hitable::HitRecord, math, ray::Ray, vector3::Vec3};

pub struct Metal {
    albedo: Rc<Texture>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Rc<Texture>, fuzz: f64) -> Rc<Metal> {
        Rc::new(Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        })
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = math::reflect(
            &math::unit_vector(&r_in.direction),
            &math::unit_vector(&rec.normal),
        );
        *scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * math::random_in_unit_sphere(),
            r_in.time,
        );
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        math::dot(&scattered.direction, &rec.normal) > 0.0
    }
}

// The code here and in the lambert repeat. Maybe there's a way to generalize it?
pub fn load_from_json(values: &Value, texture_type: TextureType) -> Rc<Material> {
    let fuzz = values["material"]["fuzz"].as_f64();
    let fuzz = match fuzz {
        Some(f) => f,
        _ => 0.0,
    };

    let result = match texture_type {
        TextureType::Checkered => {
            let or = values["material"]["colors"][0]["r"].as_f64();
            let og = values["material"]["colors"][0]["g"].as_f64();
            let ob = values["material"]["colors"][0]["b"].as_f64();
            let er = values["material"]["colors"][1]["r"].as_f64();
            let eg = values["material"]["colors"][1]["g"].as_f64();
            let eb = values["material"]["colors"][1]["b"].as_f64();
            let (or, og, ob, er, eg, eb) = match (or, og, ob, er, eg, eb) {
                (Some(or), Some(og), Some(ob), Some(er), Some(eg), Some(eb)) => {
                    (or, og, ob, er, eg, eb)
                }
                (_, _, _, _, _, _) => (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            };

            let scale = values["material"]["scale"].as_f64();
            let scale = match scale {
                Some(s) => s,
                _ => 1.0,
            };

            Metal::new(
                CheckeredTexture::new(
                    ConstantTexture::new(Vec3::new(or, og, ob)),
                    ConstantTexture::new(Vec3::new(er, eg, eb)),
                    scale,
                ),
                fuzz,
            )
        }

        TextureType::Constant => {
            let r = values["material"]["color"]["r"].as_f64();
            let g = values["material"]["color"]["g"].as_f64();
            let b = values["material"]["color"]["b"].as_f64();
            let (r, g, b) = match (r, g, b) {
                (Some(r), Some(g), Some(b)) => (r, g, b),
                (_, _, _) => (0.0, 0.0, 0.0),
            };
            Metal::new(ConstantTexture::new(Vec3::new(r, g, b)), fuzz)
        }

        TextureType::Image => {
            let filename = values["material"]["filename"].as_str();
            let filename = match filename {
                Some(filename) => filename,
                _ => {
                    return Metal::new(
                        CheckeredTexture::new(
                            ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0)),
                            ConstantTexture::new(Vec3::new(1.0, 0.0, 1.0)),
                            10.0,
                        ),
                        fuzz,
                    );
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
                    return Metal::new(
                        CheckeredTexture::new(
                            ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0)),
                            ConstantTexture::new(Vec3::new(1.0, 0.0, 1.0)),
                            10.0,
                        ),
                        fuzz,
                    );
                }
            };

            Metal::new(ImageTexture::new(image_file, scale), fuzz)
        }

        TextureType::Noise => {
            let scale = values["material"]["scale"].as_f64();
            let scale = match scale {
                Some(s) => s,
                _ => 1.0,
            };

            Metal::new(NoiseTexture::new(scale), fuzz)
        }
    };

    result
}
