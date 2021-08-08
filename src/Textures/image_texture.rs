extern crate image; // https://github.com/image-rs/image/blob/master/README.md
use image::io::Reader as ImageReader;
use image::RgbImage;

use crate::Utils::util::*;
use crate::Textures::*;

// only supports RBG Images
#[derive(Clone)]
pub struct ImageTexture {
    width: u32,
    height: u32,
    img: RgbImage,
}

impl ImageTexture {
    pub fn from_name(path: &str) -> ImageTexture{
        let img = image::open(path).unwrap().into_rgb8(); // From where it is run is local
        let (width, height) = img.dimensions();

        ImageTexture {
            width, height, img
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * (self.width as f64)) as u32;
        let mut j = (v * (self.height as f64)) as u32;

        if i >= self.width { i = self.width - 1; }
        if j >= self.height { j = self.height - 1; }

        let rgb_pixel = self.img.get_pixel(i, j);

        Color::from_floats(
            rgb_pixel[0] as f64 / 255.0, 
            rgb_pixel[1] as f64 / 255.0,
            rgb_pixel[2] as f64 / 255.0
        )
    }

    fn clone_texture(&self) -> Box<dyn Texture> {
        Box::new((*self).clone())
    }
}