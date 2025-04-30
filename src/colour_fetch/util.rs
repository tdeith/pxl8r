use crate::common::{Colour, ColourBias, OOBColour};
use crate::image_state::ImageState;

pub struct ColourShellSearcher {
    pub center: Colour,
    pub bias: ColourBias,
    pub current_priority: ColourBias,
    pub current_radius: u8,
}

impl ColourShellSearcher {
    pub fn new(center: &Colour, bias: &ColourBias) -> Self {
        let this_bias = ColourBias {
            weight_r: bias.weight_r + rand::random::<f32>(),
            weight_g: bias.weight_g + rand::random::<f32>(),
            weight_b: bias.weight_b + rand::random::<f32>(),
        };
        Self {
            center: center.clone(),
            bias: this_bias.clone(),
            current_priority: this_bias.clone(),
            current_radius: 1,
        }
    }

    pub fn expand_shell(&mut self, new_radius: u8)
    {
        self.current_radius = new_radius.max(self.current_radius);
    }

    pub fn search_shell(&mut self) -> Option<Vec<Colour>> {
        if self.current_priority.weight_r < 0.0 &&
            self.current_priority.weight_g < 0.0 &&
            self.current_priority.weight_b < 0.0
        {
            self.current_priority.weight_r += 1.0;
            self.current_priority.weight_g += 1.0;
            self.current_priority.weight_b += 1.0;
            self.current_radius += 1;
        }

        let increment = self.current_radius as i16;
        let mut perimeter_iterator: Vec<i16> = {
            if rand::random() {
                (-increment..increment + 1).collect()
            } else {
                (-increment..increment + 1).rev().collect()
            }
        };
        let increment = {
            if rand::random() {
                -increment
            } else {
                increment
            }
        };
        let c: OOBColour = self.center.clone().into();
        let prios = self.current_priority.clone();

        let mut perimter_candidates = vec![];
        if (
            prios.weight_r > prios.weight_g &&
                prios.weight_r > prios.weight_b
        ) {
            self.current_priority.weight_r = self.current_priority.weight_r - 1.0;
            for g in perimeter_iterator.iter().map(|dg| c.g + dg) {
                for b in perimeter_iterator.iter().map(|db| c.b + db) {
                    perimter_candidates.push(OOBColour{r: c.r + increment, g, b});
                    perimter_candidates.push(OOBColour{r: c.r - increment, g, b});
                }
            }
        } else if (
            prios.weight_g > prios.weight_r &&
                prios.weight_g > prios.weight_b
        ) {
            self.current_priority.weight_g = self.current_priority.weight_g - 1.0;
            for b in perimeter_iterator.iter().map(|db| c.b + db) {
                for r in perimeter_iterator.iter().map(|dr| c.r + dr) {
                    perimter_candidates.push(OOBColour{r, g: c.g + increment, b});
                    perimter_candidates.push(OOBColour{r, g: c.g - increment, b});
                }
            }
        } else {
            self.current_priority.weight_b = self.current_priority.weight_b - 1.0;
            for r in perimeter_iterator.iter().map(|dr| c.r + dr) {
                for g in perimeter_iterator.iter().map(|dg| c.g + dg) {
                    perimter_candidates.push(OOBColour{r, g, b: c.b + increment});
                    perimter_candidates.push(OOBColour{r, g, b: c.b - increment});
                }
            }
        }
        let possibles: Vec<Colour> = perimter_candidates.iter()
            .filter_map(|candidate| candidate.clone().try_into().ok())
            .collect();

        if possibles.is_empty() {
            return None;
        }
        Some(possibles)
    }
}

pub fn get_nearest_avail_colour_and_mark_shells(
    seed: &Colour, bias: &ColourBias, state: &mut ImageState
) -> Option<Colour> {
    let mut shell_walker: ColourShellSearcher = ColourShellSearcher::new(seed, bias);

    let mut fetched_colour_dist: u8 = 0;
    let mut worst_dist_marker: u8 = 0;
    while let Some(possibles) = shell_walker.search_shell() {
        worst_dist_marker = 0;
        for possible in possibles {
            fetched_colour_dist = state.colour_search_dist(&possible)?;

            // Early return if we find an available colour
            if fetched_colour_dist == 0 {
                state.mark_colour_dist(seed, shell_walker.current_radius);
                return Some(possible);
            }

            worst_dist_marker = worst_dist_marker.max(fetched_colour_dist);
        }

        // Shell came back without a result. Update our shell size, re-search.
        if worst_dist_marker > shell_walker.current_radius {
            shell_walker.expand_shell(worst_dist_marker - shell_walker.current_radius);
        }
    }
    None
}
