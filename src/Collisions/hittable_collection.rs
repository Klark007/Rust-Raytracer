use crate::Utils::util::*;

use crate::Collisions::hittable::*;

pub struct HittableCollection { // don't know size of hashset -> use box
    collection: Vec<Box<dyn HittableTrait>>,
}

impl HittableCollection {
    pub fn new() -> HittableCollection {
        HittableCollection {
            collection: Vec::new(),
        }
    }

    pub fn clear(self: &mut Self) {
        self.collection.clear();
    }

    pub fn add(self: &mut Self, hittable: Box<dyn HittableTrait>) {
        //let b = Box::new(hittable);
        self.collection.push(hittable);
    }

    pub fn len(&self) -> usize {
        self.collection.len()
    }

    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }

    pub fn collection(self: &mut Self) -> &mut Vec<Box<dyn HittableTrait>>{
        &mut self.collection
    }
}

impl HittableTrait for HittableCollection {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut Hitrecord) -> bool {
        let mut temp_record = Hitrecord::new();
        let mut hit_anything = false;

        let mut closest = t_max;

        for hit_box in self.collection.iter() {
            if hit_box.hit(ray, t_min, t_max, &mut temp_record) {
                hit_anything = true;
                if closest > temp_record.t {
                    closest = temp_record.t;
                    *record = temp_record.clone();
                }
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.is_empty() {
            return false;
        }

        let mut temp_box: AABB = AABB::new();
        let first_box = true;

        for object in self.collection.iter() {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box { temp_box } else { AABB::surrounding_box(&temp_box, output_box) };
        }

        return true;
    }
}
