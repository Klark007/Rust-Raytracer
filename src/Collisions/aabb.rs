use crate::Utils::util::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3
}

impl AABB {
    pub fn new() -> AABB{
        AABB::from_values(&Point3::new(), &Point3::new())
    }

    // pre: min < max (for each dimension)
    pub fn from_values(min: &Point3, max: &Point3) -> AABB {
        AABB {
            minimum: *min,
            maximum: *max,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for a in 0..3 {
            let inv_d = 1.0 / ray.direction().at(a);
            let mut t0 = (self.min().at(a) - ray.origin().at(a)) * inv_d;
            let mut t1 = (self.max().at(a) - ray.origin().at(a)) * inv_d;

            if inv_d < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }

            t_min = if t0 > t_min {t0} else {t_min};
            t_max = if t1 < t_max {t1} else {t_max};

            if t_max <= t_min {
                return false;
            }
        }
        return true;
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB{
        let small = Vec3::from_floats(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );

        let big = Vec3::from_floats(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );

        AABB::from_values(&small, &big)
    }

    pub fn min(&self) -> &Point3 {
       &self.minimum 
    }

    pub fn max(&self) -> &Point3 {
        &self.maximum 
     }
}

#[cfg(test)]
mod tests {
    use crate::Utils::util::*;
    use crate::Collisions::aabb::*;

    #[test]
    fn surrounding_test() {
        let same = AABB::from_values(&Point3::from_ints(-1, -1, -1), &Point3::from_ints(1, 1, 1));
        assert_eq!(same, AABB::surrounding_box(&same, &same));

        let big = AABB::from_values(&Point3::from_ints(-2, -2, -2), &Point3::from_ints(2, 2, 2));
        assert_eq!(big, AABB::surrounding_box(&big, &same));
        assert_ne!(same, AABB::surrounding_box(&big, &same));

        let left = AABB::from_values(&Point3::from_ints(-1, -1, -1), &Point3::from_floats(0.5, 0.5, 0.5));
        let right = AABB::from_values(&Point3::from_floats(-0.5, -0.5, -0.5), &Point3::from_ints(1, 1, 1));
        let intersecting_sol = AABB::from_values(&Point3::from_ints(-1, -1, -1),&Point3::from_ints(1, 1, 1));
        assert_eq!(intersecting_sol, AABB::surrounding_box(&left, &right));
    }

    #[test]
    fn hit_test() {
        let bounding_box = AABB::from_values(&Point3::from_ints(-1, -1, -1), &Point3::from_ints(1, 1, 1));

        let x_hit = Ray::from_values(&Point3::from_ints(2, 0, 0), &Vec3::from_ints(-1, 0, 0));
        assert!(bounding_box.hit(&x_hit, 0.0, 1000.0));

        let x_miss = Ray::from_values(&Point3::from_ints(2, 0, 0), &Vec3::from_ints(1, 0, 0));
        assert!(!bounding_box.hit(&x_miss, 0.0, 1000.0));


        let y_hit = Ray::from_values(&Point3::from_ints(0, 2, 0), &Vec3::from_ints(0, -1, 0));
        assert!(bounding_box.hit(&y_hit, 0.0, 1000.0));

        let y_miss = Ray::from_values(&Point3::from_ints(0, 2, 0), &Vec3::from_ints(0, 1, 0));
        assert!(!bounding_box.hit(&y_miss, 0.0, 1000.0));


        let z_hit = Ray::from_values(&Point3::from_ints(0, 0, 2), &Vec3::from_ints(0, 0, -1));
        assert!(bounding_box.hit(&z_hit, 0.0, 1000.0));

        let z_miss = Ray::from_values(&Point3::from_ints(0, 0, 2), &Vec3::from_ints(0, 0, 1));
        assert!(!bounding_box.hit(&z_miss, 0.0, 1000.0));


        let inv_bounding_box = AABB::from_values(&Point3::from_ints(1, 1, 1), &Point3::from_ints(-1, -1, -1));
        
        let x_hit = Ray::from_values(&Point3::from_ints(2, 0, 0), &Vec3::from_ints(-1, 0, 0));
        assert!(inv_bounding_box.hit(&x_hit, 0.0, 1000.0));
    }
}