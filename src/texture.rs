use crate::traits::Texture;
use crate::Color;
use crate::{Interval, Point3};
use image::io::Reader as ImageReader;
use image::ImageFormat;
use image::{open, GenericImage, GenericImageView, ImageBuffer, Rgba};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Color::new(red, green, blue),
        }
    }
    pub fn new(color: Color) -> Self {
        Self { color_value: color }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.color_value.clone()
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Rc::new(SolidColor::new(c1)),
            odd: Rc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_int: i32 = (self.inv_scale * p.x().floor()) as i32;
        let y_int: i32 = (self.inv_scale * p.y().floor()) as i32;
        let z_int: i32 = (self.inv_scale * p.z().floor()) as i32;
        match (x_int + y_int + z_int) % 2 {
            0 => self.even.value(u, v, p),
            _ => self.odd.value(u, v, p),
        }
    }
}

pub struct ImageTexture {
    image: Rc<dyn GenericImageView<Pixel = Rgba<u8>>>,
}

impl ImageTexture {
    pub fn new(file_name: String) -> Self {
        let image = ImageReader::open(file_name).unwrap().decode().unwrap();
        Self {
            image: Rc::new(image),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let (width, height) = self.image.dimensions();
        if height <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let interval = Interval::new(0.0, 1.0);
        let u = interval.clamp(u);
        let v = 1.0 - interval.clamp(v);

        let i = (u * width as f64) as u32;
        let j = (v * height as f64) as u32;
        let pixel = self.image.get_pixel(i, j);
        let [r, g, b, _] = pixel.0;
        let r = r as f64 / 256.0;
        let g = g as f64 / 256.0;
        let b = b as f64 / 256.0;
        Color::new(r, g, b)
    }
}
