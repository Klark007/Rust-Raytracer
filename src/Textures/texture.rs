use crate::Utils::util::*;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
    fn clone_texture(&self) -> Box<dyn Texture>;
}

#[derive(Clone)]
pub struct SolidTexture {
    color_value: Color
}

impl SolidTexture {
    pub fn from_color(color: &Color) -> SolidTexture {
        SolidTexture {
            color_value: *color
        }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> SolidTexture {
        SolidTexture::from_color(&Color::from_floats(r, g, b))
    }
}

impl Texture for SolidTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.color_value
    }

    fn clone_texture(&self) -> Box<dyn Texture> {
        Box::new((*self).clone())
    }
}

impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Box<dyn Texture> {
        self.clone_texture()
    }
}