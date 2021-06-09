use std::ops::*;
use crate::Utils::util::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3::from_floats(0.0, 0.0, 0.0)
    }

    pub fn from_floats(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x, y, z}
    }

    pub fn from_ints(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3::from_floats(x as f64, y as f64, z as f64)
    }

    pub fn length(&self) -> f64 { // &self is syntactic sugar for self: &Self
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3::from_floats(get_rand_f64(min, max), get_rand_f64(min, max), get_rand_f64(min, max))
    }

    pub fn random_in_unitsphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0, 1.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        unit_vector(&Vec3::random_in_unitsphere())
    }

    pub fn near_zero(&self) -> bool {
        let s: f64 = 1e-8;
        (self.x().abs() < s) && (self.y().abs() < s) && (self.y().abs() < s) 
    }

    // getters
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn r(&self) -> f64 {
        self.x
    }

    pub fn g(&self) -> f64 {
        self.y
    }

    pub fn b(&self) -> f64 {
        self.z
    }
}

// helper functions
// should take in &Vec
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    (*u).x * (*v).x + (*u).y * (*v).y + (*u).z * (*v).z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::from_floats(
        (*u).y * (*v).z - (*u).z * (*v).y,
        (*u).z * (*v).x - (*u).x * (*v).z,
        (*u).x * (*v).y - (*u).y * (*v).x
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    let len = (*v).length();
    *v / len
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - (*n * dot(v, n)  * 2.0)
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-(*uv), n).min(1.0); // https://stackoverflow.com/questions/28446632/how-do-i-get-the-minimum-or-maximum-value-of-an-iterator-containing-floating-poi
    let r_out_perp = (*uv + *n*cos_theta) * etai_over_etat;
    let r_out_para = *n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    
    r_out_perp + r_out_para
}


impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { 
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { 
            x: self.x - other.x, 
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { 
            x: self.x * other.x, 
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}


impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self { 
            x: self.x * rhs, 
            y: self.y * rhs, 
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self { 
            x: self.x / rhs, 
            y: self.y / rhs, 
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// aliasing
pub type Point3 = Vec3;
pub type Color = Vec3;