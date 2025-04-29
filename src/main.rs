mod rgbFetch;
mod coordFetch;
mod imageState;
mod common;

use std::env;
use std::str::FromStr;
use bmp::Image;
use crate::common::{Colour, ColourBias, Point};
use crate::imageState::ImageState;
use crate::rgbFetch::{BigQueueRGBFetcher, NiceGradientRGHFetcher, TreeStackRGBFetcher};
use crate::coordFetch::{FifoCoordFetcher, StackTreeCoordFetcher};

fn main() {

    let height = env::var("image_height")
        .map_err(|_| "Usage: image_width=X image_height=Y pxl8r")
        .and_then(|h| u32::from_str(&h).map_err(|_| "Usage: image_width=X image_height=Y pxl8r"))
        .unwrap_or(200);

    let width = env::var("image_width")
        .map_err(|_| "Usage: image_width=X image_height=Y pxl8r")
        .and_then(|h| u32::from_str(&h).map_err(|_| "Usage: image_width=X image_height=Y pxl8r"))
        .unwrap_or(200);

    let mut final_image = Image::new(width as u32, height as u32);

    let size = Point {
        h: height,
        w: width,
    };

    let mut image_state = ImageState::new(&size);

    let seed_coord = Point::random(&size);
    let mut fifo_coord_finder = FifoCoordFetcher::new(&size, &seed_coord);
    let mut stack_coord_finder = StackTreeCoordFetcher::new(&size, &seed_coord, 0.00005, 0);

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
        let next_colour = grad_finder.get_next(&seed_colour, &mut image_state).unwrap();

        final_image.set_pixel(next.w, next.h, next_colour.into());

        idx += 1;
        if idx % 100000 == 0 {
            final_image.save(format!("test_out.bmp")).unwrap();
            jdx += 1;
        }
    }
    final_image.save("test_out.bmp").unwrap();
}
