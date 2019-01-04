use rand::Rng;

use crate::defaults;
use crate::util::{math, ray::Ray, vector3::Vec3};

#[allow(dead_code)]
struct WUV {
    w: Vec3,
    u: Vec3,
    v: Vec3,
}

pub struct Look {
    pub from: Vec3,
    pub to: Vec3,
}

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f64,
    wuv: WUV,
    t: (f64, f64),
}

impl Camera {
    pub fn new(
        look: &Look,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
        t: (f64, f64),
    ) -> Camera {
        let w = math::unit_vector(&(look.from - look.to));
        let u = math::unit_vector(&math::cross(&vup, &w));
        let v = math::cross(&w, &u);
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        Camera {
            lower_left_corner: look.from
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: look.from,
            lens_radius: aperture / 2.0,
            wuv: WUV { w, u, v },
            t,
        }
    }

    pub fn default() -> Camera {
        Camera::new(
            &Look {
                from: Vec3::new(
                    defaults::LOOK_FROM.0,
                    defaults::LOOK_FROM.1,
                    defaults::LOOK_FROM.2,
                ),
                to: Vec3::new(
                    defaults::LOOK_TO.0,
                    defaults::LOOK_TO.1,
                    defaults::LOOK_TO.2,
                ),
            },
            Vec3::new(
                defaults::VERTICAL.0,
                defaults::VERTICAL.1,
                defaults::VERTICAL.2,
            ), // Vertical
            defaults::FOV,                                            // Field of view
            f64::from(defaults::WIDTH) / f64::from(defaults::HEIGHT), // Aspect ratio
            defaults::APERTURE,                                       // Aperture
            defaults::FOCUS_DISTANCE,                                 // Focus distance
            (defaults::T0, defaults::T1),                             // t_0 for movement
                                                                      // t_1 for movement
        )
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = math::random_in_unit_disk() * self.lens_radius;
        let offset = self.wuv.u * rd.x + self.wuv.v * rd.y;
        let mut rng = rand::thread_rng();
        let time = self.t.0 + rng.gen::<f64>() * (self.t.1 - self.t.0);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        )
    }
}
