use crate::Utils::util::*;

use crate::Collisions::hittable::*;
use crate::Collisions::hittable_collection::*;

use std::cmp::Ordering;

enum BVHNode {
    Branch { left: Box<BVH>, right:  Box<BVH>},
    Leaf(Box<dyn HittableTrait>)
}

pub struct BVH {
    node: BVHNode,
    hitbox: AABB,
}

impl BVH {
    pub fn bvh_from_collection(src_list: &mut HittableCollection, time0: f64, time1: f64) -> BVH {
        let mut collection = src_list.collection();
        BVH::bvh_from_list(collection, time0, time1)
    }

    pub fn bvh_from_list(src_obj: &mut Vec<Box<dyn HittableTrait>>, time0: f64, time1: f64) -> BVH {
        fn box_compare(time0: f64, time1: f64, axis: usize) -> impl FnMut(&Box<dyn HittableTrait>, &Box<dyn HittableTrait>) -> Ordering {
            move |a, b| {
                let mut box_a = AABB::new();
                let mut box_b = AABB::new();

                if a.bounding_box(time0, time1, &mut box_a) && b.bounding_box(time0, time1, &mut box_b) { // compare by center trough time
                    let ac = (box_a.min().at(axis) + box_a.max().at(axis)) / 2.0;
                    let bc = (box_b.min().at(axis) + box_b.max().at(axis)) / 2.0;

                    return ac.partial_cmp(&bc).unwrap();
                } else {
                    panic!("No bounding box during sorting.");
                }
            }
        }

        //let mut objects = &mut src_obj[start..end]; // range [start, end[

        let axis = get_rand_int(0, 2) as usize;
        src_obj.sort_by(box_compare(time0, time1, axis));
        
        let span = src_obj.len();
        if span == 1 {
            let hittable_obj = src_obj.pop().unwrap();

            let mut aabb = AABB::new();
            if hittable_obj.bounding_box(time0, time1, &mut aabb) {
                return BVH {
                    node: BVHNode::Leaf(hittable_obj),
                    hitbox: aabb,
                };
            } else {
                panic!("No AABB found for leaf");
            }
        } else {
            let left = BVH::bvh_from_list(&mut src_obj.drain(..span / 2).collect(), time0, time1); // vec of first half of src_obj vec
            let right = BVH::bvh_from_list(src_obj, time0, time1);

            let mut aabb_l = AABB::new();
            let mut aabb_r = AABB::new();

            if left.bounding_box(time0, time1, &mut aabb_l) 
                && right.bounding_box(time0, time1, &mut aabb_r) {
                    let aabb = AABB::surrounding_box(&aabb_l, &aabb_r);
                    return BVH {
                        node: BVHNode::Branch{ left: Box::new(left), right: Box::new(right)},
                        hitbox: aabb,
                    };
            } else {
                panic!("No bounding box in bvh_node constructor.");
            }
        }
    }
}

impl HittableTrait for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut Hitrecord) -> bool {
        if !self.hitbox.hit(ray, t_min, t_max) {
            return false;
        }

        match &self.node {
            BVHNode::Branch{left, right} => {
                let hit_left = left.hit(ray, t_min, t_max, record);
                let hit_right = right.hit(ray, t_min, if hit_left {record.t} else {t_max}, record);
                return hit_left || hit_right;
            },
            BVHNode::Leaf(leaf) => {
                let hit = leaf.hit(ray, t_min, t_max, record);
                return hit;
            }
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.hitbox;
        return true;
    }
}