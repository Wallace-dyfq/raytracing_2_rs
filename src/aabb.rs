use crate::Interval;
use crate::Point3;
use crate::Ray;
use crate::Vec3;
use std::ops;

#[derive(Debug, Clone, Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(a: &Point3, b: &Point3) -> Self {
        Self {
            x: Interval {
                min: a.x().min(b.x()),
                max: a.x().max(b.x()),
            },
            y: Interval {
                min: a.y().min(b.y()),
                max: a.y().max(b.y()),
            },
            z: Interval {
                min: a.z().min(b.z()),
                max: a.z().max(b.z()),
            },
        }
    }

    /// return an AABB that has no side narrow than some delta, paddin if necessary
    pub fn pad(&self) -> Self {
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta {
            self.x.clone()
        } else {
            self.x.expand(delta)
        };
        let new_y = if self.y.size() >= delta {
            self.y.clone()
        } else {
            self.y.expand(delta)
        };
        let new_z = if self.z.size() >= delta {
            self.z.clone()
        } else {
            self.z.expand(delta)
        };
        Self {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }
    // merge two boxes
    pub fn merge(a: &Self, b: &Self) -> Self {
        Self {
            x: Interval::merge(a.axis(0), b.axis(0)),
            y: Interval::merge(a.axis(1), b.axis(1)),
            z: Interval::merge(a.axis(2), b.axis(2)),
        }
    }
    pub fn axis(&self, n: u32) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        if !Self::hit_helper(self.axis(0), ray.orig.x(), ray.dir.x(), &mut ray_t)
            || !Self::hit_helper(self.axis(1), ray.orig.y(), ray.dir.y(), &mut ray_t)
            || !Self::hit_helper(self.axis(2), ray.orig.z(), ray.dir.z(), &mut ray_t)
        {
            //println!("{:?}, {:?}, not hit", ray, ray_t);
            return false;
        }

        //println!("{:?}, {:?}, hit", ray, ray_t);
        true
    }

    fn hit_helper(xx: &Interval, rx: f64, dx: f64, ray_t: &mut Interval) -> bool {
        let inv_d = 1.0 / dx;

        let mut t0 = (xx.min - rx) * inv_d;
        let mut t1 = (xx.max - rx) * inv_d;

        if inv_d < 0.0 {
            // so that t0 <= t1
            std::mem::swap(&mut t0, &mut t1);
        }

        // update ray_t interval if applicable
        if t0 > ray_t.min {
            ray_t.min = t0;
        }

        if t1 < ray_t.max {
            ray_t.max = t1;
        }

        if ray_t.max <= ray_t.min {
            return false;
        }
        true
    }
}

impl ops::Add<&AABB> for &Vec3 {
    type Output = AABB;
    fn add(self, rhs: &AABB) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<&AABB> for Vec3 {
    type Output = AABB;
    fn add(self, rhs: &AABB) -> Self::Output {
        rhs + &self
    }
}

impl ops::Add<&Vec3> for &AABB {
    type Output = AABB;
    fn add(self, rhs: &Vec3) -> Self::Output {
        AABB {
            x: &self.x + rhs.x(),
            y: &self.x + rhs.y(),
            z: &self.x + rhs.z(),
        }
    }
}
impl ops::Add<&Vec3> for AABB {
    type Output = AABB;
    fn add(self, rhs: &Vec3) -> Self::Output {
        AABB {
            x: &self.x + rhs.x(),
            y: &self.x + rhs.y(),
            z: &self.x + rhs.z(),
        }
    }
}
