mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittables;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod sample_scenes;
mod sphere;
mod texture;
mod traits;
mod utils;
mod vec3;

use color::write_color;
use color::Color;
use hittables::{HitRecord, Hittables};
use interval::Interval;
use ray::Ray;
use sample_scenes::*;
use std::env;
use traits::{Hittable, Material};
use vec3::{Point3, Vec3};
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

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
        5 => gen_quad(env::args().nth(2)),
        6 => simple_light(env::args().nth(2)),
        7 => cornell_box(env::args().nth(2)),
        8 => cornell_smoke(env::args().nth(2)),
        9 => final_scene(env::args().nth(2), 800, 10000, 40),
        _ => final_scene(env::args().nth(2), 400, 250, 4),
    }
}
