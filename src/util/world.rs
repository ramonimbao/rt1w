use std::fs;
use std::rc::Rc;

use image;
use rand::Rng;
use serde_json::Value;

use crate::materials::{
    dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    Material,
};
use crate::shapes::{
    cuboid::{self, Cuboid},
    moving_sphere::{self, MovingSphere},
    plane::{self, Plane},
    sphere::{self, Sphere},
};
use crate::textures::{
    checkered_texture::CheckeredTexture, constant_texture::ConstantTexture,
    image_texture::ImageTexture, noise_texture::NoiseTexture,
};
use crate::transform::{rotate::Rotate, translate::Translate};
use crate::util::{
    hitable::{HitRecord, Hitable},
    hitable_list::HitableList,
    math,
    ray::Ray,
    vector3::Vec3,
};

pub fn color(r: &Ray, world: &mut HitableList, depth: usize) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, std::f64::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), r.time);
        let mut attenuation = Vec3::zero();
        let rec_clone = rec.clone();
        let emitted = rec_clone.material.emit(rec.u, rec.v, &rec.p);
        if depth < 50
            && rec_clone
                .material
                .scatter(&r, &mut rec, &mut attenuation, &mut scattered)
        {
            return attenuation * color(&scattered, world, depth + 1)
                + if crate::defaults::ENABLE_LIGHTS {
                    emitted
                } else {
                    Vec3::zero()
                };
        } else {
            return emitted;
        }
    } else {
        if crate::defaults::ENABLE_LIGHTS {
            Vec3::zero()
        } else {
            let unit_direction = math::unit_vector(&r.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::unit() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn load_skybox_from_json(values: &Value) -> Vec<Box<Hitable>> {
    let sr = values["skybox"]["r"].as_f64();
    let sg = values["skybox"]["g"].as_f64();
    let sb = values["skybox"]["b"].as_f64();
    let (sr, sg, sb) = match (sr, sg, sb) {
        (Some(r), Some(g), Some(b)) => (r, g, b),
        (_, _, _) => return Vec::new(),
    };

    skybox(Vec3::new(sr, sg, sb))
}

fn skybox(color: Vec3) -> Vec<Box<Hitable>> {
    let min = std::f64::MIN / 2.0;
    let max = std::f64::MAX / 2.0;
    let mat = DiffuseLight::new(ConstantTexture::new(color));
    let list: Vec<Box<Hitable>> = vec![
        // Y planes
        Plane::new(
            Vec3::new(0.0, min, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            mat.clone(),
        ),
        Plane::new(
            Vec3::new(0.0, max, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            mat.clone(),
        ),
        // Z planes
        Plane::new(
            Vec3::new(0.0, 0.0, max),
            Vec3::new(0.0, 0.0, -1.0),
            mat.clone(),
        ),
        Plane::new(
            Vec3::new(0.0, 0.0, min),
            Vec3::new(0.0, 0.0, 1.0),
            mat.clone(),
        ),
        // X planes
        Plane::new(
            Vec3::new(max, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            mat.clone(),
        ),
        Plane::new(
            Vec3::new(min, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            mat.clone(),
        ),
    ];

    list
}

fn choose_random_texture() -> Rc<Material> {
    let mut rng = rand::thread_rng();
    let choose_texture: f64 = rng.gen();
    if choose_texture < 0.25 {
        Lambertian::new(ConstantTexture::new(Vec3::new(
            rng.gen::<f64>() * rng.gen::<f64>(),
            rng.gen::<f64>() * rng.gen::<f64>(),
            rng.gen::<f64>() * rng.gen::<f64>(),
        )))
    } else if choose_texture < 0.5 {
        Lambertian::new(NoiseTexture::new(10.0 + 10.0 * rng.gen::<f64>()))
    } else if choose_texture < 0.75 {
        let choose_image = rng.gen_range(0, 6);
        Lambertian::new(ImageTexture::new(
            image::open(match choose_image {
                0 => "res/images/Blood Stone CH16.png",
                1 => "res/images/Lava Planet CH16.png",
                //2 => "res/images/Mars CH16.png", // This is too boring
                3 => "res/images/Mine Rocks CH16.png",
                4 => "res/images/Red rubble ch16.png",
                _ => "res/images/Snow Planet CH16.png",
            })
            .expect("Failed to open file."),
            1.0,
        ))
    } else {
        Lambertian::new(CheckeredTexture::new(
            ConstantTexture::new(Vec3::new(
                rng.gen::<f64>() * rng.gen::<f64>(),
                rng.gen::<f64>() * rng.gen::<f64>(),
                rng.gen::<f64>() * rng.gen::<f64>(),
            )),
            ConstantTexture::new(Vec3::new(
                rng.gen::<f64>() * rng.gen::<f64>(),
                rng.gen::<f64>() * rng.gen::<f64>(),
                rng.gen::<f64>() * rng.gen::<f64>(),
            )),
            rng.gen::<f64>() * 15.0 + 10.0,
        ))
    }
}

pub fn random_scene() -> HitableList {
    let mut list: Vec<Box<Hitable>> = Vec::new();
    let mut rng = rand::thread_rng();

    //list.append(&mut skybox(Vec3::new(0.0625, 0.0625, 0.0625)));
    list.append(&mut skybox(Vec3::new(1.0, 1.0, 1.0)));

    let choose_image = rng.gen_range(0, 6);
    list.push(Plane::new(
        Vec3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        //Lambertian::new(ConstantTexture::new(Vec3::new(0.8, 0.8, 0.8))),
        //Metal::new(Vec3::new(0.5, 0.5, 0.5), 0.05),
        //Dielectric::new(1.5),
        //Lambertian::new(NoiseTexture::new(10.0)),
        //DiffuseLight::new(ConstantTexture::new(Vec3::unit())),
        Lambertian::new(ImageTexture::new(
            image::open(match choose_image {
                0 => "res/images/Blood Stone CH16.png",
                1 => "res/images/Lava Planet CH16.png",
                //2 => "res/images/Mars CH16.png", // This is too boring
                3 => "res/images/Mine Rocks CH16.png",
                4 => "res/images/Red rubble ch16.png",
                _ => "res/images/Snow Planet CH16.png",
            })
            .expect("Failed to open file."),
            10.0,
        )),
    ));

    /*
    list.push(Sphere::new(
        Vec3::new(-1000.0, 3000.0, 0.0),
        1500.0,
        DiffuseLight::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0, 5);
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                3.0,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 2.0, 0.0)).length() > 0.9 {
                if choose_mat == 0 {
                    // Moving sphere
                    list.push(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, 0.5 * rng.gen::<f64>(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        choose_random_texture(),
                    ));
                } else if choose_mat == 1 {
                    // Diffuse
                    list.push(Sphere::new(center, 0.2, choose_random_texture()));
                } else if choose_mat == 2 {
                    // Metal
                    list.push(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            Vec3::new(
                                (1.0 + rng.gen::<f64>()) * 0.5,
                                (1.0 + rng.gen::<f64>()) * 0.5,
                                (1.0 + rng.gen::<f64>()) * 0.5,
                            ),
                            0.5 * rng.gen::<f64>(),
                        ),
                    ));
                } else if choose_mat == 3 {
                    // Glass
                    list.push(Sphere::new(center, 0.2, Dielectric::new(2.4)));
                } else {
                    // Light
                    list.push(Sphere::new(
                        center,
                        0.2,
                        DiffuseLight::new(ConstantTexture::new(Vec3::new(
                            0.5 * rng.gen::<f64>() + 0.5,
                            0.5 * rng.gen::<f64>() + 0.5,
                            0.5 * rng.gen::<f64>() + 0.5,
                        ))),
                    ))
                }
            }
        }
    }

    let choose_image = rng.gen_range(0, 6);
    list.push(Sphere::new(
        Vec3::new(0.0, 1.1, 0.0),
        1.0,
        //Dielectric::new(1.5),
        //Lambertian::new(NoiseTexture::new(10.0)),
        Lambertian::new(ImageTexture::new(
            image::open(match choose_image {
                0 => "res/images/Blood Stone CH16.png",
                1 => "res/images/Lava Planet CH16.png",
                2 => "res/images/Mars CH16.png",
                3 => "res/images/Mine Rocks CH16.png",
                4 => "res/images/Red rubble ch16.png",
                _ => "res/images/Snow Planet CH16.png",
            })
            .expect("Failed to open file."),
            1.0,
        )),
        //DiffuseLight::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    ));
    list.push(Sphere::new(
        Vec3::new(-4.0, 1.1, 0.0),
        1.0,
        Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
    ));
    list.push(Sphere::new(
        Vec3::new(4.0, 1.1, 0.0),
        1.0,
        //Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
        Dielectric::new(1.8),
    ));
    list.push(Sphere::new(
        Vec3::new(-3.9, 1.1, 0.0),
        1.0,
        //Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
        Dielectric::new(1.8),
    ));
    list.push(Sphere::new(
        Vec3::new(3.0, 3.0, -1.0),
        1.0,
        //Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
        DiffuseLight::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    ));
    list.push(Sphere::new(
        Vec3::new(-8.0, 1.1, 0.0),
        1.0,
        Lambertian::new(CheckeredTexture::new(
            ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0)),
            ConstantTexture::new(Vec3::new(0.95, 0.05, 0.95)),
            10.0,
        )),
    ));
    */

    let list = HitableList::new(list);
    list
}

pub fn load_from_json(filename: String) -> HitableList {
    println!("Loading scene data from {}", filename);

    let data = match fs::read_to_string(filename) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            println!("Generating random scene...");
            return random_scene();
        }
    };

    let values: Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            println!("Generating random scene...");
            return random_scene();
        }
    };

    println!("Loaded scene data.");

    println!("Loading all objects to scene...");
    let mut list: Vec<Box<Hitable>> = Vec::new();
    list.append(&mut sphere::load_from_json(&values));
    list.append(&mut moving_sphere::load_from_json(&values));
    list.append(&mut plane::load_from_json(&values));
    list.append(&mut cuboid::load_from_json(&values));
    list.append(&mut load_skybox_from_json(&values));
    println!("Done loading.");

    // Test cube
    list.push(Translate::new(
        Rotate::new(
            Cuboid::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(2.0, 2.0, 2.0),
                Dielectric::new(1.8),
                //Lambertian::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 1.0)))
            ),
            Vec3::new(15.0, 30.0, 45.0),
        ),
        Vec3::new(0.0, 2.0, 3.5),
    ));
    list.push(Translate::new(
        Rotate::new(
            Cuboid::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(2.0, 2.0, 2.0),
                Dielectric::new(1.8),
                //Lambertian::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 1.0)))
            ),
            Vec3::new(30.0, 45.0, 15.0),
        ),
        Vec3::new(3.0, 2.0, 3.5),
    ));
    list.push(Translate::new(
        Rotate::new(
            Cuboid::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(2.0, 2.0, 2.0),
                Dielectric::new(1.8),
                //Lambertian::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 1.0)))
            ),
            Vec3::new(45.0, 10.0, 30.0),
        ),
        Vec3::new(-3.0, 2.0, 3.5),
    ));

    let list = HitableList::new(list);
    list
}
