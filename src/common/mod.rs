use bmp::Pixel;
use rand::random;

#[derive(Clone)]
pub struct Point {
    pub h: u32,
    pub w: u32,
}

impl Point {
    pub fn random(boundary: &Point) -> Self {
        let rand_h: u64 = random();
        let h = (rand_h % boundary.h as u64) as u32;

        let rand_w: u64 = random();
        let w = (rand_w % boundary.w as u64) as u32;

        Self { h, w }
    }
}

#[derive(Clone)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub fn decompose(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub fn random() -> Self {
        Self {
            r: random(),
            g: random(),
            b: random(),
        }
    }
}

impl Into<Pixel> for Colour {
    fn into(self) -> Pixel {
        let (r, g, b) = self.decompose();
        Pixel { r, g, b }
    }
}

impl From<Pixel> for Colour {
    fn from(p: Pixel) -> Self {
        Self {
            r: p.r,
            g: p.g,
            b: p.b,
        }
    }
}

#[derive(Clone)]
pub struct OOBColour {
    pub r: i16,
    pub g: i16,
    pub b: i16,
}

impl TryInto<Colour> for OOBColour {
    type Error = std::num::TryFromIntError;
    fn try_into(self) -> Result<Colour, Self::Error> {
        Ok(Colour {
            r: u8::try_from(self.r)?,
            g: u8::try_from(self.g)?,
            b: u8::try_from(self.b)?,
        })
    }
}

impl From<Colour> for OOBColour {
    fn from(colour: Colour) -> Self {
        Self {
            r: colour.r.into(),
            g: colour.g.into(),
            b: colour.b.into(),
        }
    }
}

#[derive(Clone)]
pub struct ColourBias {
    pub weight_r: f32,
    pub weight_g: f32,
    pub weight_b: f32,
}

impl ColourBias {
    pub fn random() -> Self {
        ColourBias {
            weight_r: rand::random(),
            weight_g: rand::random(),
            weight_b: rand::random(),
        }
    }
}
