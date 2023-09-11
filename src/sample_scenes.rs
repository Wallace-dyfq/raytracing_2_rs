use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::constant_medium::ConstantMedium;
use crate::hittables::Hittables;
use crate::hittables::RotateY;
use crate::hittables::Translate;
use crate::material::DiffuseLight;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::quad;
use crate::quad::Quad;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
use crate::traits::Material;
use crate::utils;
use crate::vec3::{Point3, Vec3};
use crate::Result;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

// A helper function to randomly pick a material
fn get_rand_material(choose_mat: f64) -> Arc<dyn Material> {
    match choose_mat {
        x if x < 0.8 => {
            // difuse
            let color = &Color::random(0.0, 1.0) * &Color::random(0.0, 1.0);
            Arc::new(Lambertian::new(Arc::new(SolidColor::new(color))))
        }
        x if x < 0.95 => {
            // matel
            let albedo = Color::random(0.5, 1.0);
            let fuzz = utils::random_f64_range(0.0, 0.5);
            Arc::new(Metal::new(Arc::new(SolidColor::new(albedo)), fuzz))
        }
        _ => {
            // glass
            Arc::new(Dielectric::new(1.5))
        }
    }
}

pub fn earth(fname: Option<String>) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/earth.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let earth_texture = Arc::new(ImageTexture::new("./resources/earthmap.jpg".to_string()));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));

    let globe = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface.clone(),
    ));

    let mut world = Hittables::default();
    world.add(globe);

    let image_width = 1200;
    let mut camera = Camera::new(
        16.0 / 9.0,
        image_width, /* image width*/
        500,         /* sample per pixel */
        50,          /* max depth */
        20.0,        /* vfov */
    );
    camera.look_from = Point3::new(15.0, 5.0, 13.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.background = Color::new(0.7, 0.8, 1.0);
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}

pub fn gen_quad(fname: Option<String>) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/quad.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let mut world = Hittables::default();

    // material
    let left_red = Arc::new(Lambertian::new_from_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new_from_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new_from_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new_from_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new_from_color(Color::new(0.2, 0.8, 0.8)));

    // quads
    let quad1 = Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    ));

    let quad2 = Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    let quad3 = Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    let quad4 = Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));
    let quad5 = Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    world.add(quad1);
    world.add(quad2);
    world.add(quad3);
    world.add(quad4);
    world.add(quad5);

    let image_width = 1200;
    let mut camera = Camera::new(
        1.0,         /* aspect_ratio */
        image_width, /* image width*/
        500,         /* sample per pixel */
        50,          /* max depth */
        80.0,        /* vfov */
    );
    camera.look_from = Point3::new(0.0, 0.0, 9.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.background = Color::new(0.7, 0.8, 1.0);
    //    let bvh = BvhNode::new_from_hittables(&world);
    //    let world = Hittables::new(Arc::new(bvh));
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}
pub fn two_perlin_spheres(fname: Option<String>) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/perlin_spheres.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let mut world = Hittables::default();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    let material_ground = Arc::new(Lambertian::new(pertext));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        material_ground.clone(),
    )));
    let image_width = 1200;
    let mut camera = Camera::new(
        16.0 / 9.0,
        image_width, /* image width*/
        500,         /* sample per pixel */
        50,          /* max depth */
        20.0,        /* vfov */
    );
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.background = Color::new(0.7, 0.8, 1.0);
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}
pub fn two_spheres(fname: Option<String>) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/two_spheres.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let mut world = Hittables::default();
    let checker = Arc::new(CheckerTexture::new_from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let material_ground = Arc::new(Lambertian::new(checker));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material_ground.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        material_ground.clone(),
    )));
    let image_width = 1200;
    let mut camera = Camera::new(
        16.0 / 9.0,
        image_width, /* image width*/
        500,         /* sample per pixel */
        50,          /* max depth */
        20.0,        /* vfov */
    );
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.background = Color::new(0.7, 0.8, 1.0);
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}
pub fn random_balls(fname: Option<String>) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/random_balls.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let mut world = Hittables::default();
    // ground
    //meterial
    let checker = Arc::new(CheckerTexture::new_from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let material_ground = Arc::new(Lambertian::new(checker));
    world.add(Arc::new(Sphere::new(
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
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center1,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else {
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }
    // a few big balls
    let material_1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1.clone(),
    )));
    let material_2 = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.4, 0.2, 0.1,
    )))));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2.clone(),
    )));
    let material_3 = Arc::new(Metal::new(
        Arc::new(SolidColor::new(Color::new(0.7, 0.6, 0.5))),
        0.0,
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3.clone(),
    )));
    let bvh = BvhNode::new_from_hittables(&world);
    let world = Hittables::new(Arc::new(bvh));

    let image_width = 1200; // pixels
                            // camera
    let mut camera = Camera::new(
        16.0 / 9.0,
        image_width, /* image width*/
        500,         /* sample per pixel */
        50,          /* max depth */
        20.0,        /* vfov */
    );
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    camera.background = Color::new(0.7, 0.8, 1.0);
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}

pub fn simple_light(fname: Option<String>) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/simple_light.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let mut world = Hittables::default();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    let background_ball = Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    ));
    let ball_1 = Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext.clone())),
    ));

    let difflight = Arc::new(DiffuseLight::new_from_color(Color::new(4.0, 4.0, 4.0)));
    let lightsource = Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    ));
    let ball_light = Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    ));
    world.add(background_ball);
    world.add(ball_1);
    world.add(lightsource);
    world.add(ball_light);

    let mut camera = get_default_camera(1200);

    camera.look_from = Point3::new(26.0, 3.0, 6.0);
    camera.look_at = Point3::new(0.0, 2.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.background = Color::new(0.0, 0.0, 0.0);
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}
pub fn cornell_smoke(fname: Option<String>) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/cornell_smoke.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let mut world = Hittables::default();

    let red = Arc::new(Lambertian::new_from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_from_color(Color::new(7.0, 7.0, 7.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = quad::create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));
    world.add(Arc::new(ConstantMedium::new_with_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    let box2 = quad::create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstantMedium::new_with_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    //    let bvh = BvhNode::new_from_hittables(&world);
    //    let world = Hittables::new(Arc::new(bvh));
    let image_width = 1200;
    let mut camera = Camera::new(
        1.0,
        image_width, /* image width*/
        500,         /* sample per pixel */
        50,          /* max depth */
        40.0,        /* vfov */
    );
    camera.look_from = Point3::new(278.0, 278.0, -800.0);
    camera.look_at = Point3::new(278.0, 278.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.background = Color::new(0.0, 0.0, 0.0);
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}
pub fn cornell_box(fname: Option<String>) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/cornell_box.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let mut world = Hittables::default();

    let red = Arc::new(Lambertian::new_from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 10.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = quad::create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);
    let box2 = quad::create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    //    let bvh = BvhNode::new_from_hittables(&world);
    //    let world = Hittables::new(Arc::new(bvh));
    let image_width = 1200;
    let mut camera = Camera::new(
        1.0,
        image_width, /* image width*/
        500,         /* sample per pixel */
        50,          /* max depth */
        40.0,        /* vfov */
    );
    camera.look_from = Point3::new(278.0, 278.0, -800.0);
    camera.look_at = Point3::new(278.0, 278.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.background = Color::new(0.0, 0.0, 0.0);
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}

// a sensible default camera
fn get_default_camera(image_width: u32) -> Camera {
    let mut camera = Camera::new(
        16.0 / 9.0,
        image_width, /* image width*/
        500,         /* sample per pixel */
        50,          /* max depth */
        20.0,        /* vfov */
    );
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.background = Color::new(0.7, 0.8, 1.0);
    camera
}
pub fn final_scene(
    fname: Option<String>,
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: i32,
) -> Result<()> {
    let output_fname = if let Some(fname) = fname {
        fname
    } else {
        "images/final_scene.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let mut world = Hittables::default();
    let mut boxes = Hittables::default();

    let ground = Arc::new(Lambertian::new_from_color(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    let w = 100.0;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = utils::random_range(1, 101);
            let z1 = z0 + w;
            boxes.add(quad::create_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1 as f64, z1),
                ground.clone(),
            ));
        }
    }

    let bvh_of_boxes = BvhNode::new_from_hittables(&boxes);
    world.add(Arc::new(bvh_of_boxes));

    let light = Arc::new(DiffuseLight::new_from_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light.clone(),
    )));

    // motion blur
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = &center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new_from_color(Color::new(0.70, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    // glass ball
    world.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    // metal ball
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new_from_color(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    // boundary
    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    // these two make it shiny
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new_with_color(
        boundary.clone(),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new_with_color(
        boundary.clone(),
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    // earth
    let emat = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
        "./resources/earthmap.jpg".to_string(),
    ))));

    world.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat.clone(),
    )));
    //noise ball, marbel-like
    let pertext = Arc::new(NoiseTexture::new(0.1));
    world.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));

    // box of white balls
    let mut boxes2 = Hittables::default();
    let white = Arc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new_from_hittables(&boxes2)),
            15.0,
        )),
        &Vec3::new(-100.0, 270.0, 395.0),
    )));

    let mut camera = Camera::new(
        1.0,
        image_width,       /* image width*/
        samples_per_pixel, /* sample per pixel */
        max_depth,         /* max depth */
        40.0,              /* vfov */
    );
    camera.look_from = Point3::new(478.0, 278.0, -600.0);
    camera.look_at = Point3::new(278.0, 278.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.background = Color::new(0.0, 0.0, 0.0);
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}
