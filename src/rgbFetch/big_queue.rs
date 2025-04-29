use crate::common::{Colour, ColourBias};

use super::util::ColourWindowSearcher;

pub struct BigQueueRGBFetcher {
    curr_queue: Vec<Colour>,
    perimeter_walker: ColourWindowSearcher,
}

impl BigQueueRGBFetcher {
    pub fn new(first: &Colour, bias: &ColourBias) -> Self {
        Self {
            curr_queue: vec![],
            perimeter_walker: ColourWindowSearcher::new(first, bias, 1),
        }
    }

    pub fn get_next(&mut self) -> Option<Colour> {
        if self.curr_queue.is_empty() {
            if let Some(new_batch) = self.perimeter_walker.walk_perimter() {
                self.curr_queue = new_batch;
            }
        }
        self.curr_queue.pop()
    }
}

