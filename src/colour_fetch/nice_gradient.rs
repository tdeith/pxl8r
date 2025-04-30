use crate::common::{Colour, ColourBias};
use crate::image_state::ImageState;

use super::util;

pub struct NiceGradientRGHFetcher {
    bias: ColourBias,
}

impl NiceGradientRGHFetcher {
    pub fn new(bias: &ColourBias) -> Self {
        Self { bias: bias.clone() }
    }
    pub fn get_next(&mut self, try_near: &Colour, state: &mut ImageState) -> Option<Colour> {

        if let Some(found) = util::get_nearest_avail_colour_and_mark_shells(
            try_near, &self.bias, state) {
            state.consume_colour(&found);
            return Some(found);
        }
        None
    }
}
