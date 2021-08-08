use crate::Utils::util::*;
use crate::Textures::*;

#[derive(Clone)]
pub struct UVTexture {

}

impl UVTexture {
    pub fn new() -> UVTexture {
        UVTexture {

        }
    }
}

impl Texture for UVTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let blue = Color::from_ints(0, 0, 1);
        let red = Color::from_ints(1, 0, 0);
        let green = Color::from_ints(0, 1, 0);
        let yellow = Color::from_ints(1, 1, 0);

        let mix_top = red * u + blue * (1.0 - u);
        let mix_bot = yellow * u + green * (1.0 - u);

        let result = mix_bot * v + mix_top * (1.0 - v);

        return result;
    }

    fn clone_texture(&self) -> Box<dyn Texture> {
        Box::new((*self).clone())
    }
}