use crate::Utils::util::*;
use crate::Textures::*;

#[derive(Clone)]
pub struct CheckerTexture {
    texture1: Box<dyn Texture>,
    texture2: Box<dyn Texture>,
    frequency: f64,
}

impl CheckerTexture {
    pub fn from_textures(texture1: &Box<dyn Texture>, texture2: &Box<dyn Texture>, frequency: f64) -> CheckerTexture{
        CheckerTexture {
            texture1: (*texture1).clone(),
            texture2: (*texture2).clone(),
            frequency,
        }
    }
    
    pub fn from_colors(color1: &Color, color2: &Color, frequency: f64) -> CheckerTexture {
        let texture1: Box<dyn Texture> = Box::new(SolidTexture::from_color(color1));
        let texture2: Box<dyn Texture> = Box::new(SolidTexture::from_color(color2));

        CheckerTexture::from_textures(&texture1, &texture2, frequency)
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (self.frequency*p.x()).sin() * (self.frequency*p.y()).sin() * (self.frequency*p.z()).sin();
        if sines < 0.0 {
            return self.texture1.value(u, v, p);
        } else{
            return self.texture2.value(u, v, p);
        }
    }

    fn clone_texture(&self) -> Box<dyn Texture> {
        Box::new((*self).clone())
    }
}