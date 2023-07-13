use crate::{
    colour::{inverse_pdf_wl, sample_wl, to_u32, x_bar, xyz_to_rgb, y_bar, z_bar},
    integrator::NaiveSpectral,
    prelude::*,
};
use indicatif::{ProgressBar, ProgressStyle};
use minifb::{Key, Window};
use rand::thread_rng;
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
        match handle_input(&mut window) {
            State::ReRender => {
                // investigate why this causes a memory leak
                bar.finish_and_clear();
                return render(bvh, cam, window, max_samples);
            }
            State::Exit => break,
            State::Continue => {}
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
                        let wavelength = sample_wl(&mut rng);
                        let (radiance, ray_count) =
                            NaiveSpectral::radiance(&mut ray, bvh, wavelength, &mut rng);

                        if radiance != 0.0 {
                            let radiance = radiance * inverse_pdf_wl(wavelength);
                            let xyz =
                                Vec3::new(x_bar(wavelength), y_bar(wavelength), z_bar(wavelength))
                                    * radiance;
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

#[allow(dead_code)]
pub fn render_no_window(bvh: &Bvh, cam: &Camera, max_samples: usize, filename: &str) {
    let mut render_buffer = vec![Vec3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];

    let chunk_size = 10_000usize;

    let bar = ProgressBar::new(max_samples as u64).with_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap(),
    );

    for sample in 0..max_samples {
        {
            let start = std::time::Instant::now();
            let rbuffer = &mut render_buffer;

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
                        let wavelength = sample_wl(&mut rng);
                        let (radiance, ray_count) =
                            NaiveSpectral::radiance(&mut ray, bvh, wavelength, &mut rng);

                        if radiance != 0.0 {
                            let radiance = radiance * inverse_pdf_wl(wavelength);
                            let xyz =
                                Vec3::new(x_bar(wavelength), y_bar(wavelength), z_bar(wavelength))
                                    * radiance;
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
    }

    let img = image::Rgb32FImage::from_vec(
        WIDTH as u32,
        HEIGHT as u32,
        render_buffer
            .iter()
            .flat_map(|v| [v.x, v.y, v.z])
            .collect::<Vec<f32>>(),
    )
    .unwrap();

    img.save(filename).unwrap();

    bar.finish_and_clear();
}

enum State {
    ReRender,
    Exit,
    Continue,
}

fn handle_input(window: &mut Window) -> State {
    let mut paused = false;

    if !window.is_open() || window.is_key_down(Key::Escape) {
        return State::Exit;
    } else if window.is_key_down(Key::P) {
        while window.is_key_down(Key::P) {
            window.update();
        }
        paused = true;
    }

    if paused {
        while window.is_open() && !window.is_key_down(Key::Escape) && paused {
            window.update();
            if window.is_key_down(Key::P) {
                while window.is_key_down(Key::P) {
                    window.update();
                }
                return State::Continue;
            }
            if window.is_key_down(Key::R) {
                return State::ReRender;
            }
        }
    }

    State::Continue
}
