use crate::common::{Colour, ColourBias};

use super::util::ColourShellSearcher ;

pub struct BigQueueRGBFetcher {
    curr_queue: Vec<Colour>,
    perimeter_walker: ColourShellSearcher,
}

impl BigQueueRGBFetcher {
    pub fn new(first: &Colour, bias: &ColourBias) -> Self {
        Self {
            curr_queue: vec![],
            perimeter_walker: ColourShellSearcher::new(first, bias),
        }
    }

    pub fn get_next(&mut self) -> Option<Colour> {
        if self.curr_queue.is_empty() {
            if let Some(new_batch) = self.perimeter_walker.search_shell() {
                self.curr_queue = new_batch;
            }
        }
        self.curr_queue.pop()
    }
}

