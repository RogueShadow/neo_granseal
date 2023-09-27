use num_traits::AsPrimitive;
use std::fmt::{Display, Formatter};
use std::ops::*;

/**
    Matrix Structs.
**/

/// 4x4 Matrix
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix4x4 {
    pub row1: [f32; 4],
    pub row2: [f32; 4],
    pub row3: [f32; 4],
    pub row4: [f32; 4],
}
impl Default for Matrix4x4 {
    fn default() -> Self {
        Self {
            row1: [1.0, 0.0, 0.0, 0.0],
            row2: [0.0, 1.0, 0.0, 0.0],
            row3: [0.0, 0.0, 1.0, 0.0],
            row4: [0.0, 0.0, 0.0, 1.0],
        }
    }
}
impl Matrix4x4 {
    pub fn projection(ar: f32, fov: f32, zfar: f32, znear: f32) -> Self {
        let fov = fov.to_radians();
        let f = 1.0 / f32::tan(fov * 0.5);
        let q = zfar / (zfar - znear);

        Self {
            row1: [ar * f, 0.0, 0.0, 0.0],
            row2: [0.0, f, 0.0, 0.0],
            row3: [0.0, 0.0, q, 1.0],
            row4: [0.0, 0.0, -(znear * q), 0.0],
        }
    }
    pub fn translation<T>(x: T, y: T, z: T) -> Self
    where
        T: AsPrimitive<f32>,
    {
        Self {
            row1: [1.0, 0.0, 0.0, x.as_()],
            row2: [0.0, 1.0, 0.0, y.as_()],
            row3: [0.0, 0.0, 1.0, z.as_()],
            row4: [0.0, 0.0, 0.0, 1.0],
        }
    }
    pub fn rotation_x(theta: f32) -> Self {
        Self {
            row1: [1.0, 0.0, 0.0, 0.0],
            row2: [0.0, theta.cos(), -theta.sin(), 0.0],
            row3: [0.0, theta.sin(), theta.cos(), 0.0],
            row4: [0.0, 0.0, 0.0, 1.0],
        }
    }
    pub fn rotation_y(theta: f32) -> Self {
        Self {
            row1: [theta.cos(), 0.0, theta.sin(), 0.0],
            row2: [0.0, 1.0, 0.0, 0.0],
            row3: [-theta.sin(), 0.0, theta.cos(), 0.0],
            row4: [0.0, 0.0, 0.0, 1.0],
        }
    }
    pub fn rotation_z(theta: f32) -> Self {
        Self {
            row1: [theta.cos(), -theta.sin(), 0.0, 0.0],
            row2: [theta.sin(), theta.cos(), 0.0, 0.0],
            row3: [0.0, 0.0, 1.0, 0.0],
            row4: [0.0, 0.0, 0.0, 1.0],
        }
    }
    pub fn point_at(position: &Vec3, target: &Vec3, up: &Vec3) -> Matrix4x4 {
        let forward = (*target - *position).normalize();

        let a = forward * (up.dot(&forward));
        let up = (*up - a).normalize();
        let right = up.cross(&forward);
        Matrix4x4 {
            row1: [right.x, right.y, right.z, 0.0],
            row2: [up.x, up.y, up.z, 0.0],
            row3: [forward.x, forward.y, forward.z, 0.0],
            row4: [position.x, position.y, position.z, 1.0],
        }
    }
    /// Column's indexes are 1,2,3,4 to follow standard math notation.
    pub fn column(&self, index: usize) -> Vec4 {
        Vec4 {
            x: self.row1[index - 1],
            y: self.row2[index - 1],
            z: self.row3[index - 1],
            w: self.row4[index - 1],
        }
    }
    /// Row's indexes are 1,2,3,4 to follow standard math notation.
    pub fn row(&self, index: usize) -> Vec4 {
        match index {
            1 => Vec4::new(self.row1[0], self.row1[1], self.row1[2], self.row1[3]),
            2 => Vec4::new(self.row2[0], self.row2[1], self.row2[2], self.row2[3]),
            3 => Vec4::new(self.row3[0], self.row3[1], self.row3[2], self.row3[3]),
            4 => Vec4::new(self.row4[0], self.row4[1], self.row4[2], self.row4[3]),
            _ => {
                panic!("Index out of bounds.")
            }
        }
    }
    pub fn transpose(&self) -> Self {
        Self {
            row1: [self.row1[0], self.row2[0], self.row3[0], self.row4[0]],
            row2: [self.row1[1], self.row2[1], self.row3[1], self.row4[1]],
            row3: [self.row1[2], self.row2[2], self.row3[2], self.row4[2]],
            row4: [self.row1[3], self.row2[3], self.row3[3], self.row4[3]],
        }
    }
    pub fn coefficient(&self, row: usize, column: usize) -> f32 {
        match (row, column) {
            (1, 1) => self.row1[0],
            (1, 2) => self.row1[1],
            (1, 3) => self.row1[2],
            (1, 4) => self.row1[3],
            (2, 1) => self.row2[0],
            (2, 2) => self.row2[1],
            (2, 3) => self.row2[2],
            (2, 4) => self.row2[3],
            (3, 1) => self.row3[0],
            (3, 2) => self.row3[1],
            (3, 3) => self.row3[2],
            (3, 4) => self.row3[3],
            (4, 1) => self.row4[0],
            (4, 2) => self.row4[1],
            (4, 3) => self.row4[2],
            (4, 4) => self.row4[3],
            _ => panic!("Index out of bounds, row/col"),
        }
    }
    pub fn minor(&self, row: usize, column: usize) -> Matrix3x3 {
        let row = (1..=4).filter(|r| *r != row).collect::<Vec<usize>>();
        let column = (1..=4).filter(|c| *c != column).collect::<Vec<usize>>();
        Matrix3x3 {
            row1: [
                self.coefficient(row[0], column[0]),
                self.coefficient(row[0], column[1]),
                self.coefficient(row[0], column[2]),
            ],
            row2: [
                self.coefficient(row[1], column[0]),
                self.coefficient(row[1], column[1]),
                self.coefficient(row[1], column[2]),
            ],
            row3: [
                self.coefficient(row[2], column[0]),
                self.coefficient(row[2], column[1]),
                self.coefficient(row[2], column[2]),
            ],
        }
    }
    pub fn determinant(&self) -> f32 {
        self.coefficient(1, 1) * self.minor(1, 1).determinant()
            - self.coefficient(1, 2) * self.minor(1, 2).determinant()
            + self.coefficient(1, 3) * self.minor(1, 3).determinant()
            - self.coefficient(1, 4) * self.minor(1, 4).determinant()
    }
    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        f32::powf(-1.0, (row + column) as f32) * self.minor(row, column).determinant()
    }
    pub fn adjugate(&self) -> Self {
        let cofactor_matrix = Self {
            row1: [
                self.cofactor(1, 1),
                self.cofactor(1, 2),
                self.cofactor(1, 3),
                self.cofactor(1, 4),
            ],
            row2: [
                self.cofactor(2, 1),
                self.cofactor(2, 2),
                self.cofactor(2, 3),
                self.cofactor(2, 4),
            ],
            row3: [
                self.cofactor(3, 1),
                self.cofactor(3, 2),
                self.cofactor(3, 3),
                self.cofactor(3, 4),
            ],
            row4: [
                self.cofactor(4, 1),
                self.cofactor(4, 2),
                self.cofactor(4, 3),
                self.cofactor(4, 4),
            ],
        };
        cofactor_matrix.transpose()
    }
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if !(-f32::EPSILON..=f32::EPSILON).contains(&det) {
            let scalar = 1.0 / det;
            Some(scalar * self.adjugate())
        } else {
            None
        }
    }
}

/// 3x3 Matrix
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix3x3 {
    pub row1: [f32; 3],
    pub row2: [f32; 3],
    pub row3: [f32; 3],
}
impl Default for Matrix3x3 {
    fn default() -> Self {
        Self {
            row1: [1.0, 0.0, 0.0],
            row2: [0.0, 1.0, 0.0],
            row3: [0.0, 0.0, 1.0],
        }
    }
}
impl Matrix3x3 {
    pub fn rotation_x(theta: f32) -> Self {
        Self {
            row1: [1.0, 0.0, 0.0],
            row2: [0.0, theta.cos(), -theta.sin()],
            row3: [0.0, theta.sin(), theta.cos()],
        }
    }
    pub fn rotation_y(theta: f32) -> Self {
        Self {
            row1: [theta.cos(), 0.0, theta.sin()],
            row2: [0.0, 1.0, 0.0],
            row3: [-theta.sin(), 0.0, theta.cos()],
        }
    }
    pub fn rotation_z(theta: f32) -> Self {
        Self {
            row1: [theta.cos(), -theta.sin(), 0.0],
            row2: [theta.sin(), theta.cos(), 0.0],
            row3: [0.0, 0.0, 1.0],
        }
    }
    /// Column's indexes are 1,2,3 to follow standard math notation.
    pub fn column(&self, index: usize) -> Vec3 {
        Vec3 {
            x: self.row1[index - 1],
            y: self.row2[index - 1],
            z: self.row3[index - 1],
            w: 1.0,
        }
    }
    /// Row's indexes are 1,2,3 to follow standard math notation.
    pub fn row(&self, index: usize) -> Vec3 {
        match index {
            1 => Vec3::new(self.row1[0], self.row1[1], self.row1[2]),
            2 => Vec3::new(self.row2[0], self.row2[1], self.row2[2]),
            3 => Vec3::new(self.row3[0], self.row3[1], self.row3[2]),
            _ => {
                panic!("Index out of bounds.")
            }
        }
    }
    pub fn determinant(&self) -> f32 {
        self.coefficient(1, 1) * self.minor(1, 1).determinant()
            - self.coefficient(1, 2) * self.minor(1, 2).determinant()
            + self.coefficient(1, 3) * self.minor(1, 3).determinant()
    }
    pub fn transpose(&self) -> Self {
        Self {
            row1: [self.row1[0], self.row2[0], self.row3[0]],
            row2: [self.row1[1], self.row2[1], self.row3[1]],
            row3: [self.row1[2], self.row2[2], self.row3[2]],
        }
    }
    pub fn minor(&self, row: usize, column: usize) -> Matrix2x2 {
        let row = (1..=3).filter(|r| *r != row).collect::<Vec<usize>>();
        let column = (1..=3).filter(|c| *c != column).collect::<Vec<usize>>();
        Matrix2x2 {
            row1: [
                self.coefficient(row[0], column[0]),
                self.coefficient(row[0], column[1]),
            ],
            row2: [
                self.coefficient(row[1], column[0]),
                self.coefficient(row[1], column[1]),
            ],
        }
    }
    pub fn coefficient(&self, row: usize, column: usize) -> f32 {
        match (row, column) {
            (1, 1) => self.row1[0],
            (1, 2) => self.row1[1],
            (1, 3) => self.row1[2],
            (2, 1) => self.row2[0],
            (2, 2) => self.row2[1],
            (2, 3) => self.row2[2],
            (3, 1) => self.row3[0],
            (3, 2) => self.row3[1],
            (3, 3) => self.row3[2],
            _ => panic!("Index out of bounds, row/col"),
        }
    }
    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        f32::powf(-1.0, (row + column) as f32) * self.minor(row, column).determinant()
    }
    pub fn adjugate(&self) -> Self {
        let cofactor_matrix = Self {
            row1: [
                self.cofactor(1, 1),
                self.cofactor(1, 2),
                self.cofactor(1, 3),
            ],
            row2: [
                self.cofactor(2, 1),
                self.cofactor(2, 2),
                self.cofactor(2, 3),
            ],
            row3: [
                self.cofactor(3, 1),
                self.cofactor(3, 2),
                self.cofactor(3, 3),
            ],
        };
        cofactor_matrix.transpose()
    }
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if !(-f32::EPSILON..=f32::EPSILON).contains(&det) {
            let scalar = 1.0 / det;
            Some(scalar * self.adjugate())
        } else {
            None
        }
    }
}

/// 2x2 Matrix
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix2x2 {
    pub row1: [f32; 2],
    pub row2: [f32; 2],
}
impl Default for Matrix2x2 {
    fn default() -> Self {
        Self {
            row1: [1.0, 0.0],
            row2: [0.0, 1.0],
        }
    }
}
impl Matrix2x2 {
    pub fn new() -> Self {
        Self {
            row1: [1.0, 0.0],
            row2: [0.0, 1.0],
        }
    }
    pub fn row(&self, index: usize) -> Vec2 {
        match index {
            1 => Vec2::new(self.row1[0], self.row1[1]),
            2 => Vec2::new(self.row2[0], self.row2[1]),
            _ => panic!("Index out of bounds."),
        }
    }
    pub fn column(&self, index: usize) -> Vec2 {
        Vec2 {
            x: self.row1[index - 1],
            y: self.row2[index - 1],
        }
    }
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }
    pub fn determinant(&self) -> f32 {
        self.row1[0] * self.row2[1] - self.row1[1] * self.row2[0]
    }
    pub fn transpose(&self) -> Self {
        Self {
            row1: [self.row1[0], self.row2[0]],
            row2: [self.row1[1], self.row2[1]],
        }
    }
    pub fn coefficient(&self, row: usize, column: usize) -> f32 {
        match (row, column) {
            (1, 1) => self.row1[0],
            (1, 2) => self.row1[1],
            (2, 1) => self.row2[0],
            (2, 2) => self.row2[1],
            _ => panic!("Index out of bounds, row/col"),
        }
    }
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if !(-f32::EPSILON..=f32::EPSILON).contains(&det) {
            let scalar = 1.0 / det;
            let step1 = Self {
                row1: [self.row2[1], -self.row1[1]],
                row2: [-self.row2[0], self.row1[0]],
            };
            Some(scalar * step1)
        } else {
            None
        }
    }
}

/**
       Vector Structs
**/

/// 4 component Vector x y z w
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vec4 {
    pub fn new(
        x: impl AsPrimitive<f32>,
        y: impl AsPrimitive<f32>,
        z: impl AsPrimitive<f32>,
        w: impl AsPrimitive<f32>,
    ) -> Self {
        Self {
            x: x.as_(),
            y: y.as_(),
            z: z.as_(),
            w: w.as_(),
        }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 1.0,
        }
    }
    pub fn to_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
    pub fn cross(&self, rhs: &Self) -> Self {
        (self.to_vec3().cross(&rhs.to_vec3())).to_vec4()
    }
}

/// 3 component Vector x y z. Also w, so you can treat it like a 3d vector, and still use the transformation matrices.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vec3 {
    pub fn new(
        x: impl AsPrimitive<f32>,
        y: impl AsPrimitive<f32>,
        z: impl AsPrimitive<f32>,
    ) -> Self {
        Self {
            x: x.as_(),
            y: y.as_(),
            z: z.as_(),
            w: 1.0,
        }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            w: self.w,
        }
    }
    pub fn angle(&self) -> (f32, f32) {
        let phi = (self.y / self.x).atan();
        let theta = (self.z / self.magnitude()).acos();
        (phi, theta)
    }
    pub fn magnitude(&self) -> f32 {
        (self.dot(self)).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let len = self.magnitude();
        Self::new(self.x / len, self.y / len, self.z / len)
    }
    pub fn to_vec4(&self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
    pub fn to_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

/// 2 component Vector x y
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub fn new(x: impl AsPrimitive<f32>, y: impl AsPrimitive<f32>) -> Self {
        Self {
            x: x.as_(),
            y: y.as_(),
        }
    }
    pub fn magnitude(&self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let len = self.magnitude();
        Self::new(self.x / len, self.y / len)
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
    pub fn to_vec4(&self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: 0.0,
            w: 1.0,
        }
    }
    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: 0.0,
            w: 1.0,
        }
    }
}

/**
       Operator Overrides
**/

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl Mul for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}
impl Div for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}
impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}
impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}
impl MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}
impl DivAssign for Vec4 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}
impl Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
impl<T> Add<T> for Vec4
where
    T: AsPrimitive<f32>,
{
    type Output = Vec4;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs,
        }
    }
}
impl<T> Sub<T> for Vec4
where
    T: AsPrimitive<f32>,
{
    type Output = Vec4;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs,
        }
    }
}
impl<T> Mul<T> for Vec4
where
    T: AsPrimitive<f32>,
{
    type Output = Vec4;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
impl<T> Div<T> for Vec4
where
    T: AsPrimitive<f32>,
{
    type Output = Vec4;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
impl<T> AddAssign<T> for Vec4
where
    T: AsPrimitive<f32>,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.as_();
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.w += rhs;
    }
}
impl<T> SubAssign<T> for Vec4
where
    T: AsPrimitive<f32>,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.as_();
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}
impl<T> MulAssign<T> for Vec4
where
    T: AsPrimitive<f32>,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.as_();
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}
impl<T> DivAssign<T> for Vec4
where
    T: AsPrimitive<f32>,
{
    fn div_assign(&mut self, rhs: T) {
        let d = rhs.as_();
        self.x /= d;
        self.y /= d;
        self.z /= d;
        self.w /= d;
    }
}
/////////////////////////////////////////////////
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w,
        }
    }
}
impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w,
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}
impl<T> Add<T> for Vec3
where
    T: AsPrimitive<f32>,
{
    type Output = Vec3;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w,
        }
    }
}
impl<T> Sub<T> for Vec3
where
    T: AsPrimitive<f32>,
{
    type Output = Vec3;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w,
        }
    }
}
impl<T> Mul<T> for Vec3
where
    T: AsPrimitive<f32>,
{
    type Output = Vec3;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w,
        }
    }
}
impl<T> Div<T> for Vec3
where
    T: AsPrimitive<f32>,
{
    type Output = Vec3;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w,
        }
    }
}
impl<T> AddAssign<T> for Vec3
where
    T: AsPrimitive<f32>,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.as_();
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}
impl<T> SubAssign<T> for Vec3
where
    T: AsPrimitive<f32>,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.as_();
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}
impl<T> MulAssign<T> for Vec3
where
    T: AsPrimitive<f32>,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.as_();
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl<T> DivAssign<T> for Vec3
where
    T: AsPrimitive<f32>,
{
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.as_();
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

//////////////////////////////

// impl <T>Mul<Vec2> for T where T: AsPrimitive<f32> {
//     type Output = Vec2;
//
//     fn mul(self, rhs: Vec2) -> Self::Output {
//         Vec2 {
//             x: self.as_() * rhs.x,
//             y: self.as_() * rhs.y,
//         }
//     }
// }

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
impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}
impl<T> Add<T> for Vec2
where
    T: AsPrimitive<f32>,
{
    type Output = Vec2;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
impl<T> Sub<T> for Vec2
where
    T: AsPrimitive<f32>,
{
    type Output = Vec2;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}
impl<T> Mul<T> for Vec2
where
    T: AsPrimitive<f32>,
{
    type Output = Vec2;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl<T> Div<T> for Vec2
where
    T: AsPrimitive<f32>,
{
    type Output = Vec2;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl<T> AddAssign<T> for Vec2
where
    T: AsPrimitive<f32>,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.as_();
        self.x += rhs;
        self.y += rhs;
    }
}
impl<T> SubAssign<T> for Vec2
where
    T: AsPrimitive<f32>,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.as_();
        self.x -= rhs;
        self.y -= rhs;
    }
}
impl<T> MulAssign<T> for Vec2
where
    T: AsPrimitive<f32>,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.as_();
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl<T> DivAssign<T> for Vec2
where
    T: AsPrimitive<f32>,
{
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.as_();
        self.x /= rhs;
        self.y /= rhs;
    }
}

/////////////////////////////////////////////////////

impl Add<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, rhs: Matrix4x4) -> Self::Output {
        Matrix4x4 {
            row1: [
                self.row1[0] + rhs.row1[0],
                self.row1[1] + rhs.row1[1],
                self.row1[2] + rhs.row1[2],
                self.row1[3] + rhs.row1[3],
            ],
            row2: [
                self.row2[0] + rhs.row2[0],
                self.row2[1] + rhs.row2[1],
                self.row2[2] + rhs.row2[2],
                self.row2[3] + rhs.row2[3],
            ],
            row3: [
                self.row3[0] + rhs.row3[0],
                self.row3[1] + rhs.row3[1],
                self.row3[2] + rhs.row3[2],
                self.row3[3] + rhs.row3[3],
            ],
            row4: [
                self.row4[0] + rhs.row4[0],
                self.row4[1] + rhs.row4[1],
                self.row4[2] + rhs.row4[2],
                self.row4[3] + rhs.row4[3],
            ],
        }
    }
}
impl Add<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn add(self, rhs: Matrix3x3) -> Self::Output {
        Matrix3x3 {
            row1: [
                self.row1[0] + rhs.row1[0],
                self.row1[1] + rhs.row1[1],
                self.row1[2] + rhs.row1[2],
            ],
            row2: [
                self.row2[0] + rhs.row2[0],
                self.row2[1] + rhs.row2[1],
                self.row2[2] + rhs.row2[2],
            ],
            row3: [
                self.row3[0] + rhs.row3[0],
                self.row3[1] + rhs.row3[1],
                self.row3[2] + rhs.row3[2],
            ],
        }
    }
}
impl Add<Matrix2x2> for Matrix2x2 {
    type Output = Matrix2x2;

    fn add(self, rhs: Matrix2x2) -> Self::Output {
        Matrix2x2 {
            row1: [self.row1[0] + rhs.row1[0], self.row1[1] + rhs.row1[1]],
            row2: [self.row2[0] + rhs.row2[0], self.row2[1] + rhs.row2[1]],
        }
    }
}
impl Display for Matrix4x4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "{:5} {:5} {:5} {:5}\n{:5} {:5} {:5} {:5}\n{:5} {:5} {:5} {:5}\n{:5} {:5} {:5} {:5}\n",
                self.row1[0], self.row1[1], self.row1[2], self.row1[3],
                self.row2[0], self.row2[1], self.row2[2], self.row2[3],
                self.row3[0], self.row3[1], self.row3[2], self.row3[3],
                self.row4[0], self.row4[1], self.row4[2], self.row4[3],
            ).as_str(),
        )
    }
}
impl Display for Matrix3x3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "{:5} {:5} {:5}\n{:5} {:5} {:5}\n{:5} {:5} {:5}\n",
                self.row1[0],
                self.row1[1],
                self.row1[2],
                self.row2[0],
                self.row2[1],
                self.row2[2],
                self.row3[0],
                self.row3[1],
                self.row3[2]
            )
            .as_str(),
        )
    }
}
impl Display for Matrix2x2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "{:5} {:5}\n{:5} {:5}\n",
                self.row1[0], self.row1[1], self.row2[0], self.row2[1]
            )
            .as_str(),
        )
    }
}

impl Display for Vec4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("[{} {} {} {}]", self.x, self.y, self.z, self.w).as_str())
    }
}
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("[{} {} {}]", self.x, self.y, self.z).as_str())
    }
}
impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("[{} {}]", self.x, self.y).as_str())
    }
}

impl Mul<Matrix2x2> for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Matrix2x2) -> Self::Output {
        Self {
            row1: [
                self.row(1).dot(&rhs.column(1)),
                self.row(1).dot(&rhs.column(2)),
            ],
            row2: [
                self.row(2).dot(&rhs.column(1)),
                self.row(2).dot(&rhs.column(2)),
            ],
        }
    }
}
impl Mul<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        Self {
            row1: [
                self.row(1).dot(&rhs.column(1)),
                self.row(1).dot(&rhs.column(2)),
                self.row(1).dot(&rhs.column(3)),
            ],
            row2: [
                self.row(2).dot(&rhs.column(1)),
                self.row(2).dot(&rhs.column(2)),
                self.row(2).dot(&rhs.column(3)),
            ],
            row3: [
                self.row(3).dot(&rhs.column(1)),
                self.row(3).dot(&rhs.column(2)),
                self.row(3).dot(&rhs.column(3)),
            ],
        }
    }
}
impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        Self {
            row1: [
                self.row(1).dot(&rhs.column(1)),
                self.row(1).dot(&rhs.column(2)),
                self.row(1).dot(&rhs.column(3)),
                self.row(1).dot(&rhs.column(4)),
            ],
            row2: [
                self.row(2).dot(&rhs.column(1)),
                self.row(2).dot(&rhs.column(2)),
                self.row(2).dot(&rhs.column(3)),
                self.row(2).dot(&rhs.column(4)),
            ],
            row3: [
                self.row(3).dot(&rhs.column(1)),
                self.row(3).dot(&rhs.column(2)),
                self.row(3).dot(&rhs.column(3)),
                self.row(3).dot(&rhs.column(4)),
            ],
            row4: [
                self.row(4).dot(&rhs.column(1)),
                self.row(4).dot(&rhs.column(2)),
                self.row(4).dot(&rhs.column(3)),
                self.row(4).dot(&rhs.column(4)),
            ],
        }
    }
}
impl Mul<Vec2> for Matrix2x2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.row(1).dot(&rhs), self.row(2).dot(&rhs))
    }
}
impl Mul<Vec3> for Matrix3x3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.row(1).dot(&rhs),
            self.row(2).dot(&rhs),
            self.row(3).dot(&rhs),
        )
    }
}
impl Mul<Vec4> for Matrix4x4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.row(1).dot(&rhs),
            self.row(2).dot(&rhs),
            self.row(3).dot(&rhs),
            self.row(4).dot(&rhs),
        )
    }
}
impl Mul<Vec3> for Matrix4x4 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.row(1).dot(&rhs.to_vec4()),
            y: self.row(2).dot(&rhs.to_vec4()),
            z: self.row(3).dot(&rhs.to_vec4()),
            w: self.row(4).dot(&rhs.to_vec4()),
        }
    }
}
impl Mul<f32> for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: f32) -> Self::Output {
        Matrix2x2 {
            row1: [self.row1[0] * rhs, self.row1[1] * rhs],
            row2: [self.row2[0] * rhs, self.row2[1] * rhs],
        }
    }
}
impl Mul<Matrix2x2> for f32 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Matrix2x2) -> Self::Output {
        rhs * self
    }
}

impl Mul<Matrix3x3> for f32 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        rhs * self
    }
}
impl Mul<Matrix4x4> for f32 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        rhs * self
    }
}
impl<T> Mul<T> for Matrix4x4
where
    T: AsPrimitive<f32>,
{
    type Output = Matrix4x4;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Matrix4x4 {
            row1: [
                self.row1[0] * rhs,
                self.row1[1] * rhs,
                self.row1[2] * rhs,
                self.row1[3] * rhs,
            ],
            row2: [
                self.row2[0] * rhs,
                self.row2[1] * rhs,
                self.row2[2] * rhs,
                self.row2[3] * rhs,
            ],
            row3: [
                self.row3[0] * rhs,
                self.row3[1] * rhs,
                self.row3[2] * rhs,
                self.row3[3] * rhs,
            ],
            row4: [
                self.row4[0] * rhs,
                self.row4[1] * rhs,
                self.row4[2] * rhs,
                self.row4[3] * rhs,
            ],
        }
    }
}
impl Mul<f32> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: f32) -> Self::Output {
        Matrix3x3 {
            row1: [self.row1[0] * rhs, self.row1[1] * rhs, self.row1[2] * rhs],
            row2: [self.row2[0] * rhs, self.row2[1] * rhs, self.row2[2] * rhs],
            row3: [self.row3[0] * rhs, self.row3[1] * rhs, self.row3[2] * rhs],
        }
    }
}

impl Sub for Matrix2x2 {
    type Output = Matrix2x2;

    fn sub(self, rhs: Self) -> Self::Output {
        Matrix2x2 {
            row1: [self.row1[0] - rhs.row1[0], self.row1[1] - rhs.row1[1]],
            row2: [self.row2[0] - rhs.row2[0], self.row2[1] - rhs.row2[1]],
        }
    }
}

impl Sub for Matrix3x3 {
    type Output = Matrix3x3;

    fn sub(self, rhs: Self) -> Self::Output {
        Matrix3x3 {
            row1: [
                self.row1[0] - rhs.row1[0],
                self.row1[1] - rhs.row1[1],
                self.row1[2] - rhs.row1[2],
            ],
            row2: [
                self.row2[0] - rhs.row2[0],
                self.row2[1] - rhs.row2[1],
                self.row2[2] - rhs.row2[2],
            ],
            row3: [
                self.row3[0] - rhs.row3[0],
                self.row3[1] - rhs.row3[1],
                self.row3[2] - rhs.row3[2],
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_points() {
        let p1 = Vec2::new(1, 2);
        let p2 = Vec2::new(3, 4);
        assert_eq!(p1 + p2, Vec2::new(4, 6));
    }
    #[test]
    fn test_subtract_points() {
        let p1 = Vec2::new(1, 2);
        let p2 = Vec2::new(3, 5);
        assert_eq!(p1 - p2, Vec2::new(-2, -3));
    }
    #[test]
    fn test_multiply_points() {
        let p1 = Vec2::new(2, 3);
        let p2 = Vec2::new(4, 5);
        assert_eq!(p1 * p2, Vec2::new(8, 15));
    }
    #[test]
    fn test_divide_points() {
        let p1 = Vec2::new(3, 1);
        let p2 = Vec2::new(6, 4);
        assert_eq!(p1 / p2, Vec2::new(0.5, 0.25));
    }
    #[test]
    fn test_add_vec2_f32() {
        let p1 = Vec2::new(1, 2);
        assert_eq!(p1 + 5.0, Vec2::new(6.0, 7.0));
    }
    #[test]
    fn test_subtract_vec2_f32() {
        let p1 = Vec2::new(1, 2);
        assert_eq!(p1 - 5.0, Vec2::new(-4.0, -3.0));
    }
    #[test]
    fn test_multiply_vec2_f32() {
        let p1 = Vec2::new(3, 4);
        assert_eq!(p1 * 5.0, Vec2::new(15, 20));
    }
    #[test]
    fn test_divide_vec2_f32() {
        let p1 = Vec2::new(3, 4);
        assert_eq!(p1 / 4.0, Vec2::new(0.75, 1.0));
    }
    #[test]
    fn test_negate_vec2() {
        let p1 = Vec2::new(5.0, 6.0);
        assert_eq!(-p1, Vec2::new(-5.0, -6.0));
    }
    #[test]
    fn test_add_assign_vec2() {
        let mut p1 = Vec2::new(1, 2);
        p1 += Vec2::new(3, 4);
        assert_eq!(p1, Vec2::new(4, 6));
    }
    #[test]
    fn test_sub_assign_vec2() {
        let mut p1 = Vec2::new(1, 2);
        p1 -= Vec2::new(5, 4);
        assert_eq!(p1, Vec2::new(-4, -2));
    }
    #[test]
    fn test_mul_assign_vec2() {
        let mut p1 = Vec2::new(2, 3);
        p1 *= Vec2::new(6, 5);
        assert_eq!(p1, Vec2::new(12, 15));
    }
    #[test]
    fn test_div_assign_vec2() {
        let mut p1 = Vec2::new(6, 8);
        p1 /= Vec2::new(2, 4);
        assert_eq!(p1, Vec2::new(3, 2));
    }
    #[test]
    fn test_dot_vec2() {
        let p1 = Vec2::new(2, 3);
        let p2 = Vec2::new(4, 5);
        let p3 = p1.dot(&p2);
        let p4 = p2.dot(&p1);
        assert_eq!(p3, 23.0);
        assert_eq!(p3, p4);
    }
    #[test]
    fn test_matrix2_rows_cols() {
        let mut matrix = Matrix2x2 {
            row1: [1.0, 2.0],
            row2: [3.0, 4.0],
        };
        assert_eq!(matrix.row(1), Vec2::new(1.0, 2.0));
        assert_eq!(matrix.row(2), Vec2::new(3.0, 4.0));
        assert_eq!(matrix.column(1), Vec2::new(1.0, 3.0));
        assert_eq!(matrix.column(2), Vec2::new(2.0, 4.0));
    }
    #[test]
    fn test_matrix3_mul_matrix3() {
        let ma = Matrix3x3 {
            row1: [1.0, 0.0, 0.0],
            row2: [-3.0, 1.0, 0.0],
            row3: [0.0, 0.0, 1.0],
        };
        let mb = Matrix3x3 {
            row1: [1.0, 2.0, 1.0],
            row2: [3.0, 8.0, 1.0],
            row3: [0.0, 4.0, 1.0],
        };
        let check = Matrix3x3 {
            row1: [1.0, 2.0, 1.0],
            row2: [0.0, 2.0, -2.0],
            row3: [0.0, 4.0, 1.0],
        };
        let result = ma * mb;
        assert_eq!(check, result);
        let matrix = Matrix3x3 {
            row1: [1.0, 2.0, 3.0],
            row2: [4.0, 5.0, 6.0],
            row3: [7.0, 8.0, 9.0],
        };
        let op = Matrix3x3 {
            row1: [1.0, 0.0, 0.0],
            row2: [0.0, 0.0, 1.0],
            row3: [0.0, 1.0, 0.0],
        };
        let result = op * matrix;
        assert_eq!(
            result,
            Matrix3x3 {
                row1: [1.0, 2.0, 3.0],
                row2: [7.0, 8.0, 9.0],
                row3: [4.0, 5.0, 6.0],
            }
        );
    }
    #[test]
    fn test_matrix2_mul_matrix2() {
        let ma = Matrix2x2 {
            row1: [1.0, 0.0],
            row2: [-3.0, 1.0],
        };
        let mb = Matrix2x2 {
            row1: [1.0, 2.0],
            row2: [3.0, 8.0],
        };
        let check = Matrix2x2 {
            row1: [1.0, 2.0],
            row2: [0.0, 2.0],
        };
        let result = ma * mb;
        assert_eq!(check, result);
    }
    #[test]
    fn test_matrix2x2_invertible() {
        let matrix = Matrix2x2 {
            row1: [1.0, 2.0],
            row2: [2.0, 4.0],
        };
        assert!(!matrix.is_invertible());
        let matrix2 = Matrix2x2 {
            row1: [2.0, 6.0],
            row2: [3.0, 10.0],
        };
        assert!(matrix2.is_invertible());
    }
    #[test]
    fn test_matrix3x3_invertible() {
        let matrix = Matrix3x3 {
            row1: [1.0, 2.0, 4.0],
            row2: [2.0, 4.0, 8.0],
            row3: [2.0, 4.0, 8.0],
        };
        assert!(matrix.inverse().is_none());
        let matrix2 = Matrix3x3 {
            row1: [1.0, 2.0, 3.0],
            row2: [2.0, 4.0, 3.0],
            row3: [2.0, 8.0, 2.0],
        };
        assert!(matrix2.inverse().is_some());
    }
    #[test]
    fn test_matrix2x2_determinant() {
        let matrix = Matrix2x2 {
            row1: [3.0, 7.0],
            row2: [1.0, -4.0],
        };
        assert_eq!(matrix.determinant(), -19.0);
    }
    #[test]
    fn test_matrix3x3_determinant() {
        let matrix = Matrix3x3 {
            row1: [-2.0, -1.0, 2.0],
            row2: [2.0, 1.0, 4.0],
            row3: [-3.0, 3.0, -1.0],
        };
        assert_eq!(matrix.determinant(), 54.0);
    }
    #[test]
    fn test_matrix4x4_determinant() {
        let matrix = Matrix4x4 {
            row1: [4.0, 3.0, 2.0, 2.0],
            row2: [0.0, 1.0, -3.0, 3.0],
            row3: [0.0, -1.0, 3.0, 3.0],
            row4: [0.0, 3.0, 1.0, 1.0],
        };
        assert_eq!(matrix.determinant(), -240.0);
    }
    #[test]
    fn test_matrix2x2_transpose() {
        let matrix = Matrix2x2 {
            row1: [1.0, 2.0],
            row2: [3.0, 4.0],
        };
        assert_eq!(
            matrix.transpose(),
            Matrix2x2 {
                row1: [1.0, 3.0],
                row2: [2.0, 4.0],
            }
        );
    }
    #[test]
    fn test_matrix3x3_transpose() {
        let matrix = Matrix3x3 {
            row1: [1.0, 2.0, 3.0],
            row2: [4.0, 5.0, 6.0],
            row3: [7.0, 8.0, 9.0],
        };
        assert_eq!(
            matrix.transpose(),
            Matrix3x3 {
                row1: [1.0, 4.0, 7.0],
                row2: [2.0, 5.0, 8.0],
                row3: [3.0, 6.0, 9.0],
            }
        );
    }
    #[test]
    fn test_matrix_add() {
        let matrix1 = Matrix3x3 {
            row1: [1.0, 2.0, 3.0],
            row2: [4.0, 5.0, 6.0],
            row3: [7.0, 8.0, 9.0],
        };
        let add = Matrix3x3 {
            row1: [1.0, 2.0, 3.0],
            row2: [4.0, 5.0, 6.0],
            row3: [7.0, 8.0, 9.0],
        };
        let result = matrix1 + add;
        assert_eq!(
            result,
            Matrix3x3 {
                row1: [2.0, 4.0, 6.0],
                row2: [8.0, 10.0, 12.0],
                row3: [14.0, 16.0, 18.0],
            }
        );
        let matrix2 = Matrix2x2 {
            row1: [1.0, 2.0],
            row2: [3.0, 4.0],
        };
        let add2 = Matrix2x2 {
            row1: [-2.0, -3.0],
            row2: [-5.0, -6.0],
        };
        let result2 = matrix2 + add2;
        assert_eq!(
            result2,
            Matrix2x2 {
                row1: [-1.0, -1.0],
                row2: [-2.0, -2.0]
            }
        );
    }
    #[test]
    fn test_matrix_inverse() {
        let m = Matrix2x2 {
            row1: [3.0, 5.0],
            row2: [-2.0, -4.0],
        };
        let m_i = m.inverse().unwrap();
        assert_eq!(
            m_i,
            Matrix2x2 {
                row1: [2.0, 5.0 / 2.0],
                row2: [-1.0, -3.0 / 2.0]
            }
        );
        let m3 = Matrix3x3 {
            row1: [1.0, 3.0, 1.0],
            row2: [0.0, 3.0, 1.0],
            row3: [4.0, 2.0, 0.0],
        };
        assert_eq!(
            m3.inverse().unwrap(),
            Matrix3x3 {
                row1: [1.0, -1.0, 0.0],
                row2: [-2.0, 2.0, 0.5],
                row3: [6.0, -5.0, -3.0 / 2.0]
            }
        );
    }
    #[test]
    fn test_matrix4x4_inverse() {
        let m = Matrix4x4 {
            row1: [4.0, 4.0, 2.0, 2.0],
            row2: [0.0, 1.0, -1.0, 3.0],
            row3: [0.0, -1.0, 2.0, 2.0],
            row4: [0.0, 2.0, 1.0, 1.0],
        };
        assert_eq!(
            m.inverse().unwrap(),
            Matrix4x4 {
                row1: [0.25, -0.0, -0.0, -0.5],
                row2: [0.0, -0.0, -0.2, 0.4],
                row3: [0.0, -0.25, 0.25, 0.25],
                row4: [0.0, 0.25, 0.15, -0.05],
            }
        )
    }
    #[test]
    fn test_matrix_minors() {
        let matrix = Matrix3x3 {
            row1: [1.0, 2.0, 3.0],
            row2: [4.0, 5.0, 6.0],
            row3: [7.0, 8.0, 9.0],
        };
        assert_eq!(
            matrix.minor(1, 1),
            Matrix2x2 {
                row1: [5.0, 6.0],
                row2: [8.0, 9.0]
            }
        );
        assert_eq!(
            matrix.minor(1, 2),
            Matrix2x2 {
                row1: [4.0, 6.0],
                row2: [7.0, 9.0]
            }
        );
        assert_eq!(
            matrix.minor(1, 3),
            Matrix2x2 {
                row1: [4.0, 5.0],
                row2: [7.0, 8.0]
            }
        );
        assert_eq!(
            matrix.minor(2, 1),
            Matrix2x2 {
                row1: [2.0, 3.0],
                row2: [8.0, 9.0]
            }
        );
        assert_eq!(
            matrix.minor(2, 2),
            Matrix2x2 {
                row1: [1.0, 3.0],
                row2: [7.0, 9.0]
            }
        );
        assert_eq!(
            matrix.minor(2, 3),
            Matrix2x2 {
                row1: [1.0, 2.0],
                row2: [7.0, 8.0]
            }
        );
        assert_eq!(
            matrix.minor(3, 1),
            Matrix2x2 {
                row1: [2.0, 3.0],
                row2: [5.0, 6.0]
            }
        );
        assert_eq!(
            matrix.minor(3, 2),
            Matrix2x2 {
                row1: [1.0, 3.0],
                row2: [4.0, 6.0]
            }
        );
        assert_eq!(
            matrix.minor(3, 3),
            Matrix2x2 {
                row1: [1.0, 2.0],
                row2: [4.0, 5.0]
            }
        );
    }
}
