use crate::Utils::util::*;

use crate::Materials::material::*;
use crate::Materials::lambertian::*;

pub use crate::Collisions::aabb::*;

#[derive(Clone)]
pub struct Hitrecord{
    pub p: Point3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl Hitrecord {
    pub fn new() -> Hitrecord {
        Hitrecord {
            p: Point3::new(),
            normal: Vec3::new(),
            material: Box::new(Lambertian::new()),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(self: &mut Self, ray: &Ray, outward_normal: &Vec3) { // normal is always incident to ray
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-*outward_normal};
    }
}

pub trait HittableTrait {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut Hitrecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}