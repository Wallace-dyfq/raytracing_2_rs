use crate::aabb::AABB;
use crate::interval::Interval;
use crate::utils::PI;
use crate::HitRecord;
use crate::Hittable;
use crate::Material;
use crate::Ray;
use crate::{Point3, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    center0: Point3,
    radius: f64,
    material: Arc<dyn Material>,
    center_vec: Option<Vec3>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center0: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center0: center0.clone(),
            radius,
            material,
            center_vec: None,
            bbox: AABB::new(&(&center0 - &rvec), &(&center0 + &rvec)),
        }
    }

    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        // since moving is linear, we can merge two boxes here
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::new(&(&center0 - &rvec), &(&center0 + &rvec));
        let box2 = AABB::new(&(&center1 - &rvec), &(&center1 + &rvec));
        let bbox = AABB::merge(&box1, &box2);
        Sphere {
            center0: center0.clone(),
            radius,
            material,
            center_vec: Some(center1 - center0),
            bbox,
        }
    }

    // linearly interpolate from center0 to center1 according to time
    // where time = 0 yields center0 and time = 1 yields center1
    pub fn center(&self, time: f64) -> Point3 {
        if let Some(ref center_vec) = self.center_vec {
            &self.center0 + time * center_vec
        } else {
            self.center0.clone()
        }
    }
    pub fn get_shpere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        let center = self.center(ray.tm);
        let oc = &ray.orig - &center;
        let a = ray.dir.dot(&ray.dir);
        let half_b = oc.dot(&ray.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        } else {
            // find the nearest root that lies in the acceptable range
            let mut root = (-half_b - discriminant.sqrt()) / a;
            if !ray_t.surrounds(root) {
                root = (-half_b + discriminant.sqrt()) / a;
                if !ray_t.surrounds(root) {
                    return None;
                }
            }
            let mut hit_record =
                HitRecord::new(ray.at(root), self.material.clone(), root, 0.0, 0.0);
            let outward_normal = (&hit_record.point - &center) / self.radius;
            hit_record.set_face_normal(&ray, &outward_normal);
            (hit_record.u, hit_record.v) = Self::get_shpere_uv(&outward_normal);
            Some(hit_record)
        }
    }
}
