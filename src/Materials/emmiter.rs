use crate::Utils::util::*;

use crate::Materials::material::*;
use crate::Textures::texture::*;

pub trait Emmiter {
    fn emmit(&self, u: f64, v: f64, p: &Point3) -> Color;
    fn clone_emmiter(&self) -> Box<dyn Emmiter>;
}

impl Clone for Box<dyn Emmiter> {
    fn clone(&self) -> Box<dyn Emmiter> {
        self.clone_emmiter()
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    texture: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new() -> DiffuseLight {
        DiffuseLight::from_color(&Color::from_ints(0,0,0))
    }

    pub fn from_color(a: &Color) -> DiffuseLight {
        let tex: Box<Texture> = Box::new(SolidTexture::from_color(a));
        DiffuseLight::from_texture(&tex)
    }

    pub fn from_texture(t: &Box<dyn Texture>) -> DiffuseLight {
        DiffuseLight {
            texture: (*t).clone(),
        }
    }
}

impl Emmiter for DiffuseLight {
    fn emmit(&self, u: f64, v: f64, p: &Point3) -> Color{
        self.texture.value(u, v, p)
    }

    fn clone_emmiter(&self) -> Box<dyn Emmiter> {
        Box::new((*self).clone())
    }
}

