use crate::aabb::AABB;
use crate::interval::Interval;
use crate::Color;
use crate::HitRecord;
use crate::Point3;
use crate::Ray;

#[derive(Debug, Clone, Default)]
pub struct ScatterInfo {
    pub attenuation: Color,
    pub ray_scattered: Ray,
}
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB; // or ref?
}

pub trait Material: Send + Sync {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    //TODO: might be better to combine attenuation and ray into one struct?
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterInfo>;
}

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
