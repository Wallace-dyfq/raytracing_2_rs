use crate::interval;
use crate::material::Isotropic;
use crate::traits::Texture;
use crate::utils::{random_f64, INFINITY};
use crate::Color;
use crate::Interval;
use crate::Vec3;
use crate::{HitRecord, Hittable, Material};
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    // d is density
    pub fn new(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> ConstantMedium {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new(a)),
        }
    }

    // d is density
    pub fn new_with_color(b: Arc<dyn Hittable>, d: f64, c: Color) -> ConstantMedium {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new_from_color(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: &mut crate::interval::Interval,
    ) -> Option<HitRecord> {
        //print occasional samples when debugging. To enable, set it to true
        const ENABLE_DEBUG: bool = false;
        let debugging = ENABLE_DEBUG && random_f64() < 1e-5;

        let mut interval_1 = interval::UNIVERSE_INTERVAL;
        if let Some(mut rec1) = self.boundary.hit(ray, &mut interval_1) {
            let mut interval_2 = Interval::new(rec1.t + 0.0001, INFINITY);
            if let Some(mut rec2) = self.boundary.hit(ray, &mut interval_2) {
                if debugging {
                    println!("\nray_tmin = {}, ray_tmax = {} ", rec1.t, rec2.t);
                }
                if rec1.t < ray_t.min {
                    rec1.t = ray_t.min;
                }

                if rec2.t > ray_t.max {
                    rec2.t = ray_t.max;
                }
                let ray_length = ray.dir.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random_f64().log(1.0_f64.exp());
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t = rec1.t + hit_distance / ray_length;

                let mut new_hitrecord =
                    HitRecord::new(ray.at(t), self.phase_function.clone(), t, 0.0, 0.0); // if 0 0 ok?
                new_hitrecord.front_face = true;
                new_hitrecord.material = self.phase_function.clone();

                Some(new_hitrecord)
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    fn bounding_box(&self) -> crate::aabb::AABB {
        self.boundary.bounding_box()
    }
}
