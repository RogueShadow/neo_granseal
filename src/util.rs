
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
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const LIME: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0,0.0,1.0);
    pub const YELLOW: Self = Self::rgb(1.0,1.0,0.0);
    pub const CYAN: Self = Self::rgb(0.0,1.0,1.0);
    pub const MAGENTA: Self = Self::rgb(1.0,0.0,1.0);
    pub const SILVER: Self =  Self::rgb(0.75,0.75,0.75);
    pub const GRAY: Self = Self::rgb(0.5,0.5,0.5);
    pub const MAROON: Self = Self::rgb(0.5,0.0,0.0);
    pub const OLIVE: Self = Self::rgb(0.5,0.5,0.0);
    pub const GREEN: Self = Self::rgb(0.0,0.5,0.0);
    pub const PURPLE: Self = Self::rgb(0.5,0.0,0.5);
    pub const TEAL: Self = Self::rgb(0.0,0.5,0.5);
    pub const NAVY: Self = Self::rgb(0.0,0.0,0.5);

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r,g,b,1.0)
    }
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {r,g,b,a}
    }
    pub fn invert(mut self) -> Self {
        self.r = 1.0 - self.r;
        self.g = 1.0 - self.g;
        self.b = 1.0 - self.b;
        self
    }
}
#[derive(Copy,Clone,Debug)]
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}
impl Point2d {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x,y}
    }
    pub fn len(&self) -> f32 {
        (self.x*self.x + self.y*self.y).sqrt()
    }
}