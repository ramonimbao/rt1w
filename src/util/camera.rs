use std::f64::consts::PI;

use rand::Rng;

use crate::defaults;
use crate::util::{math, ray::Ray, vector3::Vec3};

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f64,
    w: Vec3,
    u: Vec3,
    v: Vec3,
    t0: f64,
    t1: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
        t0: f64,
        t1: f64,
    ) -> Camera {
        let _w = math::unit_vector(&(look_from - look_at));
        let _u = math::unit_vector(&math::cross(&vup, &_w));
        let _v = math::cross(&_w, &_u);
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        Camera {
            lower_left_corner: look_from
                - half_width * focus_dist * _u
                - half_height * focus_dist * _v
                - focus_dist * _w,
            horizontal: 2.0 * half_width * focus_dist * _u,
            vertical: 2.0 * half_height * focus_dist * _v,
            origin: look_from,
            lens_radius: aperture / 2.0,
            w: _w,
            u: _u,
            v: _v,
            t0,
            t1,
        }
    }

    pub fn default() -> Camera {
        Camera::new(
            Vec3::new(
                defaults::LOOK_FROM.0,
                defaults::LOOK_FROM.1,
                defaults::LOOK_FROM.2,
            ), // Look from
            Vec3::new(
                defaults::LOOK_TO.0,
                defaults::LOOK_TO.1,
                defaults::LOOK_TO.2,
            ), // Look at
            Vec3::new(
                defaults::VERTICAL.0,
                defaults::VERTICAL.1,
                defaults::VERTICAL.2,
            ), // Vertical
            defaults::FOV,                                            // Field of view
            f64::from(defaults::WIDTH) / f64::from(defaults::HEIGHT), // Aspect ratio
            defaults::APERTURE,                                       // Aperture
            defaults::FOCUS_DISTANCE,                                 // Focus distance
            defaults::T0,                                             // t_0 for movement
            defaults::T1,                                             // t_1 for movement
        )
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = math::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        let mut rng = rand::thread_rng();
        let time = self.t0 + rng.gen::<f64>() * (self.t1 - self.t0);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        )
    }
}
