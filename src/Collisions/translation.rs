/*use crate::Utils::util::*;

use crate::Collisions::hittable::*;

pub struct Translation {
    object: Box<dyn HittableTrait> ,
    offset: Vec3,
}

impl Translation {
    pub fn from_values(object: Box<dyn HittableTrait>, offset: &Vec3)
}

impl HittableTrait for Translation {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut Hitrecord) -> bool {
        let modified_ray = Ray::from_values_time(&(*ray.origin() - self.offset), ray.direction(), ray.time());
        let result = self.object.hit(&modified_ray, t_min, t_max, record);

        record.p += self.offset;

        let normal = record.normal;
        record.set_face_normal(&modified_ray, &normal);

        return result;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let result = self.object.bounding_box(time0, time1, output_box);
        
        output_box.minimum += self.offset;
        output_box.maximum += self.offset;

        return result;
    }
}*/