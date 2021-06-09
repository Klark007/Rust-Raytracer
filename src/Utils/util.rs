use rand::Rng;

pub fn degrees_to_radian(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
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