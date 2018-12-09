use rand::Rng;

use crate::util::{math, vector3::Vec3};

pub struct Perlin {
    pub random_vectors: Vec<Vec3>,
    pub perm_x: Vec<i64>,
    pub perm_y: Vec<i64>,
    pub perm_z: Vec<i64>,
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {
            random_vectors: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;
        let mut c = vec![vec![vec![Vec3::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vectors[(self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255])
                        as usize];
                }
            }
        }
        trilinear_interp(c, u, v, w)
    }

    pub fn turbulence(&self, p: Vec3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

fn perlin_generate() -> Vec<Vec3> {
    let mut rng = rand::thread_rng();
    (0..256)
        .into_iter()
        .map(|_| {
            math::unit_vector(&Vec3::new(
                -1.0 + 2.0 * rng.gen::<f64>(),
                -1.0 + 2.0 * rng.gen::<f64>(),
                -1.0 + 2.0 * rng.gen::<f64>(),
            ))
        })
        .collect()
}

fn perlin_generate_perm() -> Vec<i64> {
    let mut p: Vec<i64> = (0..256).into_iter().collect();
    permute(&mut p);
    p
}

fn permute(p: &mut Vec<i64>) {
    let mut rng = rand::thread_rng();
    let n = p.len();
    for i in (0..n).rev() {
        let target = (rng.gen::<f64>() * (i + 1) as f64) as usize;
        let temp = p[i as usize];
        p[i as usize] = p[target];
        p[target] = temp;
    }
}

fn trilinear_interp(c: Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    // There's probably an idiomatic way to do this with iterators, sum(), zip(), etc.
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                    * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                    * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                    * math::dot(&c[i][j][k], &weight_v);
            }
        }
    }
    accum
}
