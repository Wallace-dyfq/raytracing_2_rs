use crate::aabb::AABB;
use crate::interval::Interval;
use crate::traits::{Hittable, Material};
use crate::utils::{degrees_to_radians, INFINITY};
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

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(p: Rc<dyn Hittable>, displacement: &Vec3) -> Self {
        Self {
            object: p.clone(),
            offset: displacement.clone(),
            bbox: &p.bounding_box() + &displacement,
        }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        // move the ray backward for the offset
        let ray_offset = Ray::new(&ray.orig - &self.offset, ray.dir.clone(), ray.tm);

        if let Some(mut rec) = self.object.hit(&ray_offset, ray_t) {
            rec.point += &self.offset;
            Some(rec)
        } else {
            return None;
        }
    }
}

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();
        let mut min_point = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max_point = Point3::new(-INFINITY, -INFINITY, -INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);
                    for c in 0..3 {
                        min_point[c] = min_point[c].min(tester[c]);
                        max_point[c] = max_point[c].max(tester[c]);
                    }
                }
            }
        }
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: AABB::new(&min_point, &max_point),
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        // change the ray from world space to object space
        let mut origin = ray.orig.clone();
        let mut direction = ray.dir.clone();

        origin[0] = self.cos_theta * ray.orig[0] - self.sin_theta * ray.orig[2];
        origin[2] = self.sin_theta * ray.orig[0] + self.cos_theta * ray.orig[2];

        direction[0] = self.cos_theta * ray.dir.x() - self.sin_theta * ray.dir.z();
        direction[2] = self.sin_theta * ray.dir.x() + self.cos_theta * ray.dir.z();
        let rotated_ray = Ray::new(origin, direction, ray.tm);

        // determine where (if any) an intersection occurs in object space
        if let Some(rec) = self.object.hit(&rotated_ray, ray_t) {
            // change the intersection point from object space to world space
            let mut point = rec.point.clone();
            point[0] = self.cos_theta * rec.point.x() + self.sin_theta * rec.point.z();
            point[2] = -self.sin_theta * rec.point.x() + self.cos_theta * rec.point.z();
            // change the normal from object space to world space
            let mut normal = rec.normal.clone();
            normal[0] = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
            normal[2] = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

            let new_hitrecord = HitRecord {
                point,
                normal,
                ..rec
            };

            Some(new_hitrecord)
        } else {
            None
        }
    }
}
