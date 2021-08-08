//https://dev.to/rncrtr/multi-select-in-visual-studio-code-19k2


use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod Utils;
use Utils::*;

mod Collisions;
use Collisions::*;

mod Materials;
use Materials::*;

mod Textures;
use Textures::*;

mod ray;
use ray::*;

mod camera;
use camera::*;



use std::time::{Duration, Instant}; // for timing function calss


fn random_scene() -> HittableCollection {
    let mut world = HittableCollection::new();

    let checker_texture: Box<dyn Texture> = Box::new(CheckerTexture::from_colors(&Color::from_floats(0.2,0.3,0.1), &Color::from_floats(0.9,0.9,0.9), 5.0));
    let ground_material: Box<dyn Material> = Box::new(Lambertian::from_texture(&checker_texture));
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
                    let sphere_material: Box<dyn Material> = Box::new(Lambertian::from_color(&albedo));

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

    let material2: Box<dyn Material> = Box::new(Lambertian::from_color(&Color::from_floats(0.4, 0.2, 0.1)));
    world.add(
        Box::new(Sphere::from_values(&Vec3::from_ints(-4, 1, 0), 1.0, &material2))
    );

    let material3: Box<dyn Material> = Box::new(Metal::from_values(&Color::from_floats(0.7, 0.6, 0.5), 0.0));
    world.add(
        Box::new(Sphere::from_values(&Vec3::from_ints(4, 1, 0), 1.0, &material3))
    );


    return world;
}

fn earth_globe() -> HittableCollection {
    let mut world = HittableCollection::new();

    let texture: Box<dyn Texture> = Box::new(ImageTexture::from_name("./earthmap.jpg"));
    let material: Box<dyn Material> = Box::new(Lambertian::from_texture(&texture));

    world.add(
        Box::new(Sphere::from_values(&Vec3::new(), 2.0, &material))
    );

    let emmitter_material: Box<dyn Material> = Box::new(Lambertian::new());
    let diffuse_emmiter: Box<dyn Emmiter> = Box::new(DiffuseLight::from_color(&Color::from_ints(10,10,10)));

    world.add(
        Box::new(Sphere::as_emmiter(&Vec3::from_ints(0, 5, 0), 0.5, &emmitter_material, &diffuse_emmiter))
    );
    
    return world;
}


fn ray_color(ray: &Ray, background_color: Color, world: &HittableTrait, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new();
    }

    let mut hit_record = Hitrecord::new();
    if !world.hit(&ray, 0.001, 1000.0, &mut hit_record) {
        return background_color;
    }

    let mut scattered = Ray::new();
    let mut attenuation = Color::new();

    let emmited = match hit_record.clone().emmiter {
        Some(emm) => emm.emmit(hit_record.u, hit_record.v, &hit_record.p),
        None => Color::new(),
    };

    // use short circuit of || 
    // we stop if even if we hit an object, if that object has an attenuation of (0,0,0)
    if !hit_record.material.scatter(&ray, &hit_record, &mut attenuation, &mut scattered) || attenuation == Color::new() {
        return emmited;
    }

    return emmited + attenuation * ray_color(&scattered, background_color, world, depth-1);
    

    // blue background
    let unit_dir = unit_vector(ray.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);
    Color::from_floats(1.0, 1.0, 1.0) * (1.0 - t) + Color::from_floats(0.5, 0.7, 1.0) * (t)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMG_WIDTH: usize= 200;
    const IMG_HEIGHT: usize = ((IMG_WIDTH as f64) / ASPECT_RATIO) as usize;

    // array to save values into
    let mut img: Vec<Vec<String>> = vec![vec!["".to_string(); IMG_HEIGHT]; IMG_WIDTH];

    let mut text = String::from("P3\n"); // colors in ASCII
    let width_height_string = IMG_WIDTH.to_string() + " " + &IMG_HEIGHT.to_string(); // dimension
    text.push_str(&width_height_string);
    text.push_str("\n255\n"); // max color

    // World
    let world; //= BVH::bvh_from_collection(&mut scene, 0.0, 1.0); //random_scene();
    let mut scene: HittableCollection; 

    let mut look_from; //= Point3::from_ints(13,2,3);
    let mut look_at;   //= Point3::from_ints(0,0,0);
    let mut vfov = 40.0;
    let mut apertue = 0.0;
    let mut background_color;

    match 0 {
        1 => {
            scene = random_scene();
            look_from = Point3::from_ints(13,2,3);
            look_at = Point3::from_ints(0,0,0);
            vfov = 20.0;
            apertue = 0.1;
            background_color = Color::from_floats(0.7, 0.8, 1.0);
        },
        _ => {
            scene = earth_globe();
            look_from = Point3::from_ints(13,2,3);
            look_at = Point3::from_ints(0,0,0);
            vfov = 20.0;
            background_color = Color::from_floats(0.0, 0.0, 0.0);
        }
    }
    world = BVH::bvh_from_collection(&mut scene, 0.0, 1.0);


    // Camera
    let dist_to_focus = 10.0;

    let vup = Vec3::from_ints(0, 1, 0);

    let camera = Camera::from_values_time(
        &look_from,
        &look_at,
        &vup,
        vfov, 
        ASPECT_RATIO,
        apertue,
        dist_to_focus,
        0.0, 1.0
    );

    const SAMPLE_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 5;


    
    // render
    for j in (0..IMG_HEIGHT).rev() {
        println!("{} scanlines remaining", j);


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

                color = color + ray_color(&ray, background_color, &world, MAX_DEPTH);
            }
            color = color / (SAMPLE_PER_PIXEL as f64);

            write_color(&mut img, &color, i, j);
        }
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
