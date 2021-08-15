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
        record.p = ray.at(t);

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::from_values(&Point3::from_floats(self.x0, self.y0, self.k-0.0001), &Point3::from_floats(self.x1, self.y1, self.k+0.0001));
        
        return true;
    }
}