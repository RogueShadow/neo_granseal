use crate::math::{angle_vec2, Vec2};
use crate::mesh::{FillStyle, MeshBuilder, Polygon};
use num_traits::AsPrimitive;
use rand::{Rng, SeedableRng};
use std::f32::consts::PI;
use std::ops::Mul;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
// Pre-defined colors from CSS colors.
impl Color {
    // misc
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    // red colors
    pub const LIGHT_SALMON: Self = Self::rgb(1.0, 160.0 / 255.0, 122.0 / 255.0);
    pub const SALMON: Self = Self::rgb(250.0 / 255.0, 0.5, 114.0 / 255.0);
    pub const DARK_SALMON: Self = Self::rgb(233.0 / 255.0, 150.0 / 255.0, 122.0 / 255.0);
    pub const LIGHT_CORAL: Self = Self::rgb(240.0 / 255.0, 0.5, 0.5);
    pub const INDIAN_RED: Self = Self::rgb(205.0 / 255.0, 92.0 / 255.0, 92.0 / 255.0);
    pub const CRIMSON: Self = Self::rgb(220.0 / 255.0, 20.0 / 255.0, 60.0 / 255.0);
    pub const FIRE_BRICK: Self = Self::rgb(178.0 / 255.0, 34.0 / 255.0, 34.0 / 255.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const DARK_RED: Self = Self::rgb(139.0 / 255.0, 0.0, 0.0);
    // orange colors
    pub const CORAL: Self = Self::rgb(1.0, 127.0 / 255.0, 80.0 / 255.0);
    pub const TOMATO: Self = Self::rgb(1.0, 99.0 / 255.0, 71.0 / 255.0);
    pub const ORANGE_RED: Self = Self::rgb(1.0, 69.0 / 255.0, 0.0);
    pub const GOLD: Self = Self::rgb(1.0, 215.0 / 255.0, 0.0);
    pub const ORANGE: Self = Self::rgb(1.0, 165.0 / 255.0, 0.0);
    pub const DARK_ORANGE: Self = Self::rgb(1.0, 140.0 / 255.0, 0.0);
    // yellow colors
    pub const LIGHT_YELLOW: Self = Self::rgb(1.0, 1.0, 224.0 / 255.0);
    pub const LEMON_CHIFFON: Self = Self::rgb(1.0, 250.0 / 255.0, 205.0 / 255.0);
    pub const LIGHT_GOLDENROD_YELLOW: Self = Self::rgb(250.0 / 255.0, 250.0 / 255.0, 210.0 / 255.0);
    pub const PAPAYA_WHIP: Self = Self::rgb(1.0, 239.0 / 255.0, 213.0 / 255.0);
    pub const MOCCASIN: Self = Self::rgb(1.0, 228.0 / 255.0, 181.0 / 255.0);
    pub const PEACH_PUFF: Self = Self::rgb(1.0, 218.0 / 255.0, 185.0 / 255.0);
    pub const PALE_GOLDENROD: Self = Self::rgb(238.0 / 255.0, 232.0 / 255.0, 170.0 / 255.0);
    pub const KHAKI: Self = Self::rgb(240.0 / 255.0, 230.0 / 255.0, 140.0 / 255.0);
    pub const DARK_KHAKI: Self = Self::rgb(189.0 / 255.0, 183.0 / 255.0, 107.0 / 255.0);
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);
    // green colors
    pub const LAWN_GREEN: Self = Self::rgb(124.0 / 255.0, 252.0 / 255.0, 0.0);
    pub const CHARTREUSE: Self = Self::rgb(127.0 / 255.0, 1.0, 0.0);
    pub const LIME_GREEN: Self = Self::rgb(50.0 / 255.0, 205.0 / 255.0, 50.0 / 255.0);
    pub const LIME: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const FOREST_GREEN: Self = Self::rgb(34.0 / 255.0, 139.0 / 255.0, 34.0 / 255.0);
    pub const GREEN: Self = Self::rgb(0.0, 128.0 / 255.0, 0.0);
    pub const DARK_GREEN: Self = Self::rgb(0.0, 100.0 / 255.0, 0.0);
    pub const GREEN_YELLOW: Self = Self::rgb(173.0 / 255.0, 1.0, 47.0 / 255.0);
    pub const YELLOW_GREEN: Self = Self::rgb(154.0 / 255.0, 205.0 / 255.0, 50.0 / 255.0);
    pub const SPRING_GREEN: Self = Self::rgb(0.0, 1.0, 127.0 / 255.0);
    pub const MEDIUM_SPRING_GREEN: Self = Self::rgb(0.0, 250.0 / 255.0, 154.0 / 255.0);
    pub const LIGHT_GREEN: Self = Self::rgb(144.0 / 255.0, 238.0 / 255.0, 144.0 / 255.0);
    pub const PALE_GREEN: Self = Self::rgb(152.0 / 255.0, 251.0 / 255.0, 152.0 / 255.0);
    pub const DARK_SEA_GREEN: Self = Self::rgb(143.0 / 255.0, 188.0 / 255.0, 143.0 / 255.0);
    pub const MEDIUM_SEA_GREEN: Self = Self::rgb(143.0 / 255.0, 188.0 / 255.0, 143.0 / 255.0);
    pub const SEA_GREEN: Self = Self::rgb(46.0 / 255.0, 139.0 / 255.0, 87.0 / 255.0);
    pub const OLIVE: Self = Self::rgb(128.0 / 255.0, 128.0 / 255.0, 0.0);
    pub const DARK_OLIVE_GREEN: Self = Self::rgb(85.0 / 255.0, 107.0 / 255.0, 47.0 / 255.0);
    pub const OLIVE_DRAB: Self = Self::rgb(107.0 / 255.0, 142.0 / 255.0, 35.0 / 255.0);
    // cyan colors
    pub const LIGHT_CYAN: Self = Self::rgb(224.0 / 255.0, 1.0, 1.0);
    pub const CYAN: Self = Self::rgb(0.0, 1.0, 1.0);
    pub const AQUA: Self = Self::rgb(0.0, 1.0, 1.0);
    pub const AQUAMARINE: Self = Self::rgb(127.0 / 255.0, 1.0, 212.0 / 255.0);
    pub const MEDIUM_AQUAMARINE: Self = Self::rgb(102.0 / 255.0, 205.0 / 255.0, 170.0 / 255.0);
    pub const PALE_TURQUOISE: Self = Self::rgb(175.0 / 255.0, 238.0 / 255.0, 238.0 / 255.0);
    pub const TURQUOISE: Self = Self::rgb(64.0 / 255.0, 224.0 / 255.0, 208.0 / 255.0);
    pub const MEDIUM_TURQUOISE: Self = Self::rgb(72.0 / 255.0, 209.0 / 255.0, 204.0 / 255.0);
    pub const DARK_TURQUOISE: Self = Self::rgb(0.0, 206.0 / 255.0, 209.0 / 255.0);
    pub const LIGHT_SEA_GREEN: Self = Self::rgb(32.0 / 255.0, 178.0 / 255.0, 170.0 / 255.0);
    pub const CADET_BLUE: Self = Self::rgb(95.0 / 255.0, 158.0 / 255.0, 160.0 / 255.0);
    pub const DARK_CYAN: Self = Self::rgb(0.0, 139.0 / 255.0, 139.0 / 255.0);
    pub const TEAL: Self = Self::rgb(0.0, 128.0 / 255.0, 128.0 / 255.0);
    // blue colors
    pub const POWDER_BLUE: Self = Self::rgb(176.0 / 255.0, 224.0 / 255.0, 230.0 / 255.0);
    pub const LIGHT_BLUE: Self = Self::rgb(173.0 / 255.0, 216.0 / 255.0, 230.0 / 255.0);
    pub const LIGHT_SKY_BLUE: Self = Self::rgb(135.0 / 255.0, 206.0 / 255.0, 250.0 / 255.0);
    pub const SKY_BLUE: Self = Self::rgb(135.0 / 255.0, 206.0 / 255.0, 235.0 / 255.0);
    pub const DEEP_SKY_BLUE: Self = Self::rgb(0.0, 191.0 / 255.0, 1.0);
    pub const LIGHT_STEEL_BLUE: Self = Self::rgb(176.0 / 255.0, 196.0 / 255.0, 222.0 / 255.0);
    pub const DODGER_BLUE: Self = Self::rgb(30.0 / 255.0, 144.0 / 255.0, 1.0);
    pub const CORNFLOWER_BLUE: Self = Self::rgb(100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0);
    pub const STEEL_BLUE: Self = Self::rgb(70.0 / 255.0, 130.0 / 255.0, 180.0 / 255.0);
    pub const ROYAL_BLUE: Self = Self::rgb(65.0 / 255.0, 105.0 / 255.0, 225.0 / 255.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    pub const MEDIUM_BLUE: Self = Self::rgb(0.0, 0.0, 205.0 / 255.0);
    pub const DARK_BLUE: Self = Self::rgb(0.0, 0.0, 139.0 / 255.0);
    pub const NAVY: Self = Self::rgb(0.0, 0.0, 128.0 / 255.0);
    pub const MIDNIGHT_BLUE: Self = Self::rgb(25.0 / 255.0, 25.0 / 255.0, 112.0 / 255.0);
    pub const MEDIUM_SLATE_BLUE: Self = Self::rgb(123.0 / 255.0, 104.0 / 255.0, 238.0 / 255.0);
    pub const SLATE_BLUE: Self = Self::rgb(106.0 / 255.0, 90.0 / 255.0, 205.0 / 255.0);
    pub const DARK_SLATE_BLUE: Self = Self::rgb(72.0 / 255.0, 61.0 / 255.0, 139.0 / 255.0);
    // purple colors
    pub const LAVENDER: Self = Self::rgb(230.0 / 255.0, 230.0 / 255.0, 250.0 / 255.0);
    pub const THISTLE: Self = Self::rgb(216.0 / 255.0, 191.0 / 255.0, 216.0 / 255.0);
    pub const PLUM: Self = Self::rgb(221.0 / 255.0, 160.0 / 255.0, 221.0 / 255.0);
    pub const VIOLET: Self = Self::rgb(238.0 / 255.0, 130.0 / 255.0, 238.0 / 255.0);
    pub const ORCHID: Self = Self::rgb(218.0 / 255.0, 112.0 / 255.0, 214.0 / 255.0);
    pub const FUCHSIA: Self = Self::rgb(1.0, 0.0, 1.0);
    pub const MAGENTA: Self = Self::rgb(1.0, 0.0, 1.0);
    pub const MEDIUM_ORCHID: Self = Self::rgb(186.0 / 255.0, 85.0 / 255.0, 211.0 / 255.0);
    pub const MEDIUM_PURPLE: Self = Self::rgb(147.0 / 255.0, 112.0 / 255.0, 219.0 / 255.0);
    pub const BLUE_VIOLET: Self = Self::rgb(138.0 / 255.0, 43.0 / 255.0, 226.0 / 255.0);
    pub const DARK_VIOLET: Self = Self::rgb(148.0 / 255.0, 0.0, 211.0 / 255.0);
    pub const DARK_ORCHID: Self = Self::rgb(153.0 / 255.0, 50.0 / 255.0, 204.0 / 255.0);
    pub const DARK_MAGENTA: Self = Self::rgb(139.0 / 255.0, 0.0, 139.0 / 255.0);
    pub const PURPLE: Self = Self::rgb(128.0 / 255.0, 0.0, 128.0 / 255.0);
    pub const INDIGO: Self = Self::rgb(75.0 / 255.0, 0.0, 130.0 / 255.0);
    // pink colors
    pub const PINK: Self = Self::rgb(1.0, 192.0 / 255.0, 203.0 / 255.0);
    pub const LIGHT_PINK: Self = Self::rgb(1.0, 182.0 / 255.0, 193.0 / 255.0);
    pub const HOT_PINK: Self = Self::rgb(1.0, 105.0 / 255.0, 180.0 / 255.0);
    pub const DEEP_PINK: Self = Self::rgb(1.0, 20.0 / 255.0, 147.0 / 255.0);
    pub const PALE_VIOLET_RED: Self = Self::rgb(219.0 / 255.0, 112.0 / 255.0, 147.0 / 255.0);
    pub const MEDIUM_VIOLET_RED: Self = Self::rgb(199.0 / 255.0, 21.0 / 255.0, 133.0 / 255.0);
    // white colors
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const SNOW: Self = Self::rgb(1.0, 250.0 / 255.0, 250.0 / 255.0);
    pub const HONEYDEW: Self = Self::rgb(240.0 / 255.0, 1.0, 240.0 / 255.0);
    pub const MINT_CREAM: Self = Self::rgb(245.0 / 255.0, 1.0, 250.0 / 255.0);
    pub const AZURE: Self = Self::rgb(240.0 / 255.0, 1.0, 1.0);
    pub const ALICE_BLUE: Self = Self::rgb(240.0 / 255.0, 248.0 / 255.0, 1.0);
    pub const GHOST_WHITE: Self = Self::rgb(248.0 / 255.0, 248.0 / 255.0, 1.0);
    pub const WHITE_SMOKE: Self = Self::rgb(245.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0);
    pub const SEASHELL: Self = Self::rgb(1.0, 245.0 / 255.0, 238.0 / 255.0);
    pub const BEIGE: Self = Self::rgb(245.0 / 255.0, 245.0 / 255.0, 220.0 / 255.0);
    pub const OLD_LACE: Self = Self::rgb(253.0 / 255.0, 245.0 / 255.0, 230.0 / 255.0);
    pub const FLORAL_WHITE: Self = Self::rgb(1.0, 250.0 / 255.0, 240.0 / 255.0);
    pub const IVORY: Self = Self::rgb(1.0, 1.0, 240.0 / 255.0);
    pub const ANTIQUE_WHITE: Self = Self::rgb(250.0 / 255.0, 235.0 / 255.0, 215.0 / 255.0);
    pub const LINEN: Self = Self::rgb(250.0 / 255.0, 240.0 / 255.0, 230.0 / 255.0);
    pub const LAVENDER_BLUSH: Self = Self::rgb(1.0, 240.0 / 255.0, 245.0 / 255.0);
    pub const MISTY_ROSE: Self = Self::rgb(1.0, 228.0 / 255.0, 225.0 / 255.0);
    // gray colors
    pub const GAINSBORO: Self = Self::rgb(220.0 / 255.0, 220.0 / 255.0, 220.0 / 255.0);
    pub const LIGHT_GRAY: Self = Self::rgb(211.0 / 255.0, 211.0 / 255.0, 211.0 / 255.0);
    pub const SILVER: Self = Self::rgb(192.0 / 255.0, 192.0 / 255.0, 192.0 / 255.0);
    pub const DARK_GRAY: Self = Self::rgb(169.0 / 255.0, 169.0 / 255.0, 169.0 / 255.0);
    pub const GRAY: Self = Self::rgb(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0);
    pub const DIM_GRAY: Self = Self::rgb(105.0 / 255.0, 105.0 / 255.0, 105.0 / 255.0);
    pub const LIGHT_SLATE_GRAY: Self = Self::rgb(119.0 / 255.0, 136.0 / 255.0, 153.0 / 255.0);
    pub const SLATE_GRAY: Self = Self::rgb(112.0 / 255.0, 128.0 / 255.0, 144.0 / 255.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    // brown colors
    pub const CORN_SILK: Self = Self::rgb(1.0, 248.0 / 255.0, 220.0 / 255.0);
    pub const BLANCHED_ALMOND: Self = Self::rgb(1.0, 235.0 / 255.0, 205.0 / 255.0);
    pub const BISQUE: Self = Self::rgb(1.0, 228.0 / 255.0, 196.0 / 255.0);
    pub const NAVAJO_WHITE: Self = Self::rgb(1.0, 222.0 / 255.0, 173.0 / 255.0);
    pub const WHEAT: Self = Self::rgb(245.0 / 255.0, 222.0 / 255.0, 179.0 / 255.0);
    pub const BURLY_WOOD: Self = Self::rgb(222.0 / 255.0, 184.0 / 255.0, 135.0 / 255.0);
    pub const TAN: Self = Self::rgb(210.0 / 255.0, 180.0 / 255.0, 140.0 / 255.0);
    pub const ROSY_BROWN: Self = Self::rgb(188.0 / 255.0, 143.0 / 255.0, 143.0 / 255.0);
    pub const SANDY_BROWN: Self = Self::rgb(188.0 / 255.0, 164.0 / 255.0, 96.0 / 255.0);
    pub const GOLDENROD: Self = Self::rgb(218.0 / 255.0, 165.0 / 255.0, 32.0 / 255.0);
    pub const PERU: Self = Self::rgb(205.0 / 255.0, 133.0 / 255.0, 63.0 / 255.0);
    pub const CHOCOLATE: Self = Self::rgb(210.0 / 255.0, 105.0 / 255.0, 30.0 / 255.0);
    pub const SADDLE_BROWN: Self = Self::rgb(139.0 / 255.0, 69.0 / 255.0, 19.0 / 255.0);
    pub const SIENNA: Self = Self::rgb(160.0 / 255.0, 82.0 / 255.0, 45.0 / 255.0);
    pub const BROWN: Self = Self::rgb(165.0 / 255.0, 42.0 / 255.0, 42.0 / 255.0);
    pub const MAROON: Self = Self::rgb(128.0 / 255.0, 0.0, 0.0);
}

impl Color {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub fn rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self::new(
            r as f32 / u8::MAX as f32,
            g as f32 / u8::MAX as f32,
            b as f32 / u8::MAX as f32,
            1.0,
        )
    }
    pub fn invert(mut self) -> Self {
        self.r = 1.0 - self.r;
        self.g = 1.0 - self.g;
        self.b = 1.0 - self.b;
        self
    }
    pub fn adjust(mut self, v: f32) -> Self {
        self.r = (self.r + v).clamp(0.0, 1.0);
        self.g = (self.g + v).clamp(0.0, 1.0);
        self.b = (self.b + v).clamp(0.0, 1.0);
        self
    }
    pub fn random() -> Self {
        let mut r = rand_xorshift::XorShiftRng::from_entropy();
        Self {
            r: r.gen::<f32>(),
            g: r.gen::<f32>(),
            b: r.gen::<f32>(),
            a: 1.0,
        }
    }
    pub fn interpolate(&self, other: &Self, pct: f32) -> Self {
        Self::new(
            lerp(self.r, other.r, pct),
            lerp(self.g, other.g, pct),
            lerp(self.b, other.b, pct),
            lerp(self.a, other.a, pct),
        )
    }
    pub fn hsl(mut self, hue: f32, s: f32, l: f32) -> Self {
        let c = (1.0 - ((2.0 * l) - 1.0).abs()) * s;
        let i1 = hue / 60.0;
        let i2 = i1 % 2.0;
        let i3 = (i2 - 1.0).abs();
        let x = c * (1.0 - i3);
        let (r1, g1, b1) = match hue {
            v if (0.0..60.0).contains(&v) => (c, x, 0.0),
            v if (60.0..120.0).contains(&v) => (x, c, 0.0),
            v if (120.0..180.0).contains(&v) => (0.0, c, x),
            v if (180.0..240.0).contains(&v) => (0.0, x, c),
            v if (240.0..300.0).contains(&v) => (x, 0.0, c),
            v if (300.0..360.0).contains(&v) => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };
        let m = l - c / 2.0;
        let (r, g, b) = (r1 + m, g1 + m, b1 + m);
        self.r = r;
        self.g = g;
        self.b = b;
        self
    }
}
impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::rgb(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}
impl From<Color> for wgpu::Color {
    fn from(c: Color) -> Self {
        Self {
            r: c.r.into(),
            g: c.g.into(),
            b: c.b.into(),
            a: c.a.into(),
        }
    }
}
pub struct AnimatedVec2 {
    start: Vec2,
    end: Vec2,
    time: f32,
}
impl AnimatedVec2 {
    pub fn new(start: Vec2, end: Vec2, time: f32) -> Self {
        Self { start, end, time }
    }
    pub fn animate(&self, pct: f32) -> Vec2 {
        let time = pct / self.time;
        if time < 0.0 {
            return self.start;
        }
        if time > 1.0 {
            return self.end;
        }
        Vec2::new(
            lerp(self.start.x, self.end.x, time),
            lerp(self.start.y, self.end.y, time),
        )
    }
}

pub struct Camera {
    offset: Vec2,
    lower_bound: Vec2,
    upper_bound: Vec2,
    screen_size: Vec2,
    bounded: bool,
}
impl Camera {
    pub fn new(screen_size: Vec2) -> Self {
        Self {
            offset: Vec2::ZERO,
            lower_bound: Vec2::ZERO,
            upper_bound: Vec2::ZERO,
            screen_size,
            bounded: false,
        }
    }
    pub fn set_bounds(&mut self, lower_bound: Vec2, upper_bound: Vec2) {
        self.bounded = true;
        self.lower_bound = lower_bound;
        self.upper_bound = upper_bound;
    }
    pub fn target(&mut self, pos: Vec2) {
        if self.bounded {
            self.offset.x = if pos.x < (self.screen_size.x / 2.0) + self.lower_bound.x {
                self.lower_bound.x
            } else if pos.x > self.upper_bound.x - self.screen_size.x / 2.0 {
                self.upper_bound.x - self.screen_size.x
            } else {
                pos.x - (self.screen_size.x / 2.0)
            };
            self.offset.y = if pos.y < (self.screen_size.y / 2.0) + self.lower_bound.y {
                self.lower_bound.y
            } else if pos.y > self.upper_bound.y - self.screen_size.y / 2.0 {
                self.upper_bound.y - self.screen_size.y
            } else {
                pos.y - (self.screen_size.y / 2.0)
            };
        } else {
            self.offset.x = pos.x - (self.screen_size.x / 2.0);
            self.offset.y = pos.y - (self.screen_size.y / 2.0);
        }
    }
    pub fn get_offset(&self) -> Vec2 {
        self.offset
    }
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub origin: Vec2,
    pub dir: Vec2,
}
pub fn ray(origin: Vec2, dir: Vec2) -> Ray {
    Ray::new(origin, dir)
}
pub fn raycast(origin: Vec2, dir: Vec2, targets: &[LineSegment]) -> Option<RayHit> {
    Ray::new(origin, dir).cast(targets)
}
impl Ray {
    pub fn new_dir(origin: Vec2, dir: f32) -> Self {
        Self {
            origin,
            dir: origin + Vec2::new(dir.cos(), dir.sin()),
        }
    }
    pub fn new(origin: Vec2, dir: Vec2) -> Self {
        Self {
            origin,
            dir: origin + dir,
        }
    }
    /// Raycast against an AABB.
    pub fn intersect_rect(&self, rect: &Rectangle) -> Option<RayHit> {
        let mut near = (rect.top_left - self.origin) / self.dir;
        let mut far = (rect.bottom_right - self.origin) / self.dir;
        if near.x > far.x {
            std::mem::swap(&mut near.x, &mut far.x)
        }
        if near.y > far.y {
            std::mem::swap(&mut near.y, &mut far.y)
        }
        if near.x > far.y || near.y > far.x {
            return None;
        }
        let hit_near = near.x.max(near.y);
        let hit_far = far.x.min(far.y);
        if hit_far < 0.0 {
            return None;
        }
        let hit = self.origin + self.dir * hit_near;
        let normal = match near.x > near.y {
            true => match self.dir.x < 0.0 {
                true => Vec2::new(1, 0),
                false => Vec2::new(-1, 0),
            },
            false => match self.dir.y < 0.0 {
                true => Vec2::new(0, 1),
                false => Vec2::new(0, -1),
            },
        };
        Some(RayHit {
            hit,
            normal,
            time: hit_near,
        })
    }
    pub fn cast_rect(&self, other: &[Rectangle]) -> Option<RayHit> {
        let mut closest = f32::MAX;
        let mut hit: Option<RayHit> = None;
        for rect in other.iter() {
            if let Some(h) = self.intersect_rect(rect) {
                let distance = (h.hit - self.origin).magnitude();
                if distance < closest {
                    hit = Some(h);
                    closest = distance;
                }
            }
        }
        hit
    }
    pub fn cast(&self, other: &[LineSegment]) -> Option<RayHit> {
        let mut closest = f32::MAX;
        let mut hit: Option<RayHit> = None;
        for line in other.iter() {
            if let Some(h) = self.intersection(line) {
                let distance = (h.hit - self.origin).magnitude();
                if distance < closest {
                    hit = Some(h);
                    closest = distance;
                }
            }
        }
        hit
    }
    /// Ray intersection against a LineSegment.
    pub fn intersection(&self, other: &LineSegment) -> Option<RayHit> {
        if let Some((t, u)) = intersect_t_u(&self.origin, &self.dir, &other.begin, &other.end) {
            if u > 0.0 && u < 1.0 && t > 0.0 {
                let x = other.begin.x + u * (other.end.x - other.begin.x);
                let y = other.begin.y + u * (other.end.y - other.begin.y);
                Some(RayHit {
                    hit: Vec2::new(x, y),
                    normal: other.normal(),
                    time: 0.0,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
pub struct RayHit {
    pub hit: Vec2,
    pub normal: Vec2,
    pub time: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rectangle {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
    pub test: bool,
}
impl Rectangle {
    pub fn new2(pos: Vec2, size: Vec2) -> Self {
        Self {
            top_left: pos,
            bottom_right: pos + size,
            test: false,
        }
    }
    pub fn new(
        x: impl AsPrimitive<f32>,
        y: impl AsPrimitive<f32>,
        w: impl AsPrimitive<f32>,
        h: impl AsPrimitive<f32>,
    ) -> Self {
        Self {
            top_left: Vec2::new(x, y),
            bottom_right: Vec2::new(x.as_() + w.as_(), y.as_() + h.as_()),
            test: false,
        }
    }
    pub fn size(&self) -> Vec2 {
        self.bottom_right - self.top_left
    }
    pub fn contains_point(&self, other: &Vec2) -> bool {
        if other.x < self.top_left.x {
            return false;
        }
        if other.x > self.bottom_right.x {
            return false;
        }
        if other.y < self.top_left.y {
            return false;
        }
        if other.y > self.bottom_right.y {
            return false;
        }
        true
    }
    pub fn intersects_rect(&self, other: &Self) -> bool {
        if other.bottom_right.x < self.top_left.x {
            return false;
        }
        if other.top_left.x > self.bottom_right.x {
            return false;
        }
        if other.bottom_right.y < self.top_left.y {
            return false;
        }
        if other.top_left.y > self.bottom_right.y {
            return false;
        }
        true
    }
    pub fn overlapping_box(&self, other: &Self) -> Option<(Vec2, Vec2)> {
        if !self.intersects_rect(other) {
            None
        } else {
            let x1 = self.top_left.x.max(other.top_left.x);
            let x2 = self.bottom_right.x.min(other.bottom_right.x);
            let y1 = self.top_left.y.max(other.top_left.y);
            let y2 = self.bottom_right.y.min(other.bottom_right.y);
            Some((Vec2::new(x1, y1), Vec2::new(x2, y2)))
        }
    }
    pub fn intersect_vector(&self, other: &Self) -> Option<Vec2> {
        if !self.intersects_rect(other) {
            None
        } else {
            let x1 = self.top_left.x.max(other.top_left.x);
            let x2 = self.bottom_right.x.min(other.bottom_right.x);
            let y1 = self.top_left.y.max(other.top_left.y);
            let y2 = self.bottom_right.y.min(other.bottom_right.y);

            Some(Vec2::new(x2 - x1, y2 - y1))
        }
    }
}
pub fn cubic_to_point(time: f32, begin: Vec2, control1: Vec2, control2: Vec2, end: Vec2) -> Vec2 {
    let part1 = begin * (1.0 - time).powf(3.0);
    let part2 = control2 * control1 * time * 3.0 * (1.0 - time).powf(2.0);
    let part3 = control2 * 3.0 * (1.0 - time) * time.powf(2.0);
    let part4 = end * time.powf(3.0);
    part1 + part2 + part3 + part4
}
pub fn quadratic_to_point(time: f32, begin: Vec2, control: Vec2, end: Vec2) -> Vec2 {
    control + (begin - control) * (1.0 - time).powf(2.0) + (end - control) * time.powf(2.0)
}

/// Meant to represent a line that stretches to infinity in both directions.
pub struct Line {
    pub first: Vec2,
    pub second: Vec2,
}
impl Line {
    pub fn new(first: Vec2, second: Vec2) -> Self {
        Self { first, second }
    }
    pub fn intersections_line_segment(&self, other: &Vec<LineSegment>) -> Option<Vec<Vec2>> {
        let mut intersections = vec![];
        for ls in other {
            if let Some(intersection) = self.intersect_line_segment(ls) {
                intersections.push(intersection);
            }
        }
        if intersections.is_empty() {
            None
        } else {
            Some(intersections)
        }
    }
    pub fn intersect_line_segment(&self, other: &LineSegment) -> Option<Vec2> {
        if let Some((_, u)) = intersect_t_u(&self.first, &self.second, &other.begin, &other.end) {
            if u > 0.0 && u < 1.0 {
                let x = other.begin.x + u * (other.end.x - other.begin.x);
                let y = other.begin.y + u * (other.end.y - other.begin.y);
                Some(Vec2::new(x, y))
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn intersections(&self, other: &Vec<Self>) -> Option<Vec<Vec2>> {
        let mut intersections = vec![];
        for axis in other {
            if let Some(intersection) = self.intersect(axis) {
                intersections.push(intersection);
            }
        }
        if intersections.is_empty() {
            return None;
        }
        Some(intersections)
    }
    pub fn intersect(&self, other: &Self) -> Option<Vec2> {
        if let Some((_, u)) = intersect_t_u(&self.first, &self.second, &other.first, &other.second)
        {
            let x = other.first.x + u * (other.second.x - other.first.x);
            let y = other.first.y + u * (other.second.y - other.first.y);
            Some(Vec2::new(x, y))
        } else {
            None
        }
    }
}
/// Line intersection  formula. For re-use. None when parallel, otherwise Some(Vec2)
pub fn intersect_t_u(p1: &Vec2, p2: &Vec2, p3: &Vec2, p4: &Vec2) -> Option<(f32, f32)> {
    let denom = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
    let t_num = (p1.x - p3.x) * (p3.y - p4.y) - (p1.y - p3.y) * (p3.x - p4.x);
    let u_num = (p1.x - p3.x) * (p1.y - p2.y) - (p1.y - p3.y) * (p1.x - p2.x);
    if denom == 0.0 {
        return None;
    }
    Some((t_num / denom, u_num / denom))
}
/// LineSegment, has a start and end Vec2.
#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    pub begin: Vec2,
    pub end: Vec2,
}
impl LineSegment {
    pub fn new(begin: Vec2, end: Vec2) -> Self {
        Self { begin, end }
    }
    pub fn normal(&self) -> Vec2 {
        let d = self.end - self.begin;
        let a = d.angle2() - PI / 2.0;
        angle_vec2(a)
    }
    pub fn length(&self) -> f32 {
        let d = self.end - self.begin;
        d.magnitude()
    }
    pub fn axis(&self) -> Vec2 {
        let d = self.end - self.begin;
        let a = d.angle2();
        Vec2::new(a.cos(), a.sin())
    }
    pub fn first_intersection(&self, others: &Vec<Self>) -> Option<Vec2> {
        let mut hit: Option<Vec2> = None;
        let mut record = f32::MAX;
        for wall in others {
            if let Some(p) = self.intersection(wall) {
                let d = (p - self.begin).magnitude();
                if d < record {
                    record = d;
                    hit = Some(p);
                }
            }
        }
        hit
    }
    pub fn intersection(&self, other: &Self) -> Option<Vec2> {
        if let Some((t, u)) = intersect_t_u(&self.begin, &self.end, &other.begin, &other.end) {
            if u > 0.0 && u < 1.0 && t > 0.0 && t < 1.0 {
                let x = other.begin.x + u * (other.end.x - other.begin.x);
                let y = other.begin.y + u * (other.end.y - other.begin.y);
                Some(Vec2::new(x, y))
            } else {
                None
            }
        } else {
            None
        }
    }
}
#[derive(Clone, Debug)]
pub struct PathData {
    pub segments: Vec<PathSegment>,
}
#[derive(Clone, Debug)]
pub struct PathSegment {
    pub contours: Vec<Contour>,
}
#[derive(Debug, Clone, Copy)]
pub enum Contour {
    MoveTo(Vec2), //basically new segment
    LineTo(Vec2),
    QuadTo(Vec2, Vec2),        // control, end
    CubicTo(Vec2, Vec2, Vec2), // control1, control2, end
    ClosePath(bool),           // basically end segment
}

pub struct PathBuilder {
    contours: Vec<Contour>,
    segments: Vec<PathSegment>,
    offset: Vec2,
    pub pathing: bool,
    pub built: bool,
}
impl Default for PathBuilder {
    fn default() -> Self {
        Self {
            contours: vec![],
            segments: vec![],
            offset: Vec2::ZERO,
            pathing: false,
            built: false,
        }
    }
}
impl PathBuilder {
    pub fn move_to(&mut self, pos: Vec2) -> &mut Self {
        self.pathing = true;
        self.contours.push(Contour::MoveTo(pos + self.offset));
        self
    }
    pub fn line_to(&mut self, end: Vec2) -> &mut Self {
        self.contours.push(Contour::LineTo(end + self.offset));
        self
    }
    pub fn quadratic_to(&mut self, control: Vec2, end: Vec2) -> &mut Self {
        self.contours
            .push(Contour::QuadTo(control + self.offset, end + self.offset));
        self
    }
    pub fn cubic_to(&mut self, control1: Vec2, control2: Vec2, end: Vec2) -> &mut Self {
        self.contours.push(Contour::CubicTo(
            control1 + self.offset,
            control2 + self.offset,
            end + self.offset,
        ));
        self
    }
    pub fn close_path(&mut self, close: bool) -> &mut Self {
        self.contours.push(Contour::ClosePath(close));
        let mut path_segment = PathSegment { contours: vec![] };
        path_segment.contours.append(&mut self.contours);
        self.segments.push(path_segment);
        self.contours.clear();
        self
    }
    pub fn set_offset(&mut self, offset: Vec2) {
        self.offset = offset;
    }
    pub fn translate_offset(&mut self, offset: Vec2) {
        self.offset += offset;
    }
    pub fn clear_offset(&mut self) {
        self.offset = Vec2::ZERO;
    }
    pub fn build(&mut self) -> PathData {
        self.built = true;
        let mut path = PathData { segments: vec![] };
        path.segments.append(&mut self.segments);
        path
    }
}

pub fn lerp(start: f32, end: f32, pct: f32) -> f32 {
    start + (end - start) * pct
}
pub fn ease_in(pct: f32) -> f32 {
    let pct = pct.clamp(0.0, 1.0);
    pct * pct
}
pub fn flip(pct: f32) -> f32 {
    let pct = pct.clamp(0.0, 1.0);
    1.0 - pct
}

pub fn is_y_monotone(polygon: &Polygon, debug: Option<&mut MeshBuilder>) -> bool {
    let mut hits = 0;
    let mut hit_p = vec![];
    let segments = &polygon
        .edges
        .iter()
        .map(|(s, e)| LineSegment::new(polygon.points[*s], polygon.points[*e]))
        .collect::<Vec<_>>();
    for p in polygon.points.iter() {
        let line = Line::new(*p, *p + Vec2::new(0, 100));
        if let Some(intersections) = line.intersections_line_segment(segments) {
            hits = intersections.len().max(hits);
            hit_p.extend(intersections);
        }
    }
    if let Some(mb) = debug {
        mb.push();
        mb.set_style(FillStyle::Solid(Color::RED));
        for h in hit_p {
            mb.set_cursor(h - Vec2::new(2, 2));
            mb.rect(Vec2::new(4, 4));
        }
        mb.pop();
    }
    hits <= 2
}
pub fn is_convex_polygon2(polygon: &Polygon) -> bool {
    let mut result = true;

    for test in 0..polygon.points.len() {
        let neighbors = polygon.get_vertex_neighbors(test);

        let testp = polygon.points[test];

        let first = testp - neighbors[0];
        let second = testp - neighbors[1];

        let mut total = (second.angle() - first.angle()).to_degrees();
        if total < 0.0 {
            total += 360.0;
        }
        if total > 180.0 {
            result = false;
            println!(
                "{:?} {:?} {:?}",
                polygon.points[test], neighbors[0], neighbors[1]
            );
        }
        println!("p: {} {:?}", test, total);
    }
    result
}
pub fn is_convex_polygon(polygon: &Polygon, debug: &mut MeshBuilder) -> bool {
    debug.push();
    let mut result = true;

    'label: for testp in 0..polygon.points.len() {
        for endp in 0..polygon.points.len() {
            if testp != endp {
                debug.set_style(FillStyle::Solid(Color::BLUE));
                let test = LineSegment::new(polygon.points[testp], polygon.points[endp]);
                debug.line(test.begin, test.end);
                let mut segments = vec![];
                polygon.edges.iter().for_each(|(b, e)| {
                    if *b != testp && *e != testp {
                        debug.set_style(FillStyle::Solid(Color::BROWN));
                        let segment = LineSegment::new(polygon.points[*b], polygon.points[*e]);
                        debug.line(segment.begin, segment.end);
                        segments.push(segment);
                    }
                });
                if test.first_intersection(&segments).is_some() {
                    result = false;
                    break 'label;
                }
            }
        }
    }
    debug.pop();
    result
}
pub fn is_x_monotone(polygon: &Polygon, debug: Option<&mut MeshBuilder>) -> bool {
    let mut hits = 0;
    let mut hit_p = vec![];

    let segments = &polygon
        .edges
        .iter()
        .map(|(s, e)| LineSegment::new(polygon.points[*s], polygon.points[*e]))
        .collect::<Vec<_>>();
    for p in polygon.points.iter() {
        let line = Line::new(*p, *p + Vec2::new(100, 0));
        if let Some(intersections) = line.intersections_line_segment(segments) {
            hits = intersections.len().max(hits);
            hit_p.extend(intersections);
        }
    }
    if let Some(mb) = debug {
        mb.push();
        mb.set_style(FillStyle::Solid(Color::MAGENTA));
        for h in hit_p {
            mb.set_cursor(h - Vec2::new(2, 2));
            mb.rect(Vec2::new(4, 4));
        }
        mb.pop();
    }
    hits <= 2
}
