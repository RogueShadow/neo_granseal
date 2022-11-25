use std::fmt::{Display, Formatter};
use std::ops::*;
use num_traits::AsPrimitive;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Matrix3x3 {
    row0: [f32;3],
    row1: [f32;3],
    row2: [f32;3],
}
impl Matrix3x3 {
    pub fn new() -> Self {
        Self {
            row0: [0.0, 0.0, 0.0],
            row1: [0.0, 0.0, 0.0],
            row2: [0.0, 0.0, 0.0],
        }
    }
    pub fn identity() -> Self {
        Self {
            row0: [1.0, 0.0, 0.0],
            row1: [0.0, 1.0, 0.0],
            row2: [0.0, 0.0, 1.0],
        }
    }
    pub fn column(&self, index: usize) -> Vec3 {
        Vec3 {
            x: self.row0[index],
            y: self.row1[index],
            z: self.row2[index],
        }
    }
    pub fn row(&self,  index: usize) -> Vec3 {
        return match index {
            0 => Vec3::new(self.row0[0],self.row0[1], self.row0[2]),
            1 => Vec3::new(self.row1[0],self.row1[1], self.row1[2]),
            2 => Vec3::new(self.row2[0],self.row2[1], self.row2[2]),
            _ => {panic!("Index out of bounds.")}
        }
    }
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }
    pub fn determinant(&self) -> f32 {
        self.row0[0] * self.row1[1] * self.row2[2] +
        self.row0[1] * self.row1[2] * self.row2[0] +
        self.row0[2] * self.row1[0] * self.row2[1] -
        self.row0[2] * self.row1[1] * self.row2[0] -
        self.row0[1] * self.row1[0] * self.row2[2] -
        self.row0[0] * self.row1[2] * self.row2[1]
    }
    pub fn transpose(&self) -> Self {
        Self {
            row0: [self.row0[0],self.row1[0],self.row2[0]],
            row1: [self.row0[1],self.row1[1],self.row2[1]],
            row2: [self.row0[2],self.row1[2],self.row2[2]],
        }
    }
}

impl Display for  Matrix3x3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!("{:5} {:5} {:5}\n{:5} {:5} {:5}\n{:5} {:5} {:5}\n",
            self.row0[0],self.row0[1],self.row0[2],
            self.row1[0],self.row1[1],self.row1[2],
            self.row2[0],self.row2[1],self.row2[2]
        ).as_str())
    }
}
impl Display for  Matrix2x2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!("{:5} {:5}\n{:5} {:5}\n",
                self.row0[0],self.row0[1],
                self.row1[0],self.row1[1]).as_str()
        )
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Matrix2x2 {
    row0: [f32;2],
    row1: [f32;2],
}
impl Matrix2x2 {
    pub fn new() -> Self {
        Self {
            row0: [0.0,0.0],
            row1: [0.0,0.0],
        }
    }
    pub fn identity() -> Self {
        Self {
            row0: [1.0,0.0],
            row1: [0.0,1.0],
        }
    }
    pub fn row(&self, index: usize) -> Vec2 {
        match index {
            0 => Vec2::new(self.row0[0],self.row0[1]),
            1 => Vec2::new(self.row1[0],self.row1[1]),
            _ => panic!("Index out of bounds.")
        }
    }
    pub fn column(&self, index: usize) -> Vec2 {
        Vec2 {
            x: self.row0[index],
            y: self.row1[index],
        }
    }
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }
    pub fn determinant(&self) -> f32 {
        self.row0[0] * self.row1[1] - self.row0[1] * self.row1[0]
    }
}

///Column Vector, 3 components.
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn new(x: impl AsPrimitive<f32>, y: impl AsPrimitive<f32>, z: impl AsPrimitive<f32>) -> Self {
        Vec3 {
            x: x.as_(),
            y: y.as_(),
            z: z.as_(),
        }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn angle(&self) -> (f32,f32) {
        let phi = (self.y / self.x).atan();
        let theta = (self.z / self.magnitude() ).acos();
        (phi,theta)
    }
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}
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
    pub fn magnitude(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let len = self.magnitude();
        Vec2::new(self.x / len, self.y / len)
    }
    pub fn angle(&self) -> f32 {
        (self.y / self.x).atan()
    }
    pub fn angle2(&self) -> f32 {
        self.y.atan2(self.x)
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn project(&self, rhs: &Self) -> f32 {
        self.dot(rhs) / rhs.magnitude()
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
        let p1 = Vec2::new(2,3);
        let p2 = Vec2::new(4,5);
        let p3 = p1.dot(&p2);
        let p4 = p2.dot(&p1);
        assert_eq!(p3, 23.0);
        assert_eq!(p3,p4);
    }
    #[test]
    fn test_matrix2_rows_cols() {
        let mut matrix = Matrix2x2::new();
        matrix.row0 = [1.0,2.0];
        matrix.row1 = [3.0,4.0];
        assert_eq!(matrix.row(0),Vec2::new(1.0,2.0));
        assert_eq!(matrix.row(1), Vec2::new(3.0,4.0));
        assert_eq!(matrix.column(0), Vec2::new(1.0,3.0));
        assert_eq!(matrix.column(1), Vec2::new(2.0,4.0));
    }
    #[test]
    fn test_matrix3_mul_matrix3() {
        let ma = Matrix3x3 {
            row0: [1.0, 0.0, 0.0],
            row1: [-3.0, 1.0, 0.0],
            row2: [0.0, 0.0, 1.0],
        };
        let mb = Matrix3x3 {
            row0: [1.0, 2.0, 1.0],
            row1: [3.0, 8.0, 1.0],
            row2: [0.0, 4.0, 1.0],
        };
        let check = Matrix3x3 {
            row0: [1.0, 2.0, 1.0],
            row1: [0.0, 2.0, -2.0],
            row2: [0.0, 4.0, 1.0],
        };
        let result = ma * mb;
        assert_eq!(check,result);
    }
    #[test]
    fn test_matrix2_mul_matrix2() {
        let ma = Matrix2x2 {
            row0: [1.0, 0.0],
            row1: [-3.0, 1.0],
        };
        let mb = Matrix2x2 {
            row0: [1.0, 2.0],
            row1: [3.0, 8.0],
        };
        let check = Matrix2x2 {
            row0: [1.0, 2.0],
            row1: [0.0, 2.0],
        };
        let result = ma * mb;
        assert_eq!(check,result);
    }
    #[test]
    fn test_matrix2x2_invertible() {
        let matrix = Matrix2x2 {
            row0: [1.0, 2.0],
            row1: [2.0, 4.0],
        };
        assert_eq!(matrix.is_invertible(),false);
        let matrix2 = Matrix2x2 {
            row0: [2.0, 6.0],
            row1: [3.0, 10.0],
        };
        assert_eq!(matrix2.is_invertible(),true);
    }
    #[test]
    fn test_matrix3x3_invertible() {
        let matrix = Matrix3x3 {
            row0: [1.0, 2.0, 4.0],
            row1: [2.0, 4.0, 8.0],
            row2: [2.0, 4.0, 8.0],
        };
        assert_eq!(matrix.is_invertible(),false);
        let matrix2 = Matrix3x3 {
            row0: [1.0, 2.0, 3.0],
            row1: [2.0, 4.0, 3.0],
            row2: [2.0, 8.0, 2.0],
        };
        assert_eq!(matrix2.is_invertible(),true);
    }
    #[test]
    fn test_matrix2x2_determinant() {
        let matrix = Matrix2x2 {
            row0: [3.0, 7.0],
            row1: [1.0, -4.0],
        };
        assert_eq!(matrix.determinant(), -19.0);
        println!("{}",matrix);
    }
    #[test]
    fn test_matrix3x3_determinant() {
        let matrix = Matrix3x3 {
            row0: [-2.0, -1.0, 2.0],
            row1: [2.0, 1.0, 4.0],
            row2: [-3.0, 3.0, -1.0],
        };
        assert_eq!(matrix.determinant(), 54.0);
    }
}

impl Mul<Matrix2x2> for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Matrix2x2) -> Self::Output {
        Self {
            row0: [self.row(0).dot(&rhs.column(0)),self.row(0).dot(&rhs.column(1))],
            row1: [self.row(1).dot(&rhs.column(0)),self.row(1).dot(&rhs.column(1))],
        }
    }
}
impl Mul<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        Self {
            row0: [self.row(0).dot(&rhs.column(0)), self.row(0).dot(&rhs.column(1)), self.row(0).dot(&rhs.column(2))],
            row1: [self.row(1).dot(&rhs.column(0)), self.row(1).dot(&rhs.column(1)), self.row(1).dot(&rhs.column(2))],
            row2: [self.row(2).dot(&rhs.column(0)), self.row(2).dot(&rhs.column(1)), self.row(2).dot(&rhs.column(2))],
        }
    }
}