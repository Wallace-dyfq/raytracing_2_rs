use crate::aabb::AABB;
use crate::interval::Interval;
use crate::traits::{Hittable, Material};
use crate::Lambertian;
use crate::Point3;
use crate::Ray;
use crate::Vec3;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, material: Rc<dyn Material>, t: f64, u: f64, v: f64) -> Self {
        Self {
            point,
            normal: Vec3::new(0.0, 0.0, 0.0),
            material,
            t,
            u,
            v,
            front_face: false,
        }
    }
    // set the hit record normal vector,
    // assuming outward_normal has unit length, i.e., it is normalized
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.dir.dot(&outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal.clone(),
            _ => outward_normal * -1.0,
        };
    }
}
#[derive(Default)]
pub struct Hittables {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl Hittables {
    pub fn new(hitable: Rc<dyn Hittable>) -> Self {
        let mut s = Self::default();
        s.add(hitable);
        s
    }
    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.bbox = AABB::merge(&self.bbox, &obj.bounding_box());
        self.objects.push(obj);
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        let mut hit_record = None;
        for object in self.objects.iter() {
            if let Some(tmp_hit_record) = object.hit(ray, ray_t) {
                ray_t.max = tmp_hit_record.t.clone();
                hit_record = Some(tmp_hit_record);
            }
        }
        hit_record
    }
    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
