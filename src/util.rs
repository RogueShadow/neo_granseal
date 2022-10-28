use std::ops::{Add, Mul, Sub};
use std::str::FromStr;
use crate::util;

#[derive(Copy,Clone,Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Color {
    pub const BLACK: Self = Self::rgb(0.0,0.0,0.0);
    pub const WHITE: Self = Self::rgb(1.0,1.0,1.0);
    pub const LIME: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0,0.0,1.0);
    pub const YELLOW: Self = Self::rgb(1.0,1.0,0.0);
    pub const CYAN: Self = Self::rgb(0.0,1.0,1.0);
    pub const MAGENTA: Self = Self::rgb(1.0,0.0,1.0);
    pub const SILVER: Self =  Self::rgb(0.75,0.75,0.75);
    pub const GRAY: Self = Self::rgb(0.5,0.5,0.5);
    pub const DIM_GRAY: Self = Self::rgb(0.4117647,0.4117647,0.4117647);
    pub const MAROON: Self = Self::rgb(0.5,0.0,0.0);
    pub const OLIVE: Self = Self::rgb(0.5,0.5,0.0);
    pub const GREEN: Self = Self::rgb(0.0,0.5,0.0);
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

    // green colors

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
            (r as f32/u8::MAX as f32),
            (g as f32/u8::MAX as f32),
            (b as f32/u8::MAX as f32),
            1.0)
    }
    pub fn invert(mut self) -> Self {
        self.r = 1.0 - self.r;
        self.g = 1.0 - self.g;
        self.b = 1.0 - self.b;
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
#[derive(Copy,Clone,Debug)]
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