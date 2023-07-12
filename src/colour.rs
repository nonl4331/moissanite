use crate::prelude::*;

#[derive(Debug)]
pub struct PiecewiseGaussian {
    mean: f32,
    std_left_sq: f32,
    std_right_sq: f32,
}

impl PiecewiseGaussian {
    pub const fn new(mean: f32, std_left: f32, std_right: f32) -> Self {
        Self {
            mean,
            std_left_sq: std_left * std_left,
            std_right_sq: std_right * std_right,
        }
    }
    pub fn evaluate(&self, x: f32) -> f32 {
        let sq = x - self.mean;
        let sq = sq * sq;
        if x < self.mean {
            (-0.5 * sq / self.std_left_sq).exp()
        } else {
            (-0.5 * sq / self.std_right_sq).exp()
        }
    }
}

const A: PiecewiseGaussian = PiecewiseGaussian::new(599.8, 37.9, 31.0);
const B: PiecewiseGaussian = PiecewiseGaussian::new(442.0, 16.0, 26.7);
const C: PiecewiseGaussian = PiecewiseGaussian::new(501.1, 20.4, 26.2);
const D: PiecewiseGaussian = PiecewiseGaussian::new(568.8, 46.9, 40.5);
const E: PiecewiseGaussian = PiecewiseGaussian::new(530.9, 16.3, 31.1);
const F: PiecewiseGaussian = PiecewiseGaussian::new(437.0, 11.8, 36.0);
const G: PiecewiseGaussian = PiecewiseGaussian::new(459.0, 26.0, 13.8);

pub fn x_bar(x: f32) -> f32 {
    1.056 * A.evaluate(x) + 0.362 * B.evaluate(x) - 0.065 * C.evaluate(x)
}

pub fn y_bar(x: f32) -> f32 {
    0.821 * D.evaluate(x) + 0.286 * E.evaluate(x)
}

pub fn z_bar(x: f32) -> f32 {
    1.217 * F.evaluate(x) + 0.681 * G.evaluate(x)
}

pub fn xyz_to_rgb(xyz: Vec3) -> Vec3 {
    let r = 3.240479 * xyz.x - 1.53715 * xyz.y - 0.498535 * xyz.z;
    let g = -0.969256 * xyz.x + 1.875991 * xyz.y + 0.041556 * xyz.z;
    let b = 0.055648 * xyz.x - 0.204043 * xyz.y + 1.057311 * xyz.z;
    Vec3::new(r, g, b)
}

pub fn to_u32(rgb: Vec3) -> u32 {
    // TODO TONEMAPPING

    // gamma correction
    let r = rgb.x.powf(1.0 / 2.2);
    let g = rgb.y.powf(1.0 / 2.2);
    let b = rgb.z.powf(1.0 / 2.2);

    // convert to u32
    let r = ((r * 255.0) as u8) as u32;
    let g = ((g * 255.0) as u8) as u32;
    let b = ((b * 255.0) as u8) as u32;

    r << 16 | g << 8 | b
}
