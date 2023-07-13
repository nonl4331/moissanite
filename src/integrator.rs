use crate::prelude::*;
use core::ops::Range;
use rand::{thread_rng, Rng};

pub struct NaiveSpectral {}

const MAX_DEPTH: u64 = 50;
const RUSSIAN_ROULETTE_THRESHOLD: u64 = 6;

impl NaiveSpectral {
    pub fn radiance(ray: &mut Ray, bvh: &Bvh, wavelength: f32, rng: &mut impl Rng) -> (f32, u64) {
        let (mut tp, mut out): (_, f32) = (1.0, 0.0);

        let mut depth = 0;

        while depth < MAX_DEPTH {
            depth += 1;

            let ranges = bvh.traverse(ray);
            let ints = get_hits(ranges);

            if let Some(int) = sort_intersections(ray, ints).get(0) {
                let mat = unsafe { &MATERIALS[int.mat] };

                let wo = ray.dir;

                let le = mat.spectral_radiance(int, wo, wavelength);

                out += le * tp;

                let exit = mat.scatter(int, ray, wavelength, rng);

                if exit {
                    break;
                }

                if !mat.delta_dist() {
                    tp *= mat.eval_li_spdf(int, wo, ray.dir, wavelength);
                } else {
                    tp *= mat.eval_li(int, wo, ray.dir, wavelength);
                }

                if depth > RUSSIAN_ROULETTE_THRESHOLD {
                    let p = tp;
                    let mut rng = thread_rng();
                    if rng.gen::<f32>() > p {
                        break;
                    }
                    tp /= p;
                }
            } else {
                return (0.0, depth);
            }
        }
        if out.is_nan() {
            return (0.0, 0);
        }
        (out, depth)
    }
}

fn get_hits(ranges: Vec<Range<usize>>) -> Vec<&'static Triangle> {
    ranges
        .into_iter()
        .flat_map(|r| unsafe { &TRIANGLES[r] })
        .collect()
}

fn sort_intersections(ray: &Ray, tris: Vec<&Triangle>) -> Vec<Intersection> {
    let mut tris: Vec<_> = tris
        .into_iter()
        .filter_map(|v| v.intersect(ray))
        .filter(|v| v.t > 0.0)
        .collect();
    tris.sort_by(|a, b| utility::float_cmp(a.t, b.t));
    tris
}
