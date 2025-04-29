use crate::common::{Colour, ColourBias};
use crate::imageState::ImageState;

use super::util::ColourWindowSearcher;

pub struct NiceGradientRGHFetcher {
    bias: ColourBias,
}

impl NiceGradientRGHFetcher {
    pub fn new(bias: &ColourBias) -> Self {
        Self { bias: bias.clone() }
    }
    pub fn get_next(&mut self, try_near: &Colour, state: &mut ImageState) -> Option<Colour> {
        if let Some(new) = self.wander(try_near, state) {
            state.consume_colour(&new);
            return Some(new);
        }
        None
    }

    fn wander(&self, try_near: &Colour, image_state: &ImageState) -> Option<Colour> {
        let mut perimeter_wanderer = ColourWindowSearcher::new(&try_near, &self.bias, 1);

        while let Some(perimeter_candidates) = perimeter_wanderer.walk_perimter() {
            for next_try in perimeter_candidates.iter() {
                if image_state.colour_available(&next_try) {
                    return Some(next_try.clone());
                }
            }
        }

        None
    }
}
