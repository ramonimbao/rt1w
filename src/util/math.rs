use crate::util::vector3::Vec3;
use rand::Rng;

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p =
        Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0 - Vec3::unit();
    while p.squared_length() >= 1.0 {
        p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0 - Vec3::unit();
    }
    p
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    while dot(&p, &p) >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    }
    p
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * dot(v, n) * 2.0
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
    let uv = unit_vector(v);
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = (uv - (*n * dt)) * ni_over_nt - (*n * discriminant.sqrt());
        true
    } else {
        false
    }
}

pub fn schlik(cosine: f64, refractive_index: f64) -> f64 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub fn get_sphere_uv(p: &Vec3, u: &mut f64, v: &mut f64) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let pi = std::f64::consts::PI;
    *u = 1.0 - (phi + pi) / (pi * 2.0);
    *v = -(theta + pi / 2.0) / pi;
}

pub fn get_plane_uv(p: &Vec3, n: &Vec3, u: &mut f64, v: &mut f64) {
    *v = (p.x * dot(n, &Vec3::new(0.0, 1.0, 0.0)))
        + (p.x * dot(n, &Vec3::new(0.0, 0.0, 1.0)))
        + (p.z * -dot(n, &Vec3::new(1.0, 0.0, 0.0)));
    *u = (p.z * dot(n, &Vec3::new(0.0, 1.0, 0.0)))
        + (p.y * dot(n, &Vec3::new(0.0, 0.0, 1.0)))
        + (p.y * dot(n, &Vec3::new(1.0, 0.0, 0.0)));
}
