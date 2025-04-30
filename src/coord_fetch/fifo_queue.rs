use crate::common::Point;
use crate::image_state::ImageState;
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;

pub struct FifoCoordFetcher {
    size: Point,
    queue: Vec<(Point, Point)>,
    rand_seed: ThreadRng,
}

impl FifoCoordFetcher {
    pub fn new(size: &Point, first: &Point) -> Self {
        Self {
            size: size.clone(),
            queue: vec![(first.clone(), first.clone())],
            rand_seed: rand::rng(),
        }
    }

    pub fn get_next(&mut self, state: &mut ImageState) -> Option<(Point, Point)> {
        if self.queue.is_empty() {
            return None;
        }
        let idx_in = ((self.queue.len() - 1) as f32).powf(rand::random::<f32>()) as usize;
        if let Some((seed, next)) = self.queue.get(idx_in).and_then(|coord| Some(coord.clone())) {
            state.consume_coord(&next);

            let (w, h) = (next.w, next.h);

            let mut candidate_points: Vec<Point> = vec![];

            if w > 0 && state.point_available(&Point{h, w: w - 1}) {
                candidate_points.push(Point{h, w: w - 1});
            }
            if w < self.size.w - 1 && state.point_available(&Point{h, w: w + 1}) {
                candidate_points.push(Point{h, w: w + 1});
            }
            if h > 0 && state.point_available(&Point{w, h: h - 1}) {
                candidate_points.push(Point { w, h: h - 1 });
            }
            if h < self.size.w - 1 && state.point_available(&Point{w, h: h + 1}) {
                candidate_points.push(Point{w, h: h + 1});
            }
            candidate_points.shuffle(&mut self.rand_seed);
            for candidate in candidate_points {
                self.append(&next, &candidate, state);
            }
            self.queue.remove(idx_in);

            return Some((seed, next));
        }
        None
    }

    fn append(&mut self, seed: &Point, next: &Point, state: &mut ImageState) {
        state.consume_coord(next);
        self.queue.push((seed.clone(), next.clone()));
    }
}
