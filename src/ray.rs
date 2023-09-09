use crate::vec3::{Point3, Vec3};

#[derive(Default, Debug)]
pub struct Ray {
    pub orig: Point3, // origin
    pub dir: Vec3,    // direction
    pub tm: f64,      // time
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, tm: f64) -> Self {
        Self { orig, dir, tm }
    }
    pub fn at(&self, t: f64) -> Point3 {
        &self.orig + &self.dir * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let ray = Ray {
            orig: Point3::default(),
            dir: Vec3::default(),
            tm: 0.0,
        };
        assert_eq!(ray.at(10.0), Vec3::default());
    }

    #[test]
    fn test_1() {
        let ray = Ray {
            orig: Point3::default(),
            dir: Vec3::new(1.0, 0.0, 0.0),
            tm: 0.0,
        };
        assert_eq!(ray.at(10.0), Vec3::new(10.0, 0.0, 0.0));
    }
    #[test]
    fn test_123() {
        let ray = Ray {
            orig: Point3::default(),
            dir: Vec3::new(1.0, 2.0, 3.0),
            tm: 0.0,
        };
        assert_eq!(ray.at(10.0), Vec3::new(10.0, 20.0, 30.0));
    }
}
