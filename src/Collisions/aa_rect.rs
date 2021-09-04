use crate::Utils::util::*;

use crate::Collisions::hittable::*;

use crate::Materials::material::*;
use crate::Materials::emmiter::*;

pub struct XY_Rect {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub k: f64,
    pub material: Box<dyn Material>,
    pub emmiter: Option<Box<dyn Emmiter>>,
}

pub struct XZ_Rect {
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
    pub k: f64,
    pub material: Box<dyn Material>,
    pub emmiter: Option<Box<dyn Emmiter>>,
}


pub struct YZ_Rect {
    pub y0: f64,
    pub z0: f64,
    pub y1: f64,
    pub z1: f64,
    pub k: f64,
    pub material: Box<dyn Material>,
    pub emmiter: Option<Box<dyn Emmiter>>,
}

impl XY_Rect {
    pub fn from_floats(x0: f64, y0: f64, x1: f64, y1: f64, k: f64, mat: &Box<dyn Material>) -> XY_Rect {
        XY_Rect {
            x0,
            y0,
            x1,
            y1,
            k,
            material: (*mat).clone(),
            emmiter: None
        }
    }

    pub fn from_ints(x0: i32, y0: i32, x1: i32, y1: i32, k: i32, mat: &Box<dyn Material>) -> XY_Rect {
        XY_Rect::from_floats(x0 as f64, y0 as f64, x1 as f64, y1 as f64, k as f64, mat)
    }

    pub fn as_emmiter(x0: f64, y0: f64, x1: f64, y1: f64, k: f64, mat: &Box<dyn Material>, emmiter: &Box<dyn Emmiter>) -> XY_Rect {
        XY_Rect {
            x0,
            y0,
            x1,
            y1,
            k,
            material: (*mat).clone(),
            emmiter: Some((*emmiter).clone())
        }
    }
}

impl XZ_Rect {
    pub fn from_floats(x0: f64, z0: f64, x1: f64, z1: f64, k: f64, mat: &Box<dyn Material>) -> XZ_Rect {
        XZ_Rect {
            x0,
            z0,
            x1,
            z1,
            k,
            material: (*mat).clone(),
            emmiter: None
        }
    }

    pub fn from_ints(x0: i32, z0: i32, x1: i32, z1: i32, k: i32, mat: &Box<dyn Material>) -> XZ_Rect {
        XZ_Rect::from_floats(x0 as f64, z0 as f64, x1 as f64, z1 as f64, k as f64, mat)
    }

    pub fn as_emmiter(x0: f64, z0: f64, x1: f64, z1: f64, k: f64, mat: &Box<dyn Material>, emmiter: &Box<dyn Emmiter>) -> XZ_Rect {
        XZ_Rect {
            x0,
            z0,
            x1,
            z1,
            k,
            material: (*mat).clone(),
            emmiter: Some((*emmiter).clone())
        }
    }
}

impl YZ_Rect {
    pub fn from_floats(y0: f64, z0: f64, y1: f64, z1: f64, k: f64, mat: &Box<dyn Material>) -> YZ_Rect {
        YZ_Rect {
            y0,
            z0,
            y1,
            z1,
            k,
            material: (*mat).clone(),
            emmiter: None
        }
    }

    pub fn from_ints(y0: i32, z0: i32, y1: i32, z1: i32, k: i32, mat: &Box<dyn Material>) -> YZ_Rect {
        YZ_Rect::from_floats(y0 as f64, z0 as f64, y1 as f64, z1 as f64, k as f64, mat)
    }

    pub fn as_emmiter(y0: f64, z0: f64, y1: f64, z1: f64, k: f64, mat: &Box<dyn Material>, emmiter: &Box<dyn Emmiter>) -> YZ_Rect {
        YZ_Rect {
            y0,
            z0,
            y1,
            z1,
            k,
            material: (*mat).clone(),
            emmiter: Some((*emmiter).clone())
        }
    }
}


impl HittableTrait for XY_Rect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut Hitrecord) -> bool {
        let t = (self.k-ray.origin().z()) / ray.direction().z();
        
        if t < t_min || t > t_max {
            return false;
        }

        let x = ray.origin().x() + ray.direction().x() * t;
        let y = ray.origin().y() + ray.direction().y() * t;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        record.u = (x - self.x0) / (self.x1 - self.x0);
        record.v = (y - self.y0) / (self.y1 - self.y0);
        record.t = t;
        
        let outward_normal = Vec3::from_ints(0,0,1);
        record.set_face_normal(ray, &outward_normal);

        record.material = self.material.clone();
        record.emmiter = match &self.emmiter {
            Some(emm) => Some((*emm).clone()),
            None => None,
        };
        record.p = ray.at(t);

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::from_values(&Point3::from_floats(self.x0, self.y0, self.k-0.0001), &Point3::from_floats(self.x1, self.y1, self.k+0.0001));
        
        return true;
    }
}

impl HittableTrait for XZ_Rect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut Hitrecord) -> bool {
        let t = (self.k-ray.origin().y()) / ray.direction().y();
        
        if t < t_min || t > t_max {
            return false;
        }

        let x = ray.origin().x() + ray.direction().x() * t;
        let z = ray.origin().z() + ray.direction().z() * t;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }

        record.u = (x - self.x0) / (self.x1 - self.x0);
        record.v = (z - self.z0) / (self.z1 - self.z0);
        record.t = t;
        
        let outward_normal = Vec3::from_ints(0,1,0);
        record.set_face_normal(ray, &outward_normal);

        record.material = self.material.clone();
        record.emmiter = match &self.emmiter {
            Some(emm) => Some((*emm).clone()),
            None => None,
        };
        record.p = ray.at(t);

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::from_values(&Point3::from_floats(self.x0, self.k-0.0001, self.z0), &Point3::from_floats(self.x1, self.k+0.0001, self.z1));
        
        return true;
    }
}

impl HittableTrait for YZ_Rect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut Hitrecord) -> bool {
        let t = (self.k-ray.origin().x()) / ray.direction().x();
        
        if t < t_min || t > t_max {
            return false;
        }

        let y = ray.origin().y() + ray.direction().y() * t;
        let z = ray.origin().z() + ray.direction().z() * t;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }

        record.u = (y - self.y0) / (self.y1 - self.y0);
        record.v = (z - self.z0) / (self.z1 - self.z0);
        record.t = t;
        
        let outward_normal = Vec3::from_ints(1,0,0);
        record.set_face_normal(ray, &outward_normal);

        record.material = self.material.clone();
        record.emmiter = match &self.emmiter {
            Some(emm) => Some((*emm).clone()),
            None => None,
        };
        record.p = ray.at(t);

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::from_values(&Point3::from_floats(self.k-0.0001, self.y0, self.z0), &Point3::from_floats(self.k+0.0001, self.y1, self.z1));
        
        return true;
    }
}