use crate::Utils::util::*;

use crate::Collisions::hittable::*;

use crate::Materials::material::*;

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub radius: f64,
    pub material: Box<dyn Material>,
    time0: f64,
    time1: f64,
}

impl MovingSphere {
    pub fn from_values(cen0: &Point3, cen1: &Point3, time0: f64, time1: f64, rad: f64, mat: &Box<dyn Material>) -> MovingSphere {
        MovingSphere {
            center0: *cen0,
            center1: *cen1,
            radius: rad,
            material: (*mat).clone(),
            time0, time1
        }
    }

    fn center(&self, time: f64) -> Point3{
        return self.center0 + ((self.center1-self.center0) * (time - self.time0) / (self.time1 - self.time0));
    }
}

impl HittableTrait for MovingSphere {
    // (P(t) - C)*(P(t)-C) = r^2
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut Hitrecord) -> bool {
        let oc = *ray.origin() - self.center(ray.time());
        let a = ray.direction().length_squared();
        let half_b = dot(ray.direction(), &oc);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false; // ray misses
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;   
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a; 
            if root < t_min || t_max < root {
                return false; // collision lies out of boundaries
            }
        }

        record.t = root;
        record.p = ray.at(record.t);
        let outward_normal = (record.p - self.center(ray.time())) / self.radius;
        record.set_face_normal(&ray, &outward_normal);
        record.material = self.material.clone();

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB::from_values(
            &(self.center(time0) - Vec3::from_floats(self.radius, self.radius, self.radius)), 
            &(self.center(time0) + Vec3::from_floats(self.radius, self.radius, self.radius))
        );

        let box1 = AABB::from_values(
            &(self.center(time1) - Vec3::from_floats(self.radius, self.radius, self.radius)), 
            &(self.center(time1) + Vec3::from_floats(self.radius, self.radius, self.radius))
        );

        *output_box = AABB::surrounding_box(&box0, &box1);

        return true;
    }
}