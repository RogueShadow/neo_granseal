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
    pub const BLACK: Self = Self::rgb(0.0,0.0,0.0);
    pub const WHITE: Self = Self::rgb(1.0,1.0,1.0);
    pub const BLUE: Self = Self::rgb(0.0,0.0,1.0);
    pub const CYAN: Self = Self::rgb(0.0,1.0,1.0);
    pub const MAGENTA: Self = Self::rgb(1.0,0.0,1.0);
    pub const SILVER: Self =  Self::rgb(0.75,0.75,0.75);
    pub const GRAY: Self = Self::rgb(0.5,0.5,0.5);
    pub const DIM_GRAY: Self = Self::rgb(0.4117647,0.4117647,0.4117647);
    pub const MAROON: Self = Self::rgb(0.5,0.0,0.0);
    pub const PURPLE: Self = Self::rgb(0.5,0.0,0.5);
    pub const TEAL: Self = Self::rgb(0.0,0.5,0.5);
    pub const NAVY: Self = Self::rgb(0.0,0.0,0.5);
    pub const TRANSPARENT: Self = Self::new(1.0, 1.0, 1.0, 0.0);
    pub const SADDLE_BROWN: Self = Self::rgb(0.5451, 0.2706, 0.0745);
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

    // blue colors

    // purple colors

    // pink colors

    // white colors

    // gray colors

    // brown colors


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