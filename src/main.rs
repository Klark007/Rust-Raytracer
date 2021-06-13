//use raylib::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod Utils;
use Utils::*;

mod Hittable;
use Hittable::*;

mod ray;
use ray::*;

mod camera;
use camera::*;

mod Materials;
use Materials::*;

use std::time::{Duration, Instant}; // for timing function calss


fn random_scene() -> HittableCollection {
    let mut world = HittableCollection::new();

    let ground_material: Box<dyn Material> = Box::new(Lambertian::from_values(&Color::from_floats(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::from_values(
        &Vec3::from_ints(0, -1000, 0),
        1000.0, 
        &ground_material
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = get_rand_f64_unit();
            let center = Vec3::from_floats(a as f64 + 0.9*get_rand_f64_unit(), 0.2, b as f64 + 0.9*get_rand_f64_unit());

            if (center - Point3::from_floats(4.0, 0.2, 0.0)).length() > 0.9 {

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let sphere_material: Box<dyn Material> = Box::new(Lambertian::from_values(&albedo));

                    let center2 = center + Vec3::from_floats(0.0, get_rand_f64(0.0, 0.5), 0.0);

                    world.add(
                        Box::new(MovingSphere::from_values(&center, &center2, 0.0, 1.0, 0.2, &sphere_material))
                    );
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = get_rand_f64(0.0, 0.5);

                    let sphere_material: Box<dyn Material> = Box::new(Metal::from_values(&albedo, fuzz));
                    world.add(
                        Box::new(Sphere::from_values(&center, 0.2, &sphere_material))
                    );
                } else {
                    // glass
                    let sphere_material: Box<dyn Material> = Box::new(Dielectric::from_values(1.5));
                    world.add(
                        Box::new(Sphere::from_values(&center, 0.2, &sphere_material))
                    );
                }
            }
        }
    }

    let material1: Box<dyn Material> = Box::new(Dielectric::from_values(1.5));
    world.add(
        Box::new(Sphere::from_values(&Vec3::from_ints(0, 1, 0), 1.0, &material1))
    );

    let material2: Box<dyn Material> = Box::new(Lambertian::from_values(&Color::from_floats(0.4, 0.2, 0.1)));
    world.add(
        Box::new(Sphere::from_values(&Vec3::from_ints(-4, 1, 0), 1.0, &material2))
    );

    let material3: Box<dyn Material> = Box::new(Metal::from_values(&Color::from_floats(0.7, 0.6, 0.5), 0.0));
    world.add(
        Box::new(Sphere::from_values(&Vec3::from_ints(4, 1, 0), 1.0, &material3))
    );


    return world;
}


fn ray_color(ray: &Ray, world: &HittableTrait, depth: i32, durations: &mut [Duration], iterations: &mut[u32]) -> Color {
    if depth <= 0 {
        return Color::new();
    }

    let mut hit_record = Hitrecord::new();
    let hit_start = Instant::now();
    if world.hit(&ray, 0.001, 1000.0, &mut hit_record) {
        durations[1] = durations[1] + hit_start.elapsed();
        iterations[1] += 1;

        let mut scattered = Ray::new();
        let mut attenuation = Color::new();

        let scatter_start = Instant::now();
        if hit_record.material.scatter(&ray, &hit_record, &mut attenuation, &mut scattered) {
            durations[2] = durations[2] + scatter_start.elapsed();
            iterations[2] += 1;

            return attenuation * ray_color(&scattered, world, depth-1, durations, iterations);
        } else {
            durations[2] = durations[2] + scatter_start.elapsed();
            iterations[2] += 1;
        }
        return Color::new();
    } else {
        durations[1] = durations[1] + hit_start.elapsed();
        iterations[1] += 1;
    }

    // blue background
    let unit_dir = unit_vector(ray.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);
    Color::from_floats(1.0, 1.0, 1.0) * (1.0 - t) + Color::from_floats(0.5, 0.7, 1.0) * (t)
}

fn main() {
    // world
    /*
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: i32= 400;
    const IMG_HEIGHT: i32 = ((IMG_WIDTH as f64) / ASPECT_RATIO) as i32;

    let mut world = HittableCollection::new();

    let material_ground: Box<dyn Material> = Box::new(Lambertian::from_values(&Color::from_floats(0.8, 0.8, 0.0)));
    let material_left: Box<dyn Material> = Box::new(Dielectric::from_values(1.5));
    let material_middle: Box<dyn Material> = Box::new(Lambertian::from_values(&Color::from_floats(0.2, 0.8, 0.1)));
    let material_right: Box<dyn Material> = Box::new(Metal::from_values(&Color::from_floats(0.7, 0.3, 0.3), 0.5));

    world.add(Box::new(Sphere::from_values(
        &Vec3::from_floats(0.0, -100.5, -1.0),
        100.0, 
        &material_ground
    )));

    // negative radius changes inside and outside
    world.add(Box::new(Sphere::from_values(
        &Vec3::from_ints(-1, 0, -1),
        0.5, 
        &material_left
    )));
    world.add(Box::new(Sphere::from_values(
        &Vec3::from_ints(-1, 0, -1),
        -0.4, 
        &material_left
    )));

    world.add(Box::new(Sphere::from_values(
        &Vec3::from_ints(1, 0, -1),
        0.5, 
        &material_right
    )));

    world.add(Box::new(Sphere::from_values(
        &Vec3::from_ints(0, 0, -1),
        0.5, 
        &material_middle
    )));
    
    
    let look_from = Point3::from_ints(3, 3, 2);
    let look_at   = Point3::from_ints(0, 0, -1);
    let vup = Vec3::from_ints(0, 1, 0);

    let apertue = 1.25;
    let dist_to_focus = (look_from-look_at).length();
    
    */

    /*let R = (std::f64::consts::PI /4.0).cos();

    let material_left: Box<dyn Material>  = Box::new(Lambertian::from_values(&Color::from_values(0.0, 0.0, 1.0)));
    let material_right: Box<dyn Material> = Box::new(Lambertian::from_values(&Color::from_values(1.0, 0.0, 0.0)));

    world.add(Box::new(Sphere::from_values(
        &Vec3::from_values(-R, 0.0, -1.0),
        R, 
        &material_left
    )));

    world.add(Box::new(Sphere::from_values(
        &Vec3::from_values(R, 0.0, -1.0),
        R, 
        &material_right
    )));*/

    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMG_WIDTH: usize= 400;
    const IMG_HEIGHT: usize = ((IMG_WIDTH as f64) / ASPECT_RATIO) as usize;

    // array to save values into
    let mut img: Vec<Vec<String>> = vec![vec!["".to_string(); IMG_HEIGHT]; IMG_WIDTH];

    const SAMPLE_PER_PIXEL: i32 = 75;
    const MAX_DEPTH: i32 = 50;


    let world = random_scene();

    let look_from = Point3::from_ints(13,2,3);
    let look_at   = Point3::from_ints(0,0,0);
    let vup = Vec3::from_ints(0, 1, 0);

    let apertue = 0.1;
    let dist_to_focus = 10.0;

    // Camera
    let camera = Camera::from_values_time(
        &look_from,
        &look_at,
        &vup,
        20.0, 
        ASPECT_RATIO,
        apertue,
        dist_to_focus,
        0.0, 1.0
    );

    
    let mut durations: [Duration; 5] = [Duration::new(0,0); 5];
    let mut iterations: [u32; 5] = [0; 5];
     
    let mut text = String::from("P3\n"); // colors in ASCII
    let width_height_string = IMG_WIDTH.to_string() + " " + &IMG_HEIGHT.to_string(); // dimension
    text.push_str(&width_height_string);
    text.push_str("\n255\n"); // max color


    
    // render
    for j in (0..IMG_HEIGHT).rev() {
        println!("{} scanlines remaining", j);

        /*if iterations[0] != 0 {
            println!("{:?} ray creation", durations[0] / iterations[0]);
        }
        if iterations[1] != 0 {
            println!("{:?} world intersection", durations[1] / iterations[1]);
        }
        if iterations[2] != 0 {
            println!("{:?} scattering", durations[2] / iterations[2]);
        }
        if iterations[3] != 0 {
            println!("{:?} s overall (per pixel)", (durations[3] / iterations[3]).as_secs_f64());
        }
        if iterations[4] != 0 {
            println!("{:?} s overall (per scanline)", (durations[4] / iterations[4]).as_secs_f64());
        }
        println!("");*/

        let start_scanline = Instant::now();
        for i in 0..IMG_WIDTH {
            /*let r = (i as f64) / ((img_width-1) as f64);
            let g = (j as f64) / ((img_height-1) as f64);
            let b = 0.25;

            let color = Color::from_values(r, g, b);
            write_color(&mut text, &color);*/
            let mut color = Color::new();

            let overall_start = Instant::now();
            for s in 0..SAMPLE_PER_PIXEL {
                let u = (i as f64 + get_rand_f64(0.0, 1.0)) / ((IMG_WIDTH-1) as f64);
                let v = (j as f64) / ((IMG_HEIGHT-1) as f64);

                // measure time for get ray
                let ray_start = Instant::now();
                let ray = camera.get_ray(u, v);
                durations[0] = durations[0] + ray_start.elapsed();
                iterations[0] += 1;

                color = color + ray_color(&ray, &world, MAX_DEPTH, &mut durations, &mut iterations);
            }
            color = color / (SAMPLE_PER_PIXEL as f64);

            write_color(&mut img, &color, i, j);
            
            durations[3] = durations[3] + overall_start.elapsed();
            iterations[3] += 1;
        }
        durations[4] = durations[4] + start_scanline.elapsed();
        iterations[4] += 1;
    }
    println!("Finished");

    // write from array to String
    for j in (0..IMG_HEIGHT).rev() {
        for i in 0..IMG_WIDTH {
            text.push_str(&img[i][j]);
        }
    }

    /*let v1 = Vec3::from_values(3.0, -3.0, 1.0);
    let v2 = Vec3::from_values(4.0, 9.0, 2.0);

    println!("{:?}", cross(v1, v2)); // :? for debug

    println!("{}", Point3::from_values(0.0, 1.0, 1.0).length());*/
    
    // write to file
    write_to_file(&text);
    
}

pub fn write_to_file(output: &String) {
    let path = Path::new("Image.ppm");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(output.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
