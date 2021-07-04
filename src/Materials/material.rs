
use crate::Utils::util::*;

use crate::Collisions::hittable::*;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hitrecord: &Hitrecord, attenuation: &mut Color, scattered: &mut Ray) -> bool; 

    fn clone_material(&self) -> Box<dyn Material>;
}

// Material Box needs to have Copy trait
// https://hashrust.com/blog/moves-copies-and-clones-in-rust/
// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6
impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_material()
    }
}