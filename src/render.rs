use crate::{
    colour::{to_u32, x_bar, xyz_to_rgb, y_bar, z_bar},
    integrator::NaiveSpectral,
    material::WAVELENGTH_RANGE,
    prelude::*,
};
use indicatif::{ProgressBar, ProgressStyle};
use minifb::{Key, Window};
use rand::{thread_rng, Rng};
use rayon::prelude::*;

pub fn render(bvh: &Bvh, cam: &Camera, mut window: Window, max_samples: usize) {
    let mut screen_buffer = vec![0u32; WIDTH * HEIGHT];

    let (render_buffer, present_buffer) = (
        std::sync::Mutex::new(vec![Vec3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT]),
        std::sync::Mutex::new(vec![Vec3::zeros(); WIDTH * HEIGHT]),
    );

    let chunk_size = 10_000usize;

    let bar = ProgressBar::new(max_samples as u64).with_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap(),
    );

    for sample in 0..max_samples {
        if !window.is_open() || window.is_key_down(Key::Escape) {
            break;
        }
        {
            let start = std::time::Instant::now();
            let mut rbuffer = render_buffer.lock().unwrap();

            let frame_ray_count: u64 = rbuffer
                .par_chunks_mut(chunk_size)
                .enumerate()
                .map(|(chunk_i, chunk)| {
                    let mut chunk_ray_count = 0;
                    let chunk_offset = chunk_size * chunk_i;
                    for (pixel_i, pixel) in chunk.iter_mut().enumerate() {
                        let pixel_i = chunk_offset + pixel_i;
                        let (u, v) = (pixel_i % WIDTH, pixel_i / WIDTH);
                        let (u, v) = (
                            u as f32 / (WIDTH - 1) as f32,
                            v as f32 / (HEIGHT - 1) as f32,
                        );
                        let mut ray = cam.get_ray(u, v);
                        let mut rng = thread_rng();
                        let wavelength = rng.gen_range(380.0..750.0);
                        let (radiance, ray_count) =
                            NaiveSpectral::radiance(&mut ray, bvh, wavelength, &mut rng);

                        if radiance != 0.0 {
                            let xyz =
                                Vec3::new(x_bar(wavelength), y_bar(wavelength), z_bar(wavelength))
                                    * radiance
                                    * WAVELENGTH_RANGE;
                            let col = xyz_to_rgb(xyz);

                            *pixel += (col - *pixel) / (sample + 1) as f32;
                        } else {
                            *pixel += (Vec3::zeros() - *pixel) / (sample + 1) as f32;
                        }

                        chunk_ray_count += ray_count;
                    }
                    chunk_ray_count
                })
                .sum();

            let dur = start.elapsed();

            bar.set_position(sample as u64);
            bar.set_message(format!(
                "{:.2} MRay/s ({})",
                frame_ray_count as f64 * 0.000001 / dur.as_secs_f64(),
                dur.as_millis()
            ));
        }

        let mut rbuf = render_buffer.lock().unwrap();
        let mut pbuf = present_buffer.lock().unwrap();

        std::mem::swap(&mut rbuf, &mut pbuf);
        screen_buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = to_u32(pbuf[i]));

        window
            .update_with_buffer(&screen_buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    bar.finish_and_clear();

    while window.is_open() && !window.is_key_down(Key::Escape) {}
}
