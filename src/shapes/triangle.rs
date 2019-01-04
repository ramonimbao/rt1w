use std::sync::Arc;

use crate::materials::Material;
use crate::util::{
    hitable::{HitRecord, Hitable},
    math,
    ray::Ray,
    vector3::Vec3,
};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3) -> Vertex {
        Vertex { position, normal }
    }
}

pub struct Triangle {
    vertices: [Vertex; 3],
    material: Arc<Material + Sync + Send>,
}

impl Triangle {
    pub fn create(vertices: [Vertex; 3], material: Arc<Material + Sync + Send>) -> Box<Triangle> {
        Box::new(Triangle { vertices, material })
    }
}

impl Hitable for Triangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Source: https://en.wikipedia.org/wiki/Möller–Trumbore_intersection_algorithm#C++_Implementation
        let epsilon = 1.0e-6;
        let edge1 = self.vertices[1].position - self.vertices[0].position;
        let edge2 = self.vertices[2].position - self.vertices[0].position;
        let h = math::cross(&r.direction, &edge2);
        let a = math::dot(&edge1, &h);
        if a > -epsilon && a < epsilon {
            return false; // The ray is parallel to the triangle
        }
        let f = 1.0 / a;
        let s = r.origin - self.vertices[0].position;
        let u = f * math::dot(&s, &h);
        if u < 0.0 || u > 1.0 {
            return false;
        }
        let q = math::cross(&s, &edge1);
        let v = f * math::dot(&r.direction, &q);
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let (n0, n1, n2) = (
            self.vertices[0].normal,
            self.vertices[1].normal,
            self.vertices[2].normal,
        );
        let normal = (1.0 - u - v) * n0 + u * n1 + v * n2;

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * math::dot(&edge2, &q);
        if t > t_min + epsilon && t < t_max {
            rec.t = t;
            rec.p = r.point_at_parameter(t);
            rec.u = u;
            rec.v = v;
            rec.normal = math::unit_vector(&normal);
            rec.material = self.material.clone();
            true
        } else {
            // This means there's a line intersection but not a ray intersection
            false
        }
    }
}
