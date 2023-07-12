use crate::prelude::*;
use bvh::bounding_hierarchy::BHShape;
use derive_new::new;

#[derive(Debug, new)]
pub struct Triangle {
    pos: [usize; 3],
    nor: [usize; 3],
    mat: usize,
    node_index: usize,
}

use bvh::aabb::{Aabb, Bounded};

impl Bounded<f32, 3> for Triangle {
    fn aabb(&self) -> Aabb<f32, 3> {
        let a = unsafe { VERTICES[self.pos[0]] };
        let b = unsafe { VERTICES[self.pos[1]] };
        let c = unsafe { VERTICES[self.pos[2]] };

        let min_x = a.x.min(b.x).min(c.x);
        let min_y = a.y.min(b.y).min(c.y);
        let min_z = a.z.min(b.z).min(c.z);

        let max_x = a.x.max(b.x).max(c.x);
        let max_y = a.y.max(b.y).max(c.y);
        let max_z = a.z.max(b.z).max(c.z);

        let mut min = Vec3::new(min_x, min_y, min_z);
        let mut max = Vec3::new(max_x, max_y, max_z);
        let diff = max - min;
        if diff.x == 0.0 {
            max.x += 1e-5;
        }
        if diff.y == 0.0 {
            max.y += 1e-5;
        }
        if diff.z == 0.0 {
            max.z += 1e-5;
        }

        max += 1e-5 * diff;
        min -= 1e-5 * diff;

        Aabb::with_bounds(min.into(), max.into())
    }
}

impl BHShape<f32, 3> for Triangle {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

pub fn gamma(n: u32) -> f32 {
    let nm = n as f32 * 0.5 * f32::EPSILON;
    nm / (1.0 - nm)
}

impl Triangle {
    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let v0 = unsafe { VERTICES[self.pos[0]] };
        let v1 = unsafe { VERTICES[self.pos[1]] };
        let v2 = unsafe { VERTICES[self.pos[2]] };

        let ro: Vec3 = Vec3::new(ray.origin.x, ray.origin.y, ray.origin.z);

        let mut p0t: Vec3 = v0 - ro;
        let mut p1t: Vec3 = v1 - ro;
        let mut p2t: Vec3 = v2 - ro;

        let (x, y, z) = (
            ray.direction.x.abs(),
            ray.direction.y.abs(),
            ray.direction.z.abs(),
        );

        let max_axis = if x > y && x > z {
            0
        } else if y > z {
            1
        } else {
            2
        };

        let mut swaped_raydir = ray.direction;

        if max_axis == 0 {
            p0t = p0t.zyx();
            p1t = p1t.zyx();
            p2t = p2t.zyx();
            swaped_raydir = swaped_raydir.zyx();
        } else if max_axis == 1 {
            p0t = p0t.xzy();
            p1t = p1t.xzy();
            p2t = p2t.xzy();
            swaped_raydir = swaped_raydir.xzy();
        }

        let sz = 1.0 / swaped_raydir.z;
        let sx = -swaped_raydir.x * sz;
        let sy = -swaped_raydir.y * sz;

        p0t.x += sx * p0t.z;
        p0t.y += sy * p0t.z;
        p1t.x += sx * p1t.z;
        p1t.y += sy * p1t.z;
        p2t.x += sx * p2t.z;
        p2t.y += sy * p2t.z;

        let mut e0 = p1t.x * p2t.y - p1t.y * p2t.x;
        let mut e1 = p2t.x * p0t.y - p2t.y * p0t.x;
        let mut e2 = p0t.x * p1t.y - p0t.y * p1t.x;
        if e0 == 0.0 || e1 == 0.0 || e2 == 0.0 {
            e0 = (p1t.x as f64 * p2t.y as f64 - p1t.y as f64 * p2t.x as f64) as f32;
            e1 = (p2t.x as f64 * p0t.y as f64 - p2t.y as f64 * p0t.x as f64) as f32;
            e2 = (p0t.x as f64 * p1t.y as f64 - p0t.y as f64 * p1t.x as f64) as f32;
        }

        if (e0 < 0.0 || e1 < 0.0 || e2 < 0.0) && (e0 > 0.0 || e1 > 0.0 || e2 > 0.0) {
            return None;
        }

        let det = e0 + e1 + e2;
        if det == 0.0 {
            return None;
        }

        p0t *= sz;
        p1t *= sz;
        p2t *= sz;

        let t_scaled = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;
        if (det < 0.0 && t_scaled >= 0.0) || (det > 0.0 && t_scaled <= 0.0) {
            return None;
        }

        let inv_det = 1.0 / det;

        let b0 = e0 * inv_det;
        let b1 = e1 * inv_det;
        let b2 = e2 * inv_det;

        let t = inv_det * t_scaled;

        //let uv = b0 * Vec2::new(0.0, 0.0) + b1 * Vec2::new(1.0, 0.0) + b2 * Vec2::new(1.0, 1.0);

        let n0 = unsafe { NORMALS[self.nor[0]] };
        let n1 = unsafe { NORMALS[self.nor[1]] };
        let n2 = unsafe { NORMALS[self.nor[2]] };

        let normal = b0 * n0 + b1 * n1 + b2 * n2;

        let out = normal.dot(&ray.direction) < 0.0;

        let x_abs_sum = (b0 * v0.x).abs() + (b1 * v1.x).abs() + (b2 * v2.x).abs();
        let y_abs_sum = (b0 * v0.y).abs() + (b1 * v1.y).abs() + (b2 * v2.y).abs();
        let z_abs_sum = (b0 * v0.z).abs() + (b1 * v1.z).abs() + (b2 * v2.z).abs();

        let point_error = gamma(7) * Vec3::new(x_abs_sum, y_abs_sum, z_abs_sum)
            + gamma(6) * Vec3::new(b2 * v2.x, b2 * v2.y, b2 * v2.z);

        let point = b0 * v0 + b1 * v1 + b2 * v2;

        Some(Intersection::new(
            t,
            point,
            point_error,
            normal,
            out,
            self.mat,
        ))
    }
}
