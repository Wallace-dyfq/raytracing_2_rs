use crate::texture::{CheckerTexture, SolidColor};
use crate::traits::{ScatterInfo, Texture};
use crate::utils::random_f64;
use crate::Color;
use crate::Material;
use crate::Point3;
use crate::Ray;
use crate::Vec3;
use std::rc::Rc;

// diffusive
#[derive(Clone)]
pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

// reflective
#[derive(Clone)]
pub struct Metal {
    albedo: Rc<dyn Texture>,
    fuzz: f64,
}

#[derive(Default, Debug, Clone)]
pub struct Dielectric {
    ir: f64, // index of reflection
}
impl Lambertian {
    pub fn new_from_color(color: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new(color)),
        }
    }

    pub fn new(texture: Rc<dyn Texture>) -> Self {
        Self { albedo: texture }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        rec: &crate::hittables::HitRecord,
    ) -> Option<ScatterInfo> {
        let mut scatter_direction = &rec.normal + Vec3::random_unit_vec3();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        let ray_scattered = Ray {
            orig: rec.point.clone(),
            dir: scatter_direction,
            tm: ray_in.tm,
        };
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.point);
        Some(ScatterInfo {
            attenuation,
            ray_scattered,
        })
    }
}

impl Metal {
    pub fn new_from_color(color: Color, fuzz: f64) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new(color)),
            fuzz,
        }
    }

    pub fn new(texture: Rc<dyn Texture>, fuzz: f64) -> Self {
        Self {
            albedo: texture,
            fuzz,
        }
    }
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &crate::hittables::HitRecord) -> Option<ScatterInfo> {
        let reflected = Vec3::reflect(&ray_in.dir, &rec.normal);
        let ray_scattered = Ray {
            orig: rec.point.clone(),
            dir: reflected + Vec3::random_unit_vec3() * self.fuzz,
            tm: ray_in.tm,
        };
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.point);
        Some(ScatterInfo {
            attenuation,
            ray_scattered,
        })
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // use Schlick's approximation for reflectance
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0.powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &crate::hittables::HitRecord) -> Option<ScatterInfo> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray_in.dir.make_unit_vector();
        let cos_theta = unit_direction.reverse().dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let can_reflect = refraction_ratio * sin_theta > 1.0;
        let direction =
            if can_reflect || Self::reflectance(cos_theta, refraction_ratio) > random_f64() {
                Vec3::reflect(&unit_direction, &rec.normal)
            } else {
                Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
            };
        let ray_scattered = Ray {
            orig: rec.point.clone(),
            dir: direction,
            tm: ray_in.tm,
        };

        let attenuation = Color::new(1.0, 1.0, 1.0);

        Some(ScatterInfo {
            attenuation,
            ray_scattered,
        })
    }
}

pub struct DiffuseLight {
    pub emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Rc<dyn Texture>) -> Self {
        Self { emit: texture }
    }

    pub fn new_from_color(color: Color) -> Self {
        Self {
            emit: Rc::new(SolidColor::new(color)),
        }
    }
}
impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }

    fn scatter(&self, ray_in: &Ray, rec: &crate::hittables::HitRecord) -> Option<ScatterInfo> {
        None
    }
}

pub struct Isotropic {
    albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn new(a: Rc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
    pub fn new_from_color(c: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new(c)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, rec: &crate::hittables::HitRecord) -> Option<ScatterInfo> {
        let ray_scattered = Ray::new(rec.point.clone(), Vec3::random_unit_vec3(), ray_in.tm);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.point);
        Some(ScatterInfo {
            attenuation,
            ray_scattered,
        })
    }
}
