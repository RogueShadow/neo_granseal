use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign, DivAssign};
use num_traits::{AsPrimitive};
use rand::{Rng, SeedableRng};

#[derive(Copy,Clone,Debug,PartialEq)]
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
    pub const LIGHT_SALMON: Self = Self::rgb(1.0, 160.0/255.0,122.0/255.0);
    pub const SALMON: Self = Self::rgb(250.0/255.0,0.5,114.0/255.0);
    pub const DARK_SALMON: Self = Self::rgb(233.0/255.0,150.0/255.0,122.0/255.0);
    pub const LIGHT_CORAL: Self = Self::rgb(240.0/255.0,0.5,0.5);
    pub const INDIAN_RED: Self = Self::rgb(205.0/255.0,92.0/255.0,92.0/255.0);
    pub const CRIMSON: Self = Self::rgb(220.0/255.0,20.0/255.0,60.0/255.0);
    pub const FIRE_BRICK: Self = Self::rgb(178.0/255.0,34.0/255.0,34.0/255.0);
    pub const RED: Self = Self::rgb(1.0,0.0,0.0);
    pub const DARK_RED: Self = Self::rgb(139.0/255.0,0.0,0.0);
    // orange colors
    pub const CORAL: Self = Self::rgb(1.0,127.0/255.0,80.0/255.0);
    pub const TOMATO: Self = Self::rgb(1.0,99.0/255.0,71.0/255.0);
    pub const ORANGE_RED: Self = Self::rgb(1.0,69.0/255.0,0.0);
    pub const GOLD: Self = Self::rgb(1.0,215.0/255.0,0.0);
    pub const ORANGE: Self = Self::rgb(1.0,165.0/255.0,0.0);
    pub const DARK_ORANGE: Self = Self::rgb(1.0,140.0/255.0,0.0);
    // yellow colors
    pub const LIGHT_YELLOW: Self = Self::rgb(1.0,1.0,224.0/255.0);
    pub const LEMON_CHIFFON: Self = Self::rgb(1.0,250.0/255.0,205.0/255.0);
    pub const LIGHT_GOLDENROD_YELLOW: Self = Self::rgb(250.0/255.0,250.0/255.0,210.0/255.0);
    pub const PAPAYA_WHIP: Self = Self::rgb(1.0,239.0/255.0,213.0/255.0);
    pub const MOCCASIN: Self = Self::rgb(1.0,228.0/255.0,181.0/255.0);
    pub const PEACH_PUFF: Self = Self::rgb(1.0,218.0/255.0,185.0/255.0);
    pub const PALE_GOLDENROD: Self = Self::rgb(238.0/255.0,232.0/255.0,170.0/255.0);
    pub const KHAKI: Self = Self::rgb(240.0/255.0,230.0/255.0,140.0/255.0);
    pub const DARK_KHAKI: Self = Self::rgb(189.0/255.0,183.0/255.0,107.0/255.0);
    pub const YELLOW: Self = Self::rgb(1.0,1.0,0.0);
    // green colors
    pub const LAWN_GREEN: Self = Self::rgb(124.0/255.0,252.0/255.0,0.0);
    pub const CHARTREUSE: Self = Self::rgb(127.0/255.0,1.0,0.0);
    pub const LIME_GREEN: Self = Self::rgb(50.0/255.0,205.0/255.0,50.0/255.0);
    pub const LIME: Self = Self::rgb(0.0,1.0,0.0);
    pub const FOREST_GREEN: Self = Self::rgb(34.0/255.0,139.0/255.0,34.0/255.0);
    pub const GREEN: Self = Self::rgb(0.0,128.0/255.0,0.0);
    pub const DARK_GREEN: Self = Self::rgb(0.0,100.0/255.0,0.0);
    pub const GREEN_YELLOW: Self = Self::rgb(173.0/255.0,1.0,47.0/255.0);
    pub const YELLOW_GREEN: Self = Self::rgb(154.0/255.0,205.0/255.0,50.0/255.0);
    pub const SPRING_GREEN: Self = Self::rgb(0.0,1.0,127.0/255.0);
    pub const MEDIUM_SPRING_GREEN: Self = Self::rgb(0.0,250.0/255.0,154.0/255.0);
    pub const LIGHT_GREEN: Self = Self::rgb(144.0/255.0,238.0/255.0,144.0/255.0);
    pub const PALE_GREEN: Self = Self::rgb(152.0/255.0,251.0/255.0,152.0/255.0);
    pub const DARK_SEA_GREEN: Self = Self::rgb(143.0/255.0,188.0/255.0,143.0/255.0);
    pub const MEDIUM_SEA_GREEN: Self = Self::rgb(143.0/255.0,188.0/255.0,143.0/255.0);
    pub const SEA_GREEN: Self = Self::rgb(46.0/255.0,139.0/255.0,87.0/255.0);
    pub const OLIVE: Self = Self::rgb(128.0/255.0,128.0/255.0,0.0);
    pub const DARK_OLIVE_GREEN: Self = Self::rgb(85.0/255.0,107.0/255.0,47.0/255.0);
    pub const OLIVE_DRAB: Self = Self::rgb(107.0/255.0,142.0/255.0,35.0/255.0);
    // cyan colors
    pub const LIGHT_CYAN: Self = Self::rgb(224.0/255.0,1.0,1.0);
    pub const CYAN: Self = Self::rgb(0.0,1.0,1.0);
    pub const AQUA: Self = Self::rgb(0.0,1.0,1.0);
    pub const AQUAMARINE: Self = Self::rgb(127.0/255.0,1.0,212.0/255.0);
    pub const MEDIUM_AQUAMARINE: Self = Self::rgb(102.0/255.0,205.0/255.0,170.0/255.0);
    pub const PALE_TURQUOISE: Self = Self::rgb(175.0/255.0,238.0/255.0,238.0/255.0);
    pub const TURQUOISE: Self = Self::rgb(64.0/255.0,224.0/255.0,208.0/255.0);
    pub const MEDIUM_TURQUOISE: Self = Self::rgb(72.0/255.0,209.0/255.0,204.0/255.0);
    pub const DARK_TURQUOISE: Self = Self::rgb(0.0,206.0/255.0,209.0/255.0);
    pub const LIGHT_SEA_GREEN: Self = Self::rgb(32.0/255.0,178.0/255.0,170.0/255.0);
    pub const CADET_BLUE: Self = Self::rgb(95.0/255.0,158.0/255.0,160.0/255.0);
    pub const DARK_CYAN: Self = Self::rgb(0.0,139.0/255.0,139.0/255.0);
    pub const TEAL: Self = Self::rgb(0.0,128.0/255.0,128.0/255.0);
    // blue colors
    pub const POWDER_BLUE: Self = Self::rgb(176.0/255.0,224.0/255.0,230.0/255.0);
    pub const LIGHT_BLUE: Self = Self::rgb(173.0/255.0,216.0/255.0,230.0/255.0);
    pub const LIGHT_SKY_BLUE: Self = Self::rgb(135.0/255.0,206.0/255.0,250.0/255.0);
    pub const SKY_BLUE: Self = Self::rgb(135.0/255.0,206.0/255.0,235.0/255.0);
    pub const DEEP_SKY_BLUE: Self = Self::rgb(0.0,191.0/255.0,1.0);
    pub const LIGHT_STEEL_BLUE: Self = Self::rgb(176.0/255.0,196.0/255.0,222.0/255.0);
    pub const DODGER_BLUE: Self = Self::rgb(30.0/255.0,144.0/255.0,1.0);
    pub const CORNFLOWER_BLUE: Self = Self::rgb(100.0/255.0,149.0/255.0,237.0/255.0);
    pub const STEEL_BLUE: Self = Self::rgb(70.0/255.0,130.0/255.0,180.0/255.0);
    pub const ROYAL_BLUE: Self = Self::rgb(65.0/255.0,105.0/255.0,225.0/255.0);
    pub const BLUE: Self = Self::rgb(0.0,0.0,1.0);
    pub const MEDIUM_BLUE: Self = Self::rgb(0.0,0.0,205.0/255.0);
    pub const DARK_BLUE: Self = Self::rgb(0.0,0.0,139.0/255.0);
    pub const NAVY: Self = Self::rgb(0.0,0.0,128.0/255.0);
    pub const MIDNIGHT_BLUE: Self = Self::rgb(25.0/255.0,25.0/255.0,112.0/255.0);
    pub const MEDIUM_SLATE_BLUE: Self = Self::rgb(123.0/255.0,104.0/255.0,238.0/255.0);
    pub const SLATE_BLUE: Self = Self::rgb(106.0/255.0,90.0/255.0,205.0/255.0);
    pub const DARK_SLATE_BLUE: Self = Self::rgb(72.0/255.0,61.0/255.0,139.0/255.0);
    // purple colors
    pub const LAVENDER: Self = Self::rgb(230.0/255.0,230.0/255.0,250.0/255.0);
    pub const THISTLE: Self = Self::rgb(216.0/255.0,191.0/255.0,216.0/255.0);
    pub const PLUM: Self = Self::rgb(221.0/255.0,160.0/255.0,221.0/255.0);
    pub const VIOLET: Self = Self::rgb(238.0/255.0,130.0/255.0,238.0/255.0);
    pub const ORCHID: Self = Self::rgb(218.0/255.0,112.0/255.0,214.0/255.0);
    pub const FUCHSIA: Self = Self::rgb(1.0, 0.0, 1.0);
    pub const MAGENTA: Self = Self::rgb(1.0,0.0,1.0);
    pub const MEDIUM_ORCHID: Self = Self::rgb(186.0/255.0,85.0/255.0,211.0/255.0);
    pub const MEDIUM_PURPLE: Self = Self::rgb(147.0/255.0,112.0/255.0,219.0/255.0);
    pub const BLUE_VIOLET: Self = Self::rgb(138.0/255.0,43.0/255.0,226.0/255.0);
    pub const DARK_VIOLET: Self = Self::rgb(148.0/255.0,0.0,211.0/255.0);
    pub const DARK_ORCHID: Self = Self::rgb(153.0/255.0,50.0/255.0,204.0/255.0);
    pub const DARK_MAGENTA: Self = Self::rgb(139.0/255.0,0.0,139.0/255.0);
    pub const PURPLE: Self = Self::rgb(128.0/255.0,0.0,128.0/255.0);
    pub const INDIGO: Self = Self::rgb(75.0/255.0,0.0,130.0/255.0);
    // pink colors
    pub const PINK: Self = Self::rgb(1.0,192.0/255.0,203.0/255.0);
    pub const LIGHT_PINK: Self = Self::rgb(1.0,182.0/255.0,193.0/255.0);
    pub const HOT_PINK: Self = Self::rgb(1.0,105.0/255.0,180.0/255.0);
    pub const DEEP_PINK: Self = Self::rgb(1.0,20.0/255.0,147.0/255.0);
    pub const PALE_VIOLET_RED: Self = Self::rgb(219.0/255.0,112.0/255.0,147.0/255.0);
    pub const MEDIUM_VIOLET_RED: Self = Self::rgb(199.0/255.0,21.0/255.0,133.0/255.0);
    // white colors
    pub const WHITE: Self = Self::rgb(1.0,1.0,1.0);
    pub const SNOW: Self = Self::rgb(1.0,250.0/255.0,250.0/255.0);
    pub const HONEYDEW: Self = Self::rgb(240.0/255.0,1.0,240.0/255.0);
    pub const MINT_CREAM: Self = Self::rgb(245.0/255.0,1.0,250.0/255.0);
    pub const AZURE: Self = Self::rgb(240.0/255.0,1.0,1.0);
    pub const ALICE_BLUE: Self = Self::rgb(240.0/255.0,248.0/255.0,1.0);
    pub const GHOST_WHITE: Self = Self::rgb(248.0/255.0,248.0/255.0, 1.0);
    pub const WHITE_SMOKE: Self = Self::rgb(245.0/255.0,245.0/255.0,245.0/255.0);
    pub const SEASHELL: Self = Self::rgb(1.0,245.0/255.0,238.0/255.0);
    pub const BEIGE: Self = Self::rgb(245.0/255.0,245.0/255.0,220.0/255.0);
    pub const OLD_LACE: Self = Self::rgb(253.0/255.0,245.0/255.0,230.0/255.0);
    pub const FLORAL_WHITE: Self = Self::rgb(1.0,250.0/255.0,240.0/255.0);
    pub const IVORY: Self = Self::rgb(1.0,1.0,240.0/255.0);
    pub const ANTIQUE_WHITE: Self = Self::rgb(250.0/255.0,235.0/255.0,215.0/255.0);
    pub const LINEN: Self = Self::rgb(250.0/255.0,240.0/255.0,230.0/255.0);
    pub const LAVENDER_BLUSH: Self = Self::rgb(1.0,240.0/255.0,245.0/255.0);
    pub const MISTY_ROSE: Self = Self::rgb(1.0,228.0/255.0,225.0/255.0);
    // gray colors
    pub const GAINSBORO: Self = Self::rgb(220.0/255.0,220.0/255.0,220.0/255.0);
    pub const LIGHT_GRAY: Self = Self::rgb(211.0/255.0,211.0/255.0,211.0/255.0);
    pub const SILVER: Self = Self::rgb(192.0/255.0,192.0/255.0,192.0/255.0);
    pub const DARK_GRAY: Self = Self::rgb(169.0/255.0,169.0/255.0,169.0/255.0);
    pub const GRAY: Self = Self::rgb(128.0/255.0,128.0/255.0,128.0/255.0);
    pub const DIM_GRAY: Self = Self::rgb(105.0/255.0,105.0/255.0,105.0/255.0);
    pub const LIGHT_SLATE_GRAY: Self = Self::rgb(119.0/255.0,136.0/255.0,153.0/255.0);
    pub const SLATE_GRAY: Self = Self::rgb(112.0/255.0,128.0/255.0,144.0/255.0);
    pub const BLACK: Self = Self::rgb(0.0,0.0,0.0);
    // brown colors
    pub const CORN_SILK: Self = Self::rgb(1.0,248.0/255.0,220.0/255.0);
    pub const BLANCHED_ALMOND: Self = Self::rgb(1.0,235.0/255.0,205.0/255.0);
    pub const BISQUE: Self = Self::rgb(1.0,228.0/255.0,196.0/255.0);
    pub const NAVAJO_WHITE: Self = Self::rgb(1.0,222.0/255.0,173.0/255.0);
    pub const WHEAT: Self = Self::rgb(245.0/255.0,222.0/255.0,179.0/255.0);
    pub const BURLY_WOOD: Self = Self::rgb(222.0/255.0,184.0/255.0,135.0/255.0);
    pub const TAN: Self = Self::rgb(210.0/255.0,180.0/255.0,140.0/255.0);
    pub const ROSY_BROWN: Self = Self::rgb(188.0/255.0,143.0/255.0,143.0/255.0);
    pub const SANDY_BROWN: Self = Self::rgb(188.0/255.0,164.0/255.0,96.0/255.0);
    pub const GOLDENROD: Self = Self::rgb(218.0/255.0,165.0/255.0,32.0/255.0);
    pub const PERU: Self = Self::rgb(205.0/255.0,133.0/255.0,63.0/255.0);
    pub const CHOCOLATE: Self = Self::rgb(210.0/255.0,105.0/255.0,30.0/255.0);
    pub const SADDLE_BROWN: Self = Self::rgb(139.0/255.0,69.0/255.0,19.0/255.0);
    pub const SIENNA: Self = Self::rgb(160.0/255.0,82.0/255.0,45.0/255.0);
    pub const BROWN: Self = Self::rgb(165.0/255.0,42.0/255.0,42.0/255.0);
    pub const MAROON: Self = Self::rgb(128.0/255.0,0.0,0.0);
}

impl Color {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {r,g,b,a}
    }
    pub fn rgb_u8(r: u8,g: u8, b: u8) -> Self {
        Self::new(
            r as f32/u8::MAX as f32,
            g as f32/u8::MAX as f32,
            b as f32/u8::MAX as f32,
            1.0)
    }
    pub fn invert(mut self) -> Self {
        self.r = 1.0 - self.r;
        self.g = 1.0 - self.g;
        self.b = 1.0 - self.b;
        self
    }
    pub fn adjust(mut self, v: f32) -> Self {
        self.r = (self.r + v).clamp(0.0,1.0);
        self.g = (self.g + v).clamp(0.0,1.0);
        self.b = (self.b + v).clamp(0.0,1.0);
        self
    }
    pub fn random() -> Self {
        let mut r = rand_xorshift::XorShiftRng::from_rng(rand::thread_rng()).unwrap();
        Self {
            r: r.gen::<f32>(),
            g: r.gen::<f32>(),
            b: r.gen::<f32>(),
            a: 1.0,
        }
    }
    pub fn hsl(mut self, hue: f32, s: f32, l: f32) -> Self {
        let c = (1.0 - ((2.0 * l) - 1.0).abs()) * s;
        let i1 = hue / 60.0;
        let i2 = i1 % 2.0;
        let i3 = (i2 - 1.0).abs();
        let x = c * (1.0 - i3);
        let (r1,g1,b1) = match hue {
            v if (0.0..60.0).contains(&v) => (c, x, 0.0),
            v if (60.0..120.0).contains(&v) => (x, c, 0.0),
            v if (120.0..180.0).contains(&v) => (0.0, c, x),
            v if (180.0..240.0).contains(&v)  => (0.0, x, c),
            v if (240.0..300.0).contains(&v) => (x, 0.0, c),
            v if (300.0..360.0).contains(&v)  => (c, 0.0, x),
            _ => {(0.0,0.0,0.0)}
        };
        let m = l - c / 2.0;
        let (r,g,b) = (r1+m,g1+m,b1+m);
        self.r = r;
        self.g = g;
        self.b = b;
        self
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
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
impl Point {
    pub const ZERO: Point = Point { x: 0.0, y: 0.0};
    pub fn new(x: impl AsPrimitive<f32>, y: impl AsPrimitive<f32>) -> Self {
        Self {x: x.as_(),y: y.as_()}
    }
    pub fn len(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    pub fn norm(&self) -> Self {
        let len = self.len();
        Point::new(self.x / len, self.y / len)
    }
    pub fn angle(&self) -> f32 {
        (self.y / self.x).atan()
    }
    pub fn rotate(&self, a: f32) -> Self {
        let na = self.angle() + a;
        Point::new(self.x * na.cos(), self.y * na.sin())
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn proj(&self, rhs: &Self) -> f32 {
        self.dot(rhs) / rhs.len()
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(self.x * rhs.x,self.y * rhs.y)
    }
}
impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Point) -> Self::Output {
        Point::new(self.x / rhs.x, self.y / rhs.y)
    }
}
impl <T>Add<T> for Point where T: AsPrimitive<f32> {
    type Output = Point;

    fn add(self, rhs: T) -> Self::Output {
        Point::new(self.x + rhs.as_(),self.y + rhs.as_())
    }
}
impl <T>Sub<T> for Point where T: AsPrimitive<f32> {
    type Output =  Point;

    fn sub(self, rhs: T) -> Self::Output {
        Point::new(self.x - rhs.as_(),self.y - rhs.as_())
    }
}
impl <T>Mul<T> for Point where T: AsPrimitive<f32> {
    type Output = Point;

    fn mul(self, rhs: T) -> Self::Output {
        Point::new(self.x * rhs.as_(),self.y * rhs.as_())
    }
}
impl <T>Div<T> for Point where T: AsPrimitive<f32> {
    type Output = Point;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output::new(self.x / rhs.as_(),self.y / rhs.as_())
    }
}
impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new(-self.x,-self.y)
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl MulAssign for Point {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
impl DivAssign for Point {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl Mul<Point> for f32  {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(self * rhs.x,self * rhs.y)
    }
}

pub struct Camera {
    offset: Point,
    lower_bound: Point,
    upper_bound: Point,
    screen_size: Point,
    bounded: bool,
}
impl Camera {
    pub fn new(screen_size: Point) -> Self {
        Self {
            offset: Point::ZERO,
            lower_bound: Point::ZERO,
            upper_bound: Point::ZERO,
            screen_size,
            bounded: false,
        }
    }
    pub fn set_bounds(&mut self, lower_bound: Point, upper_bound: Point) {
        self.bounded = true;
        self.lower_bound = lower_bound;
        self.upper_bound = upper_bound;
    }
    pub fn target(&mut self, pos: Point) {
        if self.bounded {
            self.offset.x = if pos.x < (self.screen_size.x / 2.0) + self.lower_bound.x { self.lower_bound.x } else {
                if pos.x > self.upper_bound.x - self.screen_size.x / 2.0 {
                    self.upper_bound.x - self.screen_size.x
                } else {
                    pos.x - (self.screen_size.x / 2.0)
                }
            };
            self.offset.y = if pos.y < (self.screen_size.y / 2.0) + self.lower_bound.y { self.lower_bound.y } else {
                if pos.y > self.upper_bound.y - self.screen_size.y / 2.0 {
                    self.upper_bound.y - self.screen_size.y
                } else {
                    pos.y - (self.screen_size.y / 2.0)
                }
            };
        }else{
            self.offset.x = pos.x - (self.screen_size.x / 2.0);
            self.offset.y = pos.y - (self.screen_size.y / 2.0);
        }
    }
    pub fn get_offset(&self) -> Point {
        self.offset
    }
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub origin: Point,
    pub dir: Point,
}
impl Ray {
    pub fn new(origin: Point, dir: Point) -> Self { Self { origin, dir, } }
    pub fn cast_rect(&self, rect: &Rectangle) -> Option<RayHit> {
        let mut near = (rect.top_left - self.origin) / self.dir;
        let mut far = (rect.bottom_right - self.origin) / self.dir;
        if near.x > far.x {std::mem::swap(&mut near.x,&mut far.x)}
        if near.y > far.y {std::mem::swap(&mut near.y,&mut far.y)}
        if near.x > far.y || near.y > far.x {return None}
        let hit_near = near.x.max(near.y);
        let hit_far = far.x.min(far.y);
        if hit_far < 0.0 {return None}
        let hit = self.origin + self.dir * hit_near;
        let normal = match near.x > near.y {
            true => match self.dir.x < 0.0 {
                true => Point::new(1,0),
                false => Point::new(-1,0),
            },
            false => match self.dir.y < 0.0 {
                true => Point::new(0,1),
                false => Point::new(0,-1),
            }
        };
        Some(RayHit { hit, normal, time: hit_near, })
    }
}
pub struct RayHit {
    pub hit: Point,
    pub normal: Point,
    pub time: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
    pub test: bool,
}
impl Rectangle {
    pub fn new2(pos: Point, size: Point) -> Self {
        Rectangle {
            top_left: pos,
            bottom_right: pos + size,
            test: false,
        }
    }
    pub fn new(x: impl AsPrimitive<f32>,y: impl AsPrimitive<f32>,w: impl AsPrimitive<f32>,h: impl AsPrimitive<f32>) -> Self {
        Self {
            top_left: Point::new(x,y),
            bottom_right: Point::new(x.as_()+w.as_(),y.as_()+h.as_()),
            test: false,
        }
    }
    pub fn contains_point(&self, other: &Point) -> bool {
        if other.x < self.top_left.x {return false}
        if other.x > self.bottom_right.x {return false}
        if other.y < self.top_left.y {return false}
        if other.y > self.bottom_right.y {return false}
        true
    }
    pub fn intersects_rect(&self, other: &Self) -> bool {
        if other.bottom_right.x < self.top_left.x {return false}
        if other.top_left.x > self.bottom_right.x {return false}
        if other.bottom_right.y < self.top_left.y {return false}
        if other.top_left.y > self.bottom_right.y {return false}
        true
    }
    pub fn overlapping_box(&self, other: &Self) -> Option<(Point,Point)> {
        if !self.intersects_rect(other) {
            return None;
        }else{
            let x1 = self.top_left.x.max(other.top_left.x);
            let x2 = self.bottom_right.x.min(other.bottom_right.x);
            let y1 = self.top_left.y.max(other.top_left.y);
            let y2 = self.bottom_right.y.min(other.bottom_right.y);
            Some((Point::new(x1,y1),Point::new(x2,y2)))
        }
    }
    pub fn intersect_vector(&self, other: &Self) -> Option<Point> {
        if !self.intersects_rect(other) {
            return None;
        }else{
            let x1 = self.top_left.x.max(other.top_left.x);
            let x2 = self.bottom_right.x.min(other.bottom_right.x);
            let y1 = self.top_left.y.max(other.top_left.y);
            let y2 = self.bottom_right.y.min(other.bottom_right.y);

            Some(Point::new(x2-x1,y2-y1))
        }
    }
}
pub fn cubic_to_point(time: f32, begin: Point, control1: Point, control2: Point, end: Point) -> Point {
    let part1 = (1.0 - time).powf(3.0) * begin;
    let part2 = 3.0 * (1.0 - time).powf(2.0) * time * control1;
    let part3 = 3.0 * (1.0 - time) * time.powf(2.0) * control2;
    let part4 = time.powf(3.0) * end;
    (part1 + part2 + part3 + part4)
}
pub fn quadratic_to_point(time: f32, begin: Point, control: Point, end: Point) -> Point {
    control + (1.0 - time).powf(2.0) * (begin - control) + time.powf(2.0) * (end - control)
}
pub fn text_to_path<'a>(pb: &'a mut PathBuilder,font: &rusttype::Font, text: &str,scale: f32) -> &'a PathBuilder {
    for c in text.chars() {
        let sg = font.glyph(c).scaled(rusttype::Scale::uniform(scale));
        sg.build_outline(pb);
        pb.translate_offset(Point::new(sg.h_metrics().advance_width,0));
    }
    pb
}

pub struct PathData {
    pub segments: Vec<PathSegment>,
}
pub struct PathSegment {
    pub contours: Vec<Contour>,
}
#[derive(Debug,Clone,Copy)]
pub enum Contour {
    MoveTo(Point), //basically new segment
    LineTo(Point),
    QuadTo(Point,Point), // control, end
    CubicTo(Point,Point,Point), // control1, control2, end
    ClosePath,   // basically end segment
}

pub struct PathBuilder {
    contours: Vec<Contour>,
    segments: Vec<PathSegment>,
    offset: Point,
}
impl PathBuilder {
    pub fn new() -> Self {
        Self {
            contours: vec![],
            segments: vec![],
            offset: Point::ZERO,
        }
    }
    pub fn move_to(&mut self, pos: Point) -> &mut Self {
        self.contours.push(Contour::MoveTo(pos + self.offset));
        self
    }
    pub fn line_to(&mut self, end: Point) -> &mut Self {
        self.contours.push(Contour::LineTo(end + self.offset));
        self
    }
    pub fn quadratic_to(&mut self, control: Point, end: Point) -> &mut Self {
        self.contours.push(Contour::QuadTo(control + self.offset,end + self.offset));
        self
    }
    pub fn cubic_to(&mut self, control1: Point, control2: Point, end: Point) -> &mut Self {
        self.contours.push(Contour::CubicTo(control1 + self.offset,control2 + self.offset,end + self.offset));
        self
    }
    pub fn close_path(&mut self) -> &mut Self {
        self.contours.push(Contour::ClosePath);
        let mut path_segment = PathSegment {contours: vec![]};
        path_segment.contours.append(&mut self.contours);
        self.segments.push(path_segment);
        self.contours.clear();
        self
    }
    pub fn set_offset(&mut self, offset: Point) {
        self.offset = offset;
    }
    pub fn translate_offset(&mut self, offset: Point) {
        self.offset += offset;
    }
    pub fn clear_offset(&mut self)  {
        self.offset = Point::ZERO;
    }
    pub fn build(&mut self) -> PathData {
        let mut path = PathData {segments: vec![]};
        path.segments.append(&mut self.segments);
        path
    }
}

impl rusttype::OutlineBuilder for PathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.move_to(Point::new(x,y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.line_to(Point::new(x,y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.quadratic_to(Point::new(x1,y1),Point::new(x,y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.cubic_to(Point::new(x1,y1),Point::new(x2,y2),Point::new(x,y));
    }

    fn close(&mut self) {
        self.close_path();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_points() {
        let p1 = Point::new(1,2);
        let p2 = Point::new(3,4);
        assert_eq!(p1+p2,Point::new(4,6));
    }
    #[test]
    fn test_subtract_points() {
        let p1 = Point::new(1,2);
        let p2 = Point::new(3,5);
        assert_eq!(p1-p2,Point::new(-2,-3));
    }
    #[test]
    fn test_multiply_points() {
        let p1 = Point::new(2,3);
        let p2 = Point::new(4,5);
        assert_eq!(p1*p2,Point::new(8,15));
    }
    #[test]
    fn test_divide_points() {
        let p1 =  Point::new(3,1);
        let p2 = Point::new(6,4);
        assert_eq!(p1 / p2,Point::new(0.5,0.25));
    }
    #[test]
    fn test_add_point_f32() {
        let p1 = Point::new(1,2);
        assert_eq!(p1 + 5.0,Point::new(6.0,7.0));
    }
    #[test]
    fn test_subtract_point_f32() {
        let p1 = Point::new(1,2);
        assert_eq!(p1 - 5.0,Point::new(-4.0,-3.0));
    }
    #[test]
    fn test_multiply_point_f32() {
        let p1 =  Point::new(3,4);
        assert_eq!(p1 * 5.0,Point::new(15,20));
    }
    #[test]
    fn test_divide_point_f32() {
        let p1 = Point::new(3,4);
        assert_eq!(p1 / 4.0,Point::new(0.75, 1.0));
    }
    #[test]
    fn test_negate_point() {
        let p1 = Point::new(5.0, 6.0);
        assert_eq!(-p1,Point::new(-5.0,-6.0));
    }
    #[test]
    fn test_add_assign_point() {
        let mut p1 = Point::new(1,2);
        p1 += Point::new(3,4);
        assert_eq!(p1,Point::new(4,6));
    }
    #[test]
    fn test_sub_assign_point() {
        let mut p1 = Point::new(1,2);
        p1 -= Point::new(5,4);
        assert_eq!(p1,Point::new(-4,-2));
    }
    #[test]
    fn test_mul_assign_point() {
        let mut p1 = Point::new(2,3);
        p1 *= Point::new(6,5);
        assert_eq!(p1,Point::new(12,15));
    }
    #[test]
    fn test_div_assign_point() {
        let mut p1 = Point::new(6,8);
        p1 /= Point::new(2,4);
        assert_eq!(p1,Point::new(3,2));
    }
    #[test]
    fn test_dot_point() {
        let mut p1 = Point::new(2,3);
        let mut p2 = Point::new(4,5);
        let p3 = p1.dot(&p2);
        let p4 = p2.dot(&p1);
        assert_eq!(p3, 23.0);
        assert_eq!(p3,p4);
    }
}