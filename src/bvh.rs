use crate::aabb::AABB;
use crate::utils::random_range;
use crate::Hittable;
use crate::{HitRecord, Hittables, Interval, Ray};
use std::cmp::Ordering;
use std::sync::Arc;

// bounding volume hierarchy
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new_from_hittables(hittables: &Hittables) -> Self {
        Self::new(&hittables.objects)
    }

    pub fn new(src_objects: &Vec<Arc<dyn Hittable>>) -> Self {
        //println!("creating BvhNode for {} hittables", src_objects.len());
        let mut objects = src_objects
            .iter()
            .map(|x| x.clone())
            .collect::<Vec<Arc<dyn Hittable>>>();
        match objects.len() {
            1 => Self {
                left: src_objects[0].clone(),
                right: src_objects[0].clone(),
                bbox: src_objects[0].bounding_box(),
            },
            2 => {
                let left = objects[0].clone();
                let right = objects[1].clone();
                Self {
                    left: left.clone(),
                    right: right.clone(),
                    bbox: AABB::merge(&left.bounding_box(), &right.bounding_box()),
                }
            }
            _ => {
                // randomly pick an axis to divide the space
                let axis = random_range(0, 3);
                objects.sort_by(|a, b| Self::box_compare(a.clone(), b.clone(), axis));
                let mid = objects.len() / 2;
                let left = Self::new(&objects[..mid].to_vec());
                let right = Self::new(&objects[mid..].to_vec());
                let bbox = AABB::merge(&left.bounding_box(), &right.bounding_box());
                Self {
                    left: Arc::new(left),
                    right: Arc::new(right),
                    bbox,
                }
            }
        }
    }

    pub fn box_compare(
        box_a: Arc<dyn Hittable>,
        box_b: Arc<dyn Hittable>,
        axis_index: u32,
    ) -> Ordering {
        let a = box_a.bounding_box().axis(axis_index).min;
        let b = box_b.bounding_box().axis(axis_index).min;
        a.total_cmp(&b)
    }
}
impl Hittable for BvhNode {
    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, ray_t.clone()) {
            return None;
        }
        let hit_left = self.left.hit(ray, ray_t);
        let mut new_interval = Interval {
            min: ray_t.min,
            max: match hit_left {
                Some(ref rec) => rec.t,
                None => ray_t.max,
            },
        };
        let hit_right = self.right.hit(ray, &mut new_interval);
        match (hit_left, hit_right) {
            (Some(rec), None) => Some(rec),
            (_, Some(rec)) => Some(rec),
            _ => None,
        }
    }
}
