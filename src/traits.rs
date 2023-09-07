use crate::aabb::AABB;
use crate::interval::Interval;
use crate::Color;
use crate::HitRecord;
use crate::Ray;
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB; // or ref?
}

//TODO: might be better to combine attenuation and ray into one struct?
pub trait Scatter {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool;
}
