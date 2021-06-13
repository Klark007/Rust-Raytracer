use crate::Utils::vec3::*;

pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new() -> Ray {
        Ray::from_values(&Point3::new(), &Vec3::new())
    }

    pub fn from_values(org: &Point3, dir: &Vec3) -> Ray {
        Ray::from_values_time(org, dir, 0.0)
    }

    pub fn from_values_time(org: &Point3, dir: &Vec3, t: f64) -> Ray {
        Ray {
            origin: *org,
            direction: *dir,
            time: t
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}