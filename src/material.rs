use crate::prelude::*;
use derive_new::new;
use rand::Rng;
use std::unreachable;

const MAX_WAVELENGTH: f32 = 750.0;
const MIN_WAVELENGTH: f32 = 380.0;
pub const WAVELENGTH_RANGE: f32 = MAX_WAVELENGTH - MIN_WAVELENGTH;
const BINS: usize = 16;
const INVERSE_INCREMENT: f32 = (BINS - 1) as f32 / WAVELENGTH_RANGE;

#[derive(Debug, new)]
pub enum Mat {
    SpectralPowerDistribution(SpectralPowerDistribution),
    SpectralReflectanceDistribution(SpectralReflectanceDistribution),
    SpectralRefract(SpectralRefract),
    Lambertian(Lambertian),
}

impl Mat {
    pub fn spectral_radiance(&self, int: &Intersection, wo: Vec3, wavelength: f32) -> f32 {
        match self {
            Mat::SpectralPowerDistribution(dist) => dist.spectral_radiance(int, wo, wavelength),
            _ => 0.0,
        }
    }

    pub fn scatter(
        &self,
        int: &Intersection,
        ray: &mut Ray,
        wavelength: f32,
        rng: &mut impl Rng,
    ) -> bool {
        match self {
            Mat::SpectralPowerDistribution(_) => true,
            Mat::Lambertian(_) | Mat::SpectralReflectanceDistribution(_) => {
                Lambertian::scatter(int, ray, rng)
            }
            Mat::SpectralRefract(mat) => mat.scatter(int, ray, wavelength, rng),
        }
    }

    pub fn eval_li_spdf(&self, _int: &Intersection, _wo: Vec3, _wi: Vec3, wavelength: f32) -> f32 {
        match self {
            Mat::SpectralPowerDistribution(_) => unreachable!(),
            Mat::Lambertian(l) => l.albedo,
            Mat::SpectralReflectanceDistribution(l) => l.albedo(wavelength),
            Mat::SpectralRefract(_) => unreachable!(),
        }
    }

    pub fn eval_li(&self, _int: &Intersection, _wo: Vec3, _wi: Vec3, _: f32) -> f32 {
        match self {
            Mat::SpectralRefract(_) => 1.0,
            _ => unreachable!(),
        }
    }
    pub fn delta_dist(&self) -> bool {
        matches!(self, Mat::SpectralRefract(_))
    }
}

// 380nm to 750nm
#[derive(Debug)]
pub struct SpectralPowerDistribution {
    irradiance: [f32; BINS],
}

impl SpectralPowerDistribution {
    pub fn new(irradiance: [f32; BINS]) -> Self {
        Self { irradiance }
    }
    pub fn new_with_scale(mut irradiance: [f32; BINS], scale: f32) -> Self {
        for bin in &mut irradiance {
            *bin *= scale;
        }
        Self { irradiance }
    }
    pub fn d65_illuminant(scale: f32) -> Self {
        let irradiance = [
            49.43, 86.31, 85.99, 116.70, 115.40, 108.27, 107.54, 104.0, 96.12, 90.29, 85.84, 80.50,
            80.92, 72.22, 66.23, 64.03,
        ];
        Self::new_with_scale(irradiance, scale)
    }
}

impl SpectralPowerDistribution {
    // asumme valid wavelength of from 380nm to 750nm
    pub fn spectral_radiance(&self, _: &Intersection, _: Vec3, wavelength: f32) -> f32 {
        let index = ((wavelength - MIN_WAVELENGTH) * INVERSE_INCREMENT) as usize;
        self.irradiance[index] // todo lerp
    }
}

#[derive(Debug, new)]
pub struct Lambertian {
    pub albedo: f32,
}

impl Lambertian {
    pub fn scatter(int: &Intersection, ray: &mut Ray, rng: &mut impl Rng) -> bool {
        *ray = Ray::new(
            int.pos
                + Vec3::new(
                    int.nor.x * int.err.x,
                    int.nor.y * int.err.y,
                    int.nor.z * int.err.z,
                ),
            (random_in_unit_sphere(rng) + int.nor).normalize(),
        );
        false
    }
}

fn random_vec3(rng: &mut impl Rng) -> Vec3 {
    Vec3::new(
        rng.gen::<f32>() - 0.5,
        rng.gen::<f32>() - 0.5,
        rng.gen::<f32>() - 0.5,
    )
}

fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p = random_vec3(rng);
        if p.magnitude() < 1.0 {
            return p;
        }
    }
}

// 380nm to 750nm
#[derive(Debug)]
pub struct SpectralReflectanceDistribution {
    reflectance: [f32; BINS],
}

impl SpectralReflectanceDistribution {
    pub fn new(reflectance: [f32; BINS]) -> Self {
        for bin in &reflectance {
            debug_assert!((0.0..1.0).contains(bin));
        }
        Self { reflectance }
    }
}

impl SpectralReflectanceDistribution {
    // asumme valid wavelength of from 380nm to 750nm
    pub fn albedo(&self, wavelength: f32) -> f32 {
        let index = ((wavelength - MIN_WAVELENGTH) * INVERSE_INCREMENT) as usize;
        self.reflectance[index] // todo lerp
    }
}

#[derive(Debug)]
pub struct SpectralRefract {
    ior: [f32; BINS],
}

impl SpectralRefract {
    pub fn new(ior: [f32; BINS]) -> Self {
        for ior_wl in &ior {
            debug_assert!(*ior_wl > 0.0);
        }
        Self { ior }
    }

    pub fn scatter(
        &self,
        int: &Intersection,
        ray: &mut Ray,
        wavelength: f32,
        rng: &mut impl Rng,
    ) -> bool {
        let index = ((wavelength - MIN_WAVELENGTH) * INVERSE_INCREMENT) as usize;
        let eta = self.ior[index];
        let mut eta_fraction = 1.0 / eta;
        if !int.out {
            eta_fraction = eta;
        }

        let nwo = -ray.dir;

        let cos_theta = (nwo.dot(&int.nor)).min(1.0);

        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = eta_fraction * sin_theta > 1.0;
        let f0 = (1.0 - eta_fraction) / (1.0 + eta_fraction);
        let f0 = f0 * f0;

        let (origin, dir);

        if cannot_refract || fresnel(cos_theta, f0) > rng.gen() {
            // reflect
            dir = utility::reflect_across_normal(nwo, int.nor);
            origin = utility::offset_ray(int.pos, int.nor, int.err, true);
        } else {
            // refract
            let perp = eta_fraction * (ray.dir + cos_theta * int.nor);
            let para = -1.0 * (1.0 - perp.magnitude()).abs().sqrt() * int.nor;
            dir = perp + para;
            origin = utility::offset_ray(int.pos, int.nor, int.err, false);
        }
        *ray = Ray::new(origin, dir);
        false
    }
}

pub fn fresnel(cos: f32, f0: f32) -> f32 {
    f0 + (1.0f32 - f0) * (1.0 - cos).powf(5.0)
}
