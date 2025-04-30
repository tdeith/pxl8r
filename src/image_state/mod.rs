use crate::common::{Colour, Point};

pub struct ImageState {
    points_used: Vec<Vec<bool>>,
    colours_map: Vec<Vec<Vec<u8>>>,
    size: Point,
}

impl ImageState {
    pub fn new(size: &Point) -> ImageState {
        Self {
            colours_map: vec![vec![vec![0; 256]; 256]; 256],
            points_used: vec![vec![false; size.h as usize]; size.w as usize],
            size: size.clone()
        }
    }

    pub fn point_used(&self, p: &Point) -> Option<bool> {
        self.points_used
            .get(p.w as usize)
            .and_then(|v| v.get(p.h as usize))
            .and_then(|u| Some(u.clone()))
    }

    pub fn point_available(&self, p: &Point) -> bool {
        if let Some(used) = self.point_used(p) {
            return !used;
        }
        false
    }

    pub fn point_has_neighbour(&self, candidate: &Point) -> bool {
        let (h, w) = (candidate.h, candidate.w);
        w > 0 && self.point_available(&Point { w: w - 1, h }) ||
        w < self.size.w - 1 && self.point_available(&Point { w: w + 1, h }) ||
        h > 0 && self.point_available(&Point { w, h: h - 1 }) ||
        h < self.size.h - 1 && self.point_available(&Point { w, h: h + 1 })
    }

    pub fn colour_search_dist(&self, c: &Colour) -> Option<u8> {
        self.colours_map
            .get(c.r as usize)
            .and_then(|v| v.get(c.g as usize))
            .and_then(|v| v.get(c.b as usize))
            .and_then(|u| Some(u.clone()))
    }

    pub fn colour_available(&self, c: &Colour) -> bool {
        if let Some(used) = self.colour_search_dist(c) {
            return used == 0;
        }
        false
    }

    pub fn colour_has_neighbour(&self, candidate: &Colour) -> bool {
        let (r, g, b) = candidate.decompose();
        r < 255 && self.colour_available(&Colour { r: r + 1, g, b }) ||
        r > 0 && self.colour_available(&Colour { r: r - 1, g, b }) ||
        g < 255 && self.colour_available(&Colour { r, g: g + 1, b }) ||
        g > 0 && self.colour_available(&Colour { r, g: g - 1, b }) ||
        b < 255 && self.colour_available(&Colour { r, g, b: b + 1 }) ||
        b > 0 && self.colour_available(&Colour { r, g, b: b - 1 })
    }

    pub fn consume_coord(&mut self, p: &Point) {
        self.points_used[p.w as usize][p.h as usize] = true;
    }

    pub fn consume_colour(&mut self, c: &Colour) {
        self.colours_map[c.r as usize][c.g as usize][c.b as usize] = 1;
    }

    pub fn mark_colour_dist(&mut self, c: &Colour, dist: u8) {
        self.colours_map[c.r as usize][c.g as usize][c.b as usize] = dist;
    }
}
