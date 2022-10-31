use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Copy,Clone,Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
// Pre-defined colors from CSS colors.
impl Color {
    // misc
    pub const TRANSPARENT: Self = Self::new(1.0, 1.0, 1.0, 0.0);
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
    pub fn new(x: f32, y: f32) -> Self {
        Self {x,y}
    }
    pub fn len(&self) -> f32 {
        (self.x*self.x + self.y*self.y).sqrt()
    }
    pub fn norm(&self) -> Self {
        let len = self.len();
        Point::new(self.x / len, self.y / len)
    }
    pub fn angle(&self) -> f32 {
        (self.x / self.y).atan()
    }
    pub fn rotate(&self, a: f32) -> Self {
        let na = self.angle() + a;
        Point::new(self.x * na.cos(), self.y * na.sin())
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
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<f32> for Point {
    type Output = Point;

    fn add(self, rhs: f32) -> Self::Output {
        Point::new(self.x + rhs,self.y + rhs)
    }
}
impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        Point::new(self.x * rhs,self.y * rhs)
    }
}
impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output::new(self.x / rhs,self.y / rhs)
    }
}