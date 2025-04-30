use crate::common::{Colour, ColourBias};
use crate::image_state::ImageState;
use super::util::ColourShellSearcher;

pub struct TreeStackRGBFetcher {
    seed_stack: Vec<Colour>,
    bias: ColourBias,
}

impl TreeStackRGBFetcher {
    pub fn new(first: &Colour, wander_weights: &ColourBias) -> TreeStackRGBFetcher {
        Self {
            seed_stack: vec![first.clone()],
            bias: wander_weights.clone(),
        }
    }

    pub fn get_next(&mut self, state: &mut ImageState) -> Option<Colour> {
        if let Some((seed, found)) = self.find_from_seed(state) {
            state.consume_colour(&found);
            if state.colour_has_neighbour(&seed) {
                self.seed_stack.push(seed);
            }
            self.seed_stack.push(found.clone());
            return Some(found);
        }
        None
    }

    fn wander(&self, seed: &Colour, image_state: &ImageState) -> Option<Colour> {
        let mut perimeter_wanderer = ColourShellSearcher::new(seed, &self.bias);

        while let Some(perimeter_candidates) = perimeter_wanderer.search_shell() {
            for next_try in perimeter_candidates.iter() {
                if image_state.colour_available(&next_try) {
                    return Some(next_try.clone());
                }
            }
        }
        None
    }

    fn find_from_seed(&mut self, state: &ImageState) -> Option<(Colour, Colour)> {
        while let Some(seed) = self.seed_stack.pop() {
            if let Some(found) = self.wander(&seed, state) {
                return Some((seed, found));
            }
        }
        None
    }
}
