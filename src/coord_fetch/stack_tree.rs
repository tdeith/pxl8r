use rand::prelude::SliceRandom;
use crate::common::Point;
use crate::image_state::ImageState;

pub struct StackTreeCoordFetcher {
    size: Point,
    wander_stack: Vec<(Point, Point)>,
    depth_coeff: f32,
    fifo_rate: f32,
}

impl StackTreeCoordFetcher {
    pub fn new(size: &Point, first: &Point, depth_coeff: f32, fifo_rate: f32) -> StackTreeCoordFetcher {
        StackTreeCoordFetcher {
            wander_stack: vec![(first.clone(), first.clone())],
            size: size.clone(),
            depth_coeff,
            fifo_rate,
        }
    }
    fn random_idx(&mut self) -> Option<usize> {
        if self.wander_stack.is_empty() {
            return None;
        }
        let max_idx = self.wander_stack.len() - 1;
        let inset = (
            max_idx as f32 *
            self.depth_coeff * rand::random::<f32>()
        ).floor() as usize;

        let inset_idx = {
            if rand::random::<f32>() >= self.fifo_rate {
                max_idx - inset
            }
            else {
                inset
            }
        };
        Some(inset_idx)
    }

    fn wander(&self, seed: &Point, state: &ImageState) -> Vec<Point> {
        let mut tries: Vec<Point> = vec![];
        let (h, w) = (seed.h, seed.w);
        if w > 0 {
            tries.push(Point { w: w - 1, h });
        }
        if w < self.size.w - 1 {
            tries.push(Point { w: w + 1, h })
        }
        if h > 0 {
            tries.push(Point { w, h: h - 1 })
        }
        if h < self.size.h - 1 {
            tries.push(Point { w, h: h + 1 })
        }
        let mut rng = rand::rng();
        tries.shuffle(&mut rng);
        tries.iter().filter_map(|p| {
            if state.point_available(p) {
                Some(p.clone())
            } else {
                None
            }
        }).collect()
    }

    pub fn get_next(&mut self, state: &mut ImageState) -> Option<(Point, Point)> {
        while let Some(new_pick_idx) = self.random_idx() {
            let (seed, found) = {
                if let Some((s, p)) = self.wander_stack.get(new_pick_idx) {
                    (s.clone(), p.clone())
                } else {
                    continue;
                }
            };
            state.consume_coord(&found);
            state.consume_coord(&seed);
            let mut new_candidates = self.wander(&found, state);
            if let Some(first_candidate) = new_candidates.pop() {
                state.consume_coord(&first_candidate);
                self.wander_stack[new_pick_idx] = (found.clone(), first_candidate);
                for new_candidate in new_candidates {
                    state.consume_coord(&new_candidate);
                    self.wander_stack.push((found.clone(), new_candidate));
                }
            }
            else {
                self.wander_stack.remove(new_pick_idx);
            }
            return Some((seed, found));
        }
        None
    }
}