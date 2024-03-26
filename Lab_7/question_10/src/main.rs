#[macro_use]
extern crate error_chain;
extern crate image;
extern crate num;
extern crate num_cpus;
extern crate threadpool;

use image::{ImageBuffer, Rgb};
use num::complex::Complex;
use std::sync::mpsc::{channel, RecvError};
use threadpool::ThreadPool;

error_chain! {
    foreign_links {
        MpscRecv(RecvError);
        Io(std::io::Error);
    }
}

fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
    let wave = wavelength as f32;

    let (r, g, b) = match wavelength {
        380..=439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
        440..=489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
        490..=509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
        510..=579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
        580..=644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
        645..=780 => (1.0, 0.0, 0.0),
        _ => (0.0, 0.0, 0.0),
    };

    let factor = match wavelength {
        380..=419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
        701..=780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
        _ => 1.0,
    };

    let (r, g, b) = (
        normalize(r, factor),
        normalize(g, factor),
        normalize(b, factor),
    );
    Rgb([r, g, b])
}

fn mandelbrot(c: Complex<f32>, max_iter: u32) -> Option<u32> {
    let mut z = Complex { re: 0., im: 0. };
    for i in 0..max_iter {
        if z.norm() > 2. {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn normalize(color: f32, factor: f32) -> u8 {
    ((color * factor).powf(0.8) * 255.) as u8
}

fn main() -> Result<()> {
    let (width, height) = (1920, 1080);
    let mut img = ImageBuffer::new(width, height);
    let iterations = 500;

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || {
            for x in 0..width {
                let cx = x as f32 * 3.5 / width as f32 - 2.5;
                let cy = y as f32 * 2.0 / height as f32 - 1.0;
                let c = Complex::new(cx, cy);

                let mandelbrot_result = mandelbrot(c, iterations);
                let i = mandelbrot_result.unwrap_or(iterations);

                let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                tx.send((x, y, pixel)).expect("Could not send data!");
            }
        });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x, y, pixel);
    }
    let _ = img.save("output.png");
    Ok(())
}

