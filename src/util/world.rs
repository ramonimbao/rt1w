use std::fs;
use std::rc::Rc;

use rand::Rng;
use serde_json::Value;

use crate::materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::shapes::{
    moving_sphere::{self, MovingSphere},
    plane::{self, Plane},
    sphere::{self, Sphere},
};
use crate::textures::{checkered_texture::CheckeredTexture, constant_texture::ConstantTexture};
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
        if depth < 50
            && rec
                .clone()
                .material
                .scatter(&r, &mut rec, &mut attenuation, &mut scattered)
        {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            return Vec3::zero();
        }
    } else {
        let unit_direction = math::unit_vector(&r.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::unit() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn skybox() -> Vec<Box<Hitable>> {
    let min = std::f64::MIN;
    let max = std::f64::MAX;
    //let mat = Rc::new(Lambertian::new(Vec3::unit()));
    let mat = Rc::new(Dielectric::new(1.0));
    let list: Vec<Box<Hitable>> = vec![
        // Y planes
        Box::new(Plane::new(
            Vec3::new(0.0, min, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            //Rc::new(Metal::new(Vec3::unit(), 0.0)),
            mat.clone(),
        )),
        Box::new(Plane::new(
            Vec3::new(0.0, max, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            //Rc::new(Metal::new(Vec3::unit(), 0.0)),
            mat.clone(),
        )),
        // Z planes
        Box::new(Plane::new(
            Vec3::new(0.0, 0.0, max),
            Vec3::new(0.0, 0.0, -1.0),
            //Rc::new(Metal::new(Vec3::unit(), 0.0)),
            mat.clone(),
        )),
        Box::new(Plane::new(
            Vec3::new(0.0, 0.0, min),
            Vec3::new(0.0, 0.0, 1.0),
            //Rc::new(Metal::new(Vec3::unit(), 0.0)),
            mat.clone(),
        )),
        // X planes
        Box::new(Plane::new(
            Vec3::new(max, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            //Rc::new(Metal::new(Vec3::unit(), 0.0)),
            mat.clone(),
        )),
        Box::new(Plane::new(
            Vec3::new(min, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            //Rc::new(Metal::new(Vec3::unit(), 0.0)),
            mat.clone(),
        )),
    ];

    list
}

pub fn random_scene() -> HitableList {
    let mut list: Vec<Box<Hitable>> = Vec::new();

    //list.append(&mut skybox());

    list.push(Box::new(Plane::new(
        Vec3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
            0.8, 0.8, 0.8,
        ))))),
        //Rc::new(Metal::new(Vec3::new(0.5, 0.5, 0.5), 0.05)),
        //Rc::new(Dielectric::new(1.5)),
    )));

    let mut rng = rand::thread_rng();
    /*
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 2.0, 0.0)).length() > 0.9 {
                if choose_mat < 0.4 {
                    // Moving sphere
                    list.push(Box::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, 0.5 * rng.gen::<f64>(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))))),
                    )));
                } else if choose_mat < 0.8 {
                    // Diffuse

                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))))),
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(
                            Vec3::new(
                                (1.0 + rng.gen::<f64>()) * 0.5,
                                (1.0 + rng.gen::<f64>()) * 0.5,
                                (1.0 + rng.gen::<f64>()) * 0.5,
                            ),
                            0.5 * rng.gen::<f64>(),
                        )),
                    )));
                } else {
                    // Glass
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(2.4)),
                    )));
                }
            }
        }
    }
    */

    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Rc::new(CheckeredTexture::new(
            Rc::new(ConstantTexture::new(Vec3::new(0.05, 0.05, 0.05))),
            Rc::new(ConstantTexture::new(Vec3::new(0.95, 0.05, 0.95))),
        )))),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

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

    println!("Load all objects to scene...");
    let mut list: Vec<Box<Hitable>> = Vec::new();
    list.append(&mut sphere::load_from_json(&values));
    list.append(&mut plane::load_from_json(&values));
    println!("Done loading.");

    let list = HitableList::new(list);
    list
}
