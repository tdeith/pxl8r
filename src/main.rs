mod colour_fetch;
mod common;
mod coord_fetch;
mod image_state;

use std::env;
use std::str::FromStr;
use std::thread::sleep;

use bmp::Image;
// use image::PngEncoder;

use crate::colour_fetch::{BigQueueRGBFetcher, NiceGradientRGHFetcher, TreeStackRGBFetcher};
use crate::common::{Colour, ColourBias, Point};
use crate::coord_fetch::{FifoCoordFetcher, StackTreeCoordFetcher};
use crate::image_state::ImageState;

fn main() {
    let height = env::var("image_height")
        .map_err(|_| "Usage: image_width=X image_height=Y pxl8r")
        .and_then(|h| u32::from_str(&h).map_err(|_| "Usage: image_width=X image_height=Y pxl8r"))
        .unwrap_or(900);

    let width = env::var("image_width")
        .map_err(|_| "Usage: image_width=X image_height=Y pxl8r")
        .and_then(|h| u32::from_str(&h).map_err(|_| "Usage: image_width=X image_height=Y pxl8r"))
        .unwrap_or(1600);

    let mut final_image = Image::new(width as u32, height as u32);

    let size = Point {
        h: height,
        w: width,
    };

    let mut image_state = ImageState::new(&size);

    let seed_coord = Point::random(&size);
    let mut fifo_coord_finder = FifoCoordFetcher::new(&size, &seed_coord);

    let depth_rand = rand::random::<f32>();
    let fifo_rand = rand::random::<f32>();

    let depth_coeff_pow = depth_rand * 6.0;
    let depth_coeff = (10.0f32).powf(-depth_coeff_pow);

    let fifo_rate = fifo_rand.powf(depth_coeff_pow);

    eprintln!(
        "depth: {:0.2} ({}), fifo: {:0.2} ({})",
        depth_rand, depth_coeff, fifo_rand, fifo_rate
    );

    let mut stack_coord_finder =
        StackTreeCoordFetcher::new(&size, &seed_coord, depth_coeff, fifo_rate);

    let bias = ColourBias::random();
    let mut seed_colour = Colour::random();
    let mut grad_finder = NiceGradientRGHFetcher::new(&bias);
    // let mut colour_finder = TreeStackRGBFetcher::new(&seed_colour, &bias);

    final_image.set_pixel(seed_coord.w, seed_coord.h, seed_colour.into());

    let mut idx = 0;
    let mut jdx = 0;
    // while let Some((seed, next)) = fifo_coord_finder.get_next(&mut image_state) {
    while let Some((seed, next)) = stack_coord_finder.get_next(&mut image_state) {
        seed_colour = final_image.get_pixel(seed.w, seed.h).into();
        // let next_colour = colour_finder.get_next(&mut image_state).unwrap();
        // let next_colour = colour_finder.get_next(&mut image_state).unwrap();
        let next_colour = grad_finder
            .get_next(&seed_colour, &mut image_state)
            .unwrap();

        final_image.set_pixel(next.w, next.h, next_colour.into());

        idx += 1;
        if idx % 5 == 0 {
            // Slow down and batch a bit to avoid hurting our CPU
            sleep(std::time::Duration::from_millis(1));
        }
        if idx % 50000 == 0 {
            // Save image slowly over time
            save(&final_image, "/usr/share/backgrounds/last_generated.bmp").unwrap();
        }
    }
    save(&final_image, "/usr/share/backgrounds/last_generated.bmp").unwrap();
}

fn save(image: &Image, path: &str) -> Result<(), ()> {
    image.save(path).map_err(|_| ())
}
