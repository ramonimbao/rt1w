use std::rc::Rc;

use serde_json::Value;

use crate::materials::{dielectric, diffuse_light, lambertian, metal, Material};
use crate::textures::TextureType;
use crate::transform::{rotate::Rotate, translate::Translate};
use crate::util::{
    hitable::{HitRecord, Hitable},
    json, math,
    ray::Ray,
    vector3::Vec3,
};

pub struct Triangle {
    vertices: Vec<Vec3>,
    normal: Vec3,
    material: Rc<Material>,
}

impl Triangle {
    pub fn new(vertices: Vec<Vec3>, material: Rc<Material>) -> Box<Triangle> {
        let normal = math::cross(&(vertices[1] - vertices[0]), &(vertices[2] - vertices[0]));

        Box::new(Triangle {
            vertices,
            normal,
            material,
        })
    }
}

impl Hitable for Triangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Source: https://en.wikipedia.org/wiki/Möller–Trumbore_intersection_algorithm#C++_Implementation
        let epsilon = 1.0e-6;
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        let h = math::cross(&r.direction, &edge2);
        let a = math::dot(&edge1, &h);
        if a > -epsilon && a < epsilon {
            return false; // The ray is parallel to the triangle
        }
        let f = 1.0 / a;
        let s = r.origin - self.vertices[0];
        let u = f * math::dot(&s, &h);
        if u < 0.0 || u > 1.0 {
            return false;
        }
        let q = math::cross(&s, &edge1);
        let v = f * math::dot(&r.direction, &q);
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * math::dot(&edge2, &q);
        if t > t_min + epsilon && t < t_max {
            rec.t = t;
            rec.p = r.point_at_parameter(t);
            rec.u = u;
            rec.v = v;
            rec.normal = self.normal;
            rec.material = self.material.clone();
            true
        } else {
            // This means there's a line intersection but not a ray intersection
            false
        }
    }
}
