use crate::Utils::util::*;

use crate::Materials::material::*;

use crate::Collisions::hittable::*;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new() -> Lambertian {
        Lambertian::from_values(&Color::from_floats(0.5, 0.5, 0.5))
    }

    pub fn from_values(a: &Color) -> Lambertian {
        Lambertian {
            albedo: *a,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hitrecord: &Hitrecord, attenuation: &mut Color, scattered: &mut Ray) -> bool{
        let mut scatter_direction = hitrecord.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hitrecord.normal;
        }
        
        *scattered = Ray::from_values_time(&hitrecord.p, &scatter_direction, ray_in.time());
        *attenuation = self.albedo;

        true
    }

    fn clone_material(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}