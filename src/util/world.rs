use std::rc::Rc;

use rand::Rng;

use crate::materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::shapes::{plane::Plane, sphere::Sphere};
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
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
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

pub fn five_spheres() -> HitableList {
    HitableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Rc::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Rc::new(Dielectric::new(1.5)),
        )),
    ])
}

fn skybox() -> Vec<Box<Hitable>> {
    let min = std::f64::MIN;
    let max = std::f64::MAX;
    let list: Vec<Box<Hitable>> = vec![
        // Y planes
        Box::new(Plane::new(
            Vec3::new(0.0, min, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Rc::new(Metal::new(Vec3::unit(), 0.0)),
        )),
        Box::new(Plane::new(
            Vec3::new(0.0, max, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Rc::new(Metal::new(Vec3::unit(), 0.0)),
        )),
        // Z planes
        Box::new(Plane::new(
            Vec3::new(0.0, 0.0, max),
            Vec3::new(0.0, 0.0, 1.0),
            Rc::new(Metal::new(Vec3::unit(), 0.0)),
        )),
        Box::new(Plane::new(
            Vec3::new(0.0, 0.0, min),
            Vec3::new(0.0, 0.0, -1.0),
            Rc::new(Metal::new(Vec3::unit(), 0.0)),
        )),
        // X planes
        Box::new(Plane::new(
            Vec3::new(max, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Rc::new(Metal::new(Vec3::unit(), 0.0)),
        )),
        Box::new(Plane::new(
            Vec3::new(min, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Rc::new(Metal::new(Vec3::unit(), 0.0)),
        )),
    ];

    list
}

pub fn random_scene() -> HitableList {
    let mut list: Vec<Box<Hitable>> = Vec::new();

    list.append(&mut skybox());

    list.push(Box::new(Plane::new(
        Vec3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.8))),
        //Rc::new(Metal::new(Vec3::new(0.5, 0.5, 0.5), 0.05)),
        //Rc::new(Dielectric::new(1.5)),
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 2.0, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(Vec3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))),
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

    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    let list = HitableList::new(list);
    list
}
