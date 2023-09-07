mod aabb;
mod bvh;
mod camera;
mod color;
mod hittables;
mod interval;
mod material;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod traits;
mod utils;
mod vec3;
use std::env;
use texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};

use bvh::BvhNode;
use camera::Camera;
use color::write_color;
use color::Color;
use hittables::{HitRecord, Hittables};
use interval::Interval;
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;
use traits::{Hittable, Scatter};
use vec3::{Point3, Vec3};
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// A helper function to randomly pick a material
fn get_rand_material(choose_mat: f64) -> Rc<dyn Scatter> {
    match choose_mat {
        x if x < 0.8 => {
            // difuse
            let color = &Color::random(0.0, 1.0) * &Color::random(0.0, 1.0);
            Rc::new(Lambertian::new(Rc::new(SolidColor::new(color))))
        }
        x if x < 0.95 => {
            // matel
            let albedo = Color::random(0.5, 1.0);
            let fuzz = utils::random_f64_range(0.0, 0.5);
            Rc::new(Metal::new(Rc::new(SolidColor::new(albedo)), fuzz))
        }
        _ => {
            // glass
            Rc::new(Dielectric::new(1.5))
        }
    }
}

fn earth(fname: Option<String>) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/image_0.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg".to_string()));
    let earth_surface = Rc::new(Lambertian::new(earth_texture));

    let globe = Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface.clone(),
    ));

    let mut world = Hittables::default();
    world.add(globe);

    let image_width = 400;
    let mut camera = Camera::new(
        16.0 / 9.0,
        image_width, /* image width*/
        100,         /* sample per pixel */
        50,          /* max depth */
        20.0,        /* vfov */
    );
    camera.look_from = Point3::new(15.0, 5.0, 13.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.0;
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}

fn two_perlin_spheres(fname: Option<String>) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/image_0.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let mut world = Hittables::default();
    let pertext = Rc::new(NoiseTexture::default());
    let material_ground = Rc::new(Lambertian::new(pertext));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        material_ground.clone(),
    )));
    let image_width = 400;
    let mut camera = Camera::new(
        16.0 / 9.0,
        image_width, /* image width*/
        100,         /* sample per pixel */
        50,          /* max depth */
        20.0,        /* vfov */
    );
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.0;
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}
fn two_spheres(fname: Option<String>) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/image_0.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let mut world = Hittables::default();
    let checker = Rc::new(CheckerTexture::new_from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let material_ground = Rc::new(Lambertian::new(checker));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material_ground.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        material_ground.clone(),
    )));
    let image_width = 400;
    let mut camera = Camera::new(
        16.0 / 9.0,
        image_width, /* image width*/
        100,         /* sample per pixel */
        50,          /* max depth */
        20.0,        /* vfov */
    );
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.0;
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}
fn random_balls(fname: Option<String>) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/image_0.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let mut world = Hittables::default();
    // ground
    //meterial
    let checker = Rc::new(CheckerTexture::new_from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let material_ground = Rc::new(Lambertian::new(checker));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    )));

    // many small balls
    let count = 11;
    let p = Point3::new(4.0, 0.2, 0.0);
    for a in -count..count {
        for b in -count..count {
            let center = Point3::new(
                a as f64 + utils::random_f64(),
                0.2,
                b as f64 + utils::random_f64(),
            );
            if (&center - &p).length() > 0.9 {
                let choose_mat = utils::random_f64();
                let sphere_material = get_rand_material(choose_mat);
                if choose_mat < 0.8 {
                    let center1 = &center + Vec3::new(0.0, utils::random_f64_range(0.0, 0.5), 0.0);
                    world.add(Rc::new(Sphere::new_moving(
                        center,
                        center1,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else {
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }
    // a few big balls
    let material_1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1.clone(),
    )));
    let material_2 = Rc::new(Lambertian::new(Rc::new(SolidColor::new(Color::new(
        0.4, 0.2, 0.1,
    )))));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2.clone(),
    )));
    let material_3 = Rc::new(Metal::new(
        Rc::new(SolidColor::new(Color::new(0.7, 0.6, 0.5))),
        0.0,
    ));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3.clone(),
    )));
    let bvh = BvhNode::new_from_hittables(&world);
    let world = Hittables::new(Rc::new(bvh));

    let image_width = 400; // pixels
                           // camera
    let mut camera = Camera::new(
        16.0 / 9.0,
        image_width, /* image width*/
        100,         /* sample per pixel */
        50,          /* max depth */
        20.0,        /* vfov */
    );
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}
fn main() -> Result<()> {
    let case: u8 = env::args()
        .nth(1)
        .unwrap_or("1".to_string())
        .parse()
        .unwrap();
    match case {
        1 => random_balls(env::args().nth(2)),
        2 => two_spheres(env::args().nth(2)),
        3 => earth(env::args().nth(2)),
        4 => two_perlin_spheres(env::args().nth(2)),
        _ => {
            println!("not implemented");
            Ok(())
        }
    }
}
