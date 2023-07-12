#![feature(const_fn_floating_point_arithmetic)]

mod camera;
mod colour;
mod cornell_box;
mod integrator;
mod load_obj;
mod material;
mod render;
mod triangle;

use crate::{cornell_box::cornell_box, prelude::*, render::render};
use derive_new::new;
use fern::colors::{Color, ColoredLevelConfig};
use minifb::*;

pub type Vec3 = nalgebra::Vector3<f32>;
pub type Ray = bvh::ray::Ray<f32, 3>;
pub type Vec2 = nalgebra::Vector2<f32>;
pub type Bvh = bvh::bvh::Bvh<f32, 3>;

pub const WIDTH: usize = 1080;
pub const HEIGHT: usize = 1080;

pub static mut VERTICES: Vec<Vec3> = vec![];
pub static mut NORMALS: Vec<Vec3> = vec![];
pub static mut MATERIALS: Vec<Mat> = vec![];
pub static mut TRIANGLES: Vec<Triangle> = vec![];

pub mod prelude {
    pub use super::{
        Bvh, Intersection, Ray, Vec2, Vec3, HEIGHT, MATERIALS, NORMALS, TRIANGLES, VERTICES, WIDTH,
    };
    pub use crate::{
        camera::Camera,
        float_cmp,
        material::{Mat, SpectralPowerDistribution, SpectralReflectanceDistribution},
        triangle::Triangle,
    };
}

#[derive(Debug, new)]
pub struct Intersection {
    pub t: f32,
    pub pos: Vec3,
    pub err: Vec3,
    pub nor: Vec3,
    pub out: bool,
    pub mat: usize,
}

fn main() {
    create_logger();

    load_triangles();

    let bvh = unsafe { Bvh::build(&mut TRIANGLES) };

    let camera = Camera::new(
        Vec3::new(0.0, -2.5, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        70.0,
        1.0,
        1.0,
    );
    let mut window = Window::new("minifbpt", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    window.limit_update_rate(Some(std::time::Duration::from_millis(16)));

    render(&bvh, &camera, window, 1000);
}

fn load_triangles() {
    unsafe {
        cornell_box(1.0);

        let no = NORMALS.len();
        let mo = MATERIALS.len();
        let vo = VERTICES.len();

        VERTICES.extend([
            Vec3::new(0.0, 0.5, 0.999),
            Vec3::new(-0.5, -0.5, 0.999),
            Vec3::new(0.5, -0.5, 0.999),
        ]);
        NORMALS.extend([Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)]);
        MATERIALS.extend([Mat::SpectralPowerDistribution(
            SpectralPowerDistribution::d65_illuminant(3e-4),
        )]);
        TRIANGLES.extend([Triangle::new([vo, 1 + vo, 2 + vo], [no, no, no], mo, 0)]);
    }
}

pub fn create_logger() {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Cyan)
        .debug(Color::Magenta);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} [{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("winit", log::LevelFilter::Warn)
        .chain(std::io::stderr())
        .apply()
        .unwrap();
}

use std::cmp::Ordering;

#[allow(clippy::float_cmp)]
pub fn float_cmp(a: f32, b: f32) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a == b {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}
