use rand::Rng;
pub use crate::Utils::vec3::*;
pub use crate::ray::*;

pub fn degrees_to_radian(degrees: f64) -> f64 {
    return degrees * PI() / 180.0;
}

pub fn PI() -> f64 {
    std::f64::consts::PI
}

pub fn get_rand_int(min: i32, max: i32) -> i32 {
    get_rand_f64(min as f64, (max+1) as f64) as i32
}

pub fn get_rand_f64(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn get_rand_f64_unit() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    } else {
        return x;
    }
}