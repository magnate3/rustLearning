//Allocate memory for output image of given width and height with ImageBuffer::new. Rgb::from_channels calculates RGB pixel values. Create ThreadPool with thread count equal to number of cores with num_cpus::get. ThreadPool::execute receives each pixel as a separate job.

//mpsc::channel receives the jobs and Receiver::recv retrieves them. ImageBuffer::put_pixel uses the data to set the pixel color. ImageBuffer::save writes the image to output.png.
#[macro_use]
extern crate error_chain;
extern crate threadpool;
extern crate num;
extern crate num_cpus;
extern crate image;

use std::sync::mpsc::{channel, RecvError};
use threadpool::ThreadPool;
use num::complex::Complex;
use image::{ImageBuffer, Pixel, Rgb};

error_chain! {
    foreign_links {
        MpscRecv(RecvError);
        Io(std::io::Error);
    }
}

// Function converting intensity values to RGB
// Based on http://www.efg2.com/Lab/ScienceAndEngineering/Spectra.htm
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

    let (r, g, b) = (normalize(r, factor), normalize(g, factor), normalize(b, factor));
    Rgb::from_channels(r, g, b, 0)
}

// Maps Julia set distance estimation to intensity values
fn julia(c: Complex<f32>, x: u32, y: u32, width: u32, height: u32, max_iter: u32) -> u32 {
    let width = width as f32;
    let height = height as f32;

    let mut z = Complex {
        // scale and translate the point to image coordinates
        re: 3.0 * (x as f32 - 0.5 * width) / width,
        im: 2.0 * (y as f32 - 0.5 * height) / height,
    };

    let mut i = 0;
    for t in 0..max_iter {
        if z.norm() >= 2.0 {
            break;
        }
        z = z * z + c;
        i = t;
    }
    i
}

// Normalizes color intensity values within RGB range
fn normalize(color: f32, factor: f32) -> u8 {
    ((color * factor).powf(0.8) * 255.) as u8
}

fn main() -> Result<()> {
    let (width, height) = (1920, 1080);
    let mut img = ImageBuffer::new(width, height);
    let iterations = 300;

    let c = Complex::new(-0.8, 0.156);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || for x in 0..width {
                         let i = julia(c, x, y, width, height, iterations);
                         let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                         tx.send((x, y, pixel)).expect("Could not send data!");
                     });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x, y, pixel);
    }
    let _ = img.save("output.png")?;
    Ok(())
}

/*
cargo run -p thread
   Compiling thread v0.1.0 (/home/jay/b/wk/rust/books/learningrustcookbook/code/concurrency/thread)
error[E0277]: `?` couldn't convert the error to `Error`
  --> thread/src/main.rs:98:35
   |
98 |     let _ = img.save("output.png")?;
   |                                   ^ the trait `std::convert::From<image::ImageError>` is not implemented for `Error`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following implementations were found:
             <Error as std::convert::From<&'a str>>
             <Error as std::convert::From<ErrorKind>>
             <Error as std::convert::From<std::io::Error>>
             <Error as std::convert::From<std::string::String>>
             <Error as std::convert::From<std::sync::mpsc::RecvError>>
   = note: required by `std::convert::From::from`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `thread`.

To learn more, run the command again with --verbose.

*/
