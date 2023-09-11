use crate::aabb::AABB;
use crate::Hittables;
use crate::{HitRecord, Hittable, Material, Point3, Ray, Vec3};
use std::sync::Arc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    material: Arc<dyn Material>,
    normal: Vec3, //  unit vector
    d: f64,       // distance from origin to the quad along normal
    bbox: AABB,
    w: Vec3,
}
// box is a reserved keyword for rust
pub fn create_box(a: Point3, b: Point3, material: Arc<dyn Material>) -> Arc<Hittables> {
    let mut sides = Hittables::default();
    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));

    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx.clone(),
        dy.clone(),
        material.clone(),
    ))); // front
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -&dz,
        dy.clone(),
        material.clone(),
    ))); // right
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -&dx,
        dy.clone(),
        material.clone(),
    ))); // back
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz.clone(),
        dy.clone(),
        material.clone(),
    ))); // left
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx.clone(),
        -&dz,
        material.clone(),
    ))); // top
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx.clone(),
        dz.clone(),
        material.clone(),
    ))); // bottom

    Arc::new(sides)
}
impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let p = &q + (&u + &v);
        let bbox = AABB::new(&q, &p).pad();
        let n = u.cross(&v);
        let normal = n.make_unit_vector();
        let d = normal.dot(&q);
        let w = &n / (n.dot(&n));
        Self {
            q,
            u,
            v,
            material,
            normal,
            d,
            bbox,
            w,
        }
    }

    // if 0<=a<=1 and 0 <=b<=1 then it is internal, otherwise false
    fn is_interior(a: f64, b: f64) -> bool {
        if a < 0.0 || a > 1.0 || b < 0.0 || b > 1.0 {
            false
        } else {
            true
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: &mut crate::interval::Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.dir);
        // no hit if the ray is parallel to the plane
        if denom.abs() < 1e-8 {
            return None;
        }
        let t = (self.d - self.normal.dot(&ray.orig)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hitpt_vec = &intersection - &self.q;
        let alpha = self.w.dot(&planar_hitpt_vec.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vec));

        if !Self::is_interior(alpha, beta) {
            return None;
        }

        let mut hit_record = HitRecord::new(ray.at(t), self.material.clone(), t, 0.0, 0.0);
        hit_record.u = alpha;
        hit_record.v = beta;
        hit_record.set_face_normal(&ray, &self.normal);

        Some(hit_record)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
