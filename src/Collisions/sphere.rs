use crate::Utils::util::*;

use crate::Collisions::hittable::*;

use crate::Materials::material::*;
use crate::Materials::lambertian::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new() -> Sphere {
        let b: Box<dyn Material> = Box::new(Lambertian::new());
        Sphere::from_values(&Point3::new(), 0.0, &b)
    }

    pub fn from_values(cen: &Point3, rad: f64, mat: &Box<dyn Material>) -> Sphere {
        Sphere {
            center: *cen,
            radius: rad,
            material: (*mat).clone(),
        }
    }

    fn sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let phi = p.z().atan2(-p.x()) + PI();
        let theta = (-p.y()).acos();

        *u = phi / (2.0*PI());
        *v = theta / PI();
    }
}

impl HittableTrait for Sphere {
    // (P(t) - C)*(P(t)-C) = r^2
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut Hitrecord) -> bool {
        let oc = *ray.origin() - self.center;
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
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(&ray, &outward_normal);
        Sphere::sphere_uv(&record.p, &mut record.u, &mut record.v);
        record.material = self.material.clone();

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::from_values(
            &(self.center - Vec3::from_floats(self.radius, self.radius, self.radius)), 
            &(self.center + Vec3::from_floats(self.radius, self.radius, self.radius))
        );

        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::Utils::util::*;
    use crate::Collisions::sphere::*;

    #[test]
    fn bounding_box_test() {

    }
}