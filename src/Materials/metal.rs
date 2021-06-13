use crate::Materials::material::*;

use crate::Utils::vec3::*;
use crate::ray::*;
use crate::Hittable::hittable::*;

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn from_values(a: &Color, f: f64) -> Metal {
        Metal {
            albedo: *a,
            fuzz: f,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hitrecord: &Hitrecord, attenuation: &mut Color, scattered: &mut Ray) -> bool{
        let reflected = reflect(&unit_vector(ray_in.direction()), &hitrecord.normal);
        *scattered = Ray::from_values_time(&hitrecord.p, &(reflected + Vec3::random_in_unitsphere() * self.fuzz), ray_in.time());
        *attenuation = self.albedo;

        return dot(scattered.direction(), &hitrecord.normal) > 0.0;
    }

    fn clone_material(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}