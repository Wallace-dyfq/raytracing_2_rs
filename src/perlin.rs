use crate::utils::*;
use crate::Point3;

#[derive(Debug)]
pub struct Perlin {
    ranfloat: Vec<f64>,
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
        let ranfloat = (0..POINT_COUNT)
            .into_iter()
            .map(|_| random_f64())
            .collect::<Vec<f64>>();
        Self {
            ranfloat,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x()) as i32 & 255;
        let j = (4.0 * p.y()) as i32 & 255;
        let k = (4.0 * p.z()) as i32 & 255;

        self.ranfloat
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
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
