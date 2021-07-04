use crate::Utils::util::*;

use crate::Materials::material::*;

use crate::Collisions::hittable::*;

#[derive(Clone)]
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn from_values(ir: f64) -> Dielectric {
        Dielectric {
            index_of_refraction: ir,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0 * r0;
        r0 + (1.0-r0)*(1.0-cosine).powi(5)  // powf and powi functions
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hitrecord: &Hitrecord, attenuation: &mut Color, scattered: &mut Ray) -> bool{
        *attenuation = Color::from_floats(1.0, 1.0, 1.0);
        let refraction_ratio = if hitrecord.front_face {1.0 / self.index_of_refraction} else {self.index_of_refraction};

        let unit_direction = unit_vector(ray_in.direction());

        let cos_theta = dot(&-unit_direction, &hitrecord.normal).min(1.0);
        let sin_theta = (1.0-cos_theta*cos_theta).sqrt();

        // total internal reflection
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;

        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > get_rand_f64(0.0, 1.0) {
            // must reflect
            direction = reflect(&unit_direction, &hitrecord.normal);
        } else {
            // can refract
            direction = refract(&unit_direction, &hitrecord.normal, refraction_ratio);
        }

        *scattered = Ray::from_values_time(&hitrecord.p, &direction, ray_in.time());

        return true;
    }

    fn clone_material(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}