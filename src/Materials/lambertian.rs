use crate::Utils::util::*;

use crate::Materials::material::*;
use crate::Textures::texture::*;

use crate::Collisions::hittable::*;

pub struct Lambertian {
    texture: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new() -> Lambertian {
        Lambertian::from_color(&Color::from_floats(0.5, 0.5, 0.5))
    }

    pub fn from_color(a: &Color) -> Lambertian {
        let tex: Box<Texture> = Box::new(SolidTexture::from_color(a));
        Lambertian::from_texture(&tex)
    }

    pub fn from_texture(t: &Box<dyn Texture>) -> Lambertian {
        Lambertian {
            texture: (*t).clone()
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
        
        *attenuation = self.texture.value(hitrecord.u, hitrecord.v, &hitrecord.p);

        true
    }

    fn clone_material(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        Lambertian::from_texture(&self.texture)
    }
}