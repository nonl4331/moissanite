use crate::prelude::*;

pub struct Camera {
    pub lower_left: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        up: Vec3,
        hfov: f32,
        focus_dist: f32,
        aspect_ratio: f32,
    ) -> Self {
        let forward = (look_at - origin).normalize();
        let up = up.normalize();

        let right_mag = focus_dist * 2.0 * (0.5 * hfov.to_radians()).tan();
        let up_mag = right_mag / aspect_ratio;

        let right = forward.cross(&up) * right_mag;
        let up = up * up_mag;

        let lower_left = origin - 0.5 * right - 0.5 * up + forward * focus_dist;

        Camera {
            origin,
            lower_left,
            right,
            up,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + self.right * u + self.up * (1.0 - v) - self.origin,
        )
    }
}
