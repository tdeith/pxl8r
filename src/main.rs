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
    let mut seed_fetchers = vec![];
    let mut seed_weights = vec![];
    let mut sum_seed_weights = 0.0f32;

    loop {
        let depth_rand = rand::random::<f32>();
        let fifo_rand = rand::random::<f32>();
        let depth_coeff_pow = depth_rand * 6.0;
        let depth_coeff = (10.0f32).powf(-depth_coeff_pow);
        let fifo_rate = fifo_rand.powf(depth_coeff_pow);
        let this_seed_weight = rand::random::<f32>() + 0.25; // Seeds can be up to 5x more prolific than one-another

        let point = Point::random(&size);
        let colour = Colour::random();

        final_image.set_pixel(point.w, point.h, colour.clone().into());
        seed_fetchers.push(StackTreeCoordFetcher::new(&size, &point, depth_coeff, fifo_rate));
        seed_weights.push(this_seed_weight);

        eprintln!(
            "seed {}: rate: {:0.2}, depth: {:0.2}, fifo: {:0.2}, ({},{})=({},{},{})",
            seed_fetchers.len(), this_seed_weight, depth_rand, fifo_rand,
            point.w, point.h, colour.r, colour.g, colour.b,
        );

        if rand::random::<f32>() > 0.7 {
            break;
        }
    }

    let bias = ColourBias::random();
    let mut seed_colour = Colour::random();
    let mut grad_finder = NiceGradientRGHFetcher::new(&bias);
    // let mut colour_finder = TreeStackRGBFetcher::new(&seed_colour, &bias);

    let mut idx = 0;
    let mut jdx = 0;

    // while let Some((seed, next)) = fifo_coord_finder.get_next(&mut image_state) {
    while ! seed_fetchers.is_empty() {
        let fetcher_idx = get_idx_of_weight(&seed_weights);
        let (seed, next) = match seed_fetchers
            .get_mut(fetcher_idx)
            .and_then(|fetcher| fetcher.get_next(&mut image_state))
        {
            Some((seed, point)) => (seed, point),
            None => {
                seed_fetchers.remove(fetcher_idx);
                seed_weights.remove(fetcher_idx);
                continue;
            },
        };

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

fn get_idx_of_weight(weights: &Vec<f32>) -> usize {
    let mut idx = 0;
    let sum_weights = weights.iter().sum::<f32>();
    let mut rand_selection = rand::random::<f32>() * sum_weights;
    loop {
        rand_selection -= weights[idx];
        if rand_selection <= 0.0 {
            return idx;
        }
        idx += 1;
    }
}
