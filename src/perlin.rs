use crate::utils::*;
use crate::{Point3, Vec3};

#[derive(Debug)]
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

static POINT_COUNT: usize = 256;
impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

impl Perlin {
    pub fn new() -> Self {
        let ranvec = (0..POINT_COUNT)
            .into_iter()
            .map(|_| Vec3::random(0.0, 1.0))
            .collect::<Vec<Vec3>>();
        Self {
            ranvec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = (p.x().floor()) as i32;
        let j = (p.y().floor()) as i32;
        let k = (p.z().floor()) as i32;
        let mut c = vec![vec![vec![Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let ii = ((i + di as i32) & 255) as usize;
                    let jj = ((j + dj as i32) & 255) as usize;
                    let kk = ((k + dk as i32) & 255) as usize;
                    c[di][dj][dk] = self.ranvec
                        [(self.perm_x[ii] ^ self.perm_y[jj] ^ self.perm_z[kk]) as usize]
                        .clone();
                }
            }
        }
        Self::trilinear_interp(c, u, v, w)
    }

    fn trilinear_interp(c: Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = vec![0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            if let Some(elem) = p.get_mut(i) {
                *elem = i as i32;
            }
        }

        Self::permute(&mut p, POINT_COUNT);
        p
    }
    fn permute(p: &mut Vec<i32>, n: usize) {
        for i in (1..n).rev() {
            let target = random_range(0, i);
            p.swap(i, target);
        }
    }
}
