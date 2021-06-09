use crate::Utils::vec3::*;

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new() -> Ray {
        Ray::from_values(&Point3::new(), &Vec3::new())
    }

    pub fn from_values(org: &Point3, dir: &Vec3) -> Ray {
        Ray {
            origin: *org,
            direction: *dir,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}