use crate::Utils::vec3::*;
use crate::Utils::util::clamp;

pub fn write_color(output: &mut Vec<Vec<String>>, color: &Color, x: usize, y: usize) {
    let r = color.r();
    let g = color.g();
    let b = color.b();

    // gamma correction
    let r = (256.0 * (clamp((r as f64).sqrt(), 0.0, 0.999))) as i32;
    let g = (256.0 * (clamp((g as f64).sqrt(), 0.0, 0.999))) as i32;
    let b = (256.0 * (clamp((b as f64).sqrt(), 0.0, 0.999))) as i32;


    let rgb = r.to_string() +" "+ &g.to_string() +" "+ &b.to_string() + "\n";
    output[x][y] = rgb;
    //output.push_str(&rgb);
}