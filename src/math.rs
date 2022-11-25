use std::ops::*;
use num_traits::AsPrimitive;

///Column Vector, 2 components.
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0};
    pub fn new(x: impl AsPrimitive<f32>, y: impl AsPrimitive<f32>) -> Self {
        Self {x: x.as_(),y: y.as_()}
    }
    pub fn len(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    pub fn norm(&self) -> Self {
        let len = self.len();
        Vec2::new(self.x / len, self.y / len)
    }
    pub fn angle(&self) -> f32 {
        (self.y / self.x).atan()
    }
    pub fn angle2(&self) -> f32 {
        self.y.atan2(self.x)
    }
    pub fn rotate(&self, a: f32) -> Self {
        let na = self.angle() + a;
        Vec2::new(self.x * na.cos(), self.y * na.sin())
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn proj(&self, rhs: &Self) -> f32 {
        self.dot(rhs) / rhs.len()
    }
}
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}
impl Div for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}
impl <T>Add<T> for Vec2 where T: AsPrimitive<f32> {
    type Output = Vec2;

    fn add(self, rhs: T) -> Self::Output {
        Vec2::new(self.x + rhs.as_(), self.y + rhs.as_())
    }
}
impl <T>Sub<T> for Vec2 where T: AsPrimitive<f32> {
    type Output = Vec2;

    fn sub(self, rhs: T) -> Self::Output {
        Vec2::new(self.x - rhs.as_(), self.y - rhs.as_())
    }
}
impl <T>Mul<T> for Vec2 where T: AsPrimitive<f32> {
    type Output = Vec2;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2::new(self.x * rhs.as_(), self.y * rhs.as_())
    }
}
impl <T>Div<T> for Vec2 where T: AsPrimitive<f32> {
    type Output = Vec2;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output::new(self.x / rhs.as_(),self.y / rhs.as_())
    }
}
impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl Mul<Vec2> for f32  {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self * rhs.x, self * rhs.y)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_points() {
        let p1 = Vec2::new(1,2);
        let p2 = Vec2::new(3,4);
        assert_eq!(p1+p2,Vec2::new(4,6));
    }
    #[test]
    fn test_subtract_points() {
        let p1 = Vec2::new(1,2);
        let p2 = Vec2::new(3,5);
        assert_eq!(p1-p2,Vec2::new(-2,-3));
    }
    #[test]
    fn test_multiply_points() {
        let p1 = Vec2::new(2,3);
        let p2 = Vec2::new(4,5);
        assert_eq!(p1*p2,Vec2::new(8,15));
    }
    #[test]
    fn test_divide_points() {
        let p1 =  Vec2::new(3,1);
        let p2 = Vec2::new(6,4);
        assert_eq!(p1 / p2,Vec2::new(0.5,0.25));
    }
    #[test]
    fn test_add_vec2_f32() {
        let p1 = Vec2::new(1,2);
        assert_eq!(p1 + 5.0,Vec2::new(6.0,7.0));
    }
    #[test]
    fn test_subtract_vec2_f32() {
        let p1 = Vec2::new(1,2);
        assert_eq!(p1 - 5.0,Vec2::new(-4.0,-3.0));
    }
    #[test]
    fn test_multiply_vec2_f32() {
        let p1 =  Vec2::new(3,4);
        assert_eq!(p1 * 5.0,Vec2::new(15,20));
    }
    #[test]
    fn test_divide_vec2_f32() {
        let p1 = Vec2::new(3,4);
        assert_eq!(p1 / 4.0,Vec2::new(0.75, 1.0));
    }
    #[test]
    fn test_negate_vec2() {
        let p1 = Vec2::new(5.0, 6.0);
        assert_eq!(-p1,Vec2::new(-5.0,-6.0));
    }
    #[test]
    fn test_add_assign_vec2() {
        let mut p1 = Vec2::new(1,2);
        p1 += Vec2::new(3,4);
        assert_eq!(p1,Vec2::new(4,6));
    }
    #[test]
    fn test_sub_assign_vec2() {
        let mut p1 = Vec2::new(1,2);
        p1 -= Vec2::new(5,4);
        assert_eq!(p1,Vec2::new(-4,-2));
    }
    #[test]
    fn test_mul_assign_vec2() {
        let mut p1 = Vec2::new(2,3);
        p1 *= Vec2::new(6,5);
        assert_eq!(p1,Vec2::new(12,15));
    }
    #[test]
    fn test_div_assign_vec2() {
        let mut p1 = Vec2::new(6,8);
        p1 /= Vec2::new(2,4);
        assert_eq!(p1,Vec2::new(3,2));
    }
    #[test]
    fn test_dot_vec2() {
        let mut p1 = Vec2::new(2,3);
        let mut p2 = Vec2::new(4,5);
        let p3 = p1.dot(&p2);
        let p4 = p2.dot(&p1);
        assert_eq!(p3, 23.0);
        assert_eq!(p3,p4);
    }
}