use crate::prelude::*;
use rand::{thread_rng, Rng};

pub struct NaiveSpectral {}

const MAX_DEPTH: u64 = 50;
const RUSSIAN_ROULETTE_THRESHOLD: u64 = 3;

impl NaiveSpectral {
    pub fn radiance(ray: &mut Ray, _bvh: &Bvh, wavelength: f32, rng: &mut impl Rng) -> (f32, u64) {
        let (mut tp, mut out): (_, f32) = (1.0, 0.0);

        let mut depth = 0;

        while depth < MAX_DEPTH {
            depth += 1;
            //let tris = unsafe { bvh.traverse(ray, &TRIANGLES) };
            let tris = unsafe { TRIANGLES[..].iter().collect() };
            if let Some(int) = sort_intersections(ray, tris).get(0) {
                let mat = unsafe { &MATERIALS[int.mat] };

                let wo = ray.direction;

                let le = mat.spectral_radiance(int, wo, wavelength);

                out += le * tp;

                let exit = mat.scatter(int, ray, wavelength, rng);

                if exit {
                    break;
                }

                if !mat.delta_dist() {
                    tp *= mat.eval_li_spdf(int, wo, ray.direction, wavelength);
                } else {
                    unimplemented!()
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

fn sort_intersections(ray: &mut Ray, tris: Vec<&Triangle>) -> Vec<Intersection> {
    let mut tris: Vec<_> = tris.into_iter().filter_map(|v| v.intersect(ray)).collect();
    tris.sort_by(|a, b| float_cmp(a.t, b.t));
    tris
}
