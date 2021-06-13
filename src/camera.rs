use crate::Utils::vec3::*;
use crate::Utils::util::*;
use crate::ray::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3, v: Vec3, w: Vec3,
    lens_radius: f64,
    time0: f64, time1: f64,
}

impl Camera {
    pub fn new(
        look_from: &Point3, look_at: &Point3, 
        vup: &Vec3, 
        vfoc: f64, 
        aspect_ratio: f64, 
        apertue: f64, 
        focus_dist: f64
    ) -> Camera {
        Camera::from_values_time(
            look_from, look_at,
            vup, vfoc,
            aspect_ratio,
            apertue,
            focus_dist,
            0.0, 0.0
        )
    }

    pub fn from_values_time(
        look_from: &Point3, look_at: &Point3, 
        vup: &Vec3, 
        vfoc: f64, 
        aspect_ratio: f64, 
        apertue: f64, 
        focus_dist: f64,
        time0: f64, time1: f64,
    ) -> Camera {
        let theta = degrees_to_radian(vfoc);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0*h;
        let viewport_width = aspect_ratio * viewport_height;

        //let focal_length = 1.0;

        let w = unit_vector(&(*look_from - *look_at));
        let u = unit_vector(&cross(vup, &w));
        let v = cross(&w, &u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = *origin - horizontal/2.0 - vertical/2.0 - w*focus_dist;

        Camera {
            origin: origin.clone(),
            lower_left_corner,
            horizontal,
            vertical,
            u, v, w,
            lens_radius: apertue/2.0,
            time0, time1
        }
    }

    pub fn get_ray(&self, s: f64, t:f64) -> Ray {
        let rd = Vec3::random_in_unitsphere() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::from_values_time(&(self.origin+offset), &(self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset), get_rand_f64(self.time0, self.time1))
    }
}