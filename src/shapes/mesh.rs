use std::fs::OpenOptions;
use std::sync::Arc;

use serde_json::Value;

use crate::materials::{dielectric, diffuse_light, lambertian, metal, Material};
use crate::shapes::triangle::Triangle;
use crate::textures::TextureType;
use crate::transform::{rotate::Rotate, translate::Translate};
use crate::util::{
    hitable::{HitRecord, Hitable},
    hitable_list::HitableList,
    json, math,
    ray::Ray,
    vector3::Vec3,
};

pub struct Mesh {
    triangles: HitableList,
}

impl Mesh {
    pub fn new(filename: &str, material: Arc<Material + Sync + Send>, scale: f64) -> Box<Hitable> {
        let mut triangles: Vec<Box<Hitable + Sync>> = Vec::new();

        let mut file = OpenOptions::new().read(true).open(filename).unwrap();
        let stl = stl_io::read_stl(&mut file).unwrap();

        let mut vertices: Vec<Vec3> = Vec::new();

        for v in &stl.vertices {
            vertices.push(Vec3::new(
                f64::from(v[0]) * scale,
                f64::from(v[1]) * scale,
                f64::from(v[2]) * scale,
            ));
        }

        for indices in &stl.faces {
            triangles.push(Triangle::new(
                vec![
                    vertices[indices.vertices[0] as usize],
                    vertices[indices.vertices[1] as usize],
                    vertices[indices.vertices[2] as usize],
                ],
                material.clone(),
            ));
        }

        Box::new(Mesh {
            triangles: HitableList::new(triangles),
        })
    }
}

impl Hitable for Mesh {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.triangles.hit(r, t_min, t_max, rec)
    }
}
