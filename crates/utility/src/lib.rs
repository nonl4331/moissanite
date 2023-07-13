use std::cmp::Ordering;

pub type Vec3 = nalgebra::Vector3<f32>;
pub type Vec2 = nalgebra::Vector2<f32>;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub inv_dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self {
            origin,
            dir,
            inv_dir: Vec3::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z),
        }
    }
}

pub fn offset_ray(origin: Vec3, normal: Vec3, error: Vec3, is_brdf: bool) -> Vec3 {
    let offset_val = normal.abs().dot(&error);
    let mut offset = offset_val * normal;

    if !is_brdf {
        offset = -offset;
    }

    let mut new_origin = origin + offset;

    if offset.x > 0.0 {
        new_origin.x = next_float(new_origin.x);
    } else {
        new_origin.x = previous_float(new_origin.x);
    }

    if offset.y > 0.0 {
        new_origin.y = next_float(new_origin.y);
    } else {
        new_origin.y = previous_float(new_origin.y);
    }

    if offset.z > 0.0 {
        new_origin.z = next_float(new_origin.z);
    } else {
        new_origin.z = previous_float(new_origin.z);
    }

    new_origin
}

pub fn next_float(mut float: f32) -> f32 {
    if float.is_infinite() && float > 0.0 {
        return float;
    }

    if float == -0.0 {
        float = 0.0
    }

    f32::from_bits(if float >= 0.0 {
        f32::to_bits(float) + 1
    } else {
        f32::to_bits(float) - 1
    })
}

pub fn previous_float(mut float: f32) -> f32 {
    if float.is_infinite() && float < 0.0 {
        return float;
    }

    if float == 0.0 {
        float = -0.0
    }

    f32::from_bits(if float <= 0.0 {
        f32::to_bits(float) + 1
    } else {
        f32::to_bits(float) - 1
    })
}

pub fn gamma(n: u32) -> f32 {
    let nm = n as f32 * 0.5 * f32::EPSILON;
    nm / (1.0 - nm)
}

pub fn max_vec3(a: &Vec3, b: &Vec3) -> Vec3 {
    let max_x = a.x.max(b.x);
    let max_y = a.y.max(b.y);
    let max_z = a.z.max(b.z);

    Vec3::new(max_x, max_y, max_z)
}

pub fn min_vec3(a: &Vec3, b: &Vec3) -> Vec3 {
    let min_x = a.x.min(b.x);
    let min_y = a.y.min(b.y);
    let min_z = a.z.min(b.z);

    Vec3::new(min_x, min_y, min_z)
}

pub fn max_axis(a: &Vec3) -> usize {
    if a.x > a.y && a.x > a.z {
        0
    } else if a.y > a.z {
        1
    } else {
        2
    }
}

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

pub fn sort_by_indices<T>(vec: &mut [T], mut indices: Vec<usize>) {
    for index in 0..vec.len() {
        if indices[index] != index {
            let mut current_index = index;
            loop {
                let target_index = indices[current_index];
                indices[current_index] = current_index;
                if indices[target_index] == target_index {
                    break;
                }
                vec.swap(current_index, target_index);
                current_index = target_index;
            }
        }
    }
}

pub fn reflect_across_normal(val: Vec3, normal: Vec3) -> Vec3 {
    2.0 * val.dot(&normal) * normal - val
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sort_vec_by_indices() {
        let indices = vec![0, 4, 2, 1, 3];
        let mut values = ["a", "b", "c", "d", "e"];

        sort_by_indices(&mut values, indices);

        assert_eq!(values, ["a", "e", "c", "b", "d"]);
    }
}
