use crate::Hittable::hittable::*;
use crate::ray::*;

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
}
