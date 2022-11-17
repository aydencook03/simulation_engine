//! Provides a 3-dimensional vector object, a 3x3 matrix, associated functions, and other useful things.
//!
//! Includes functions for things like polar coordinates, dot products,
//! cross products, affine transformations, etc.

//---------------------------------------------------------------------------------------------------//
// The Vec3 type.

pub const PI: f64 = core::f64::consts::PI;

/// Another name for a Vec3 for semantic purposes
pub type Point3 = Vec3;

/// A 3d euclidean vector
#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A 3x3 matrix
#[derive(Copy, Clone)]
pub struct Matrix3(pub [[f64; 3]; 3]);

//---------------------------------------------------------------------------------------------------//
// Associated functions and methods of Vec3.

impl Vec3 {
    /// Create a Vec3 using x and y components
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Create a Vec3 in the x-y plane using polar components
    pub fn new_polar(r: f64, angle: f64) -> Vec3 {
        Vec3 {
            x: r * angle.cos(),
            y: r * angle.sin(),
            z: 0.0,
        }
    }

    /// Create a Vec3 using spherical coordinates (radius, azimuth, polar)
    pub fn new_spherical(r: f64, theta: f64, phi: f64) -> Vec3 {
        Vec3 {
            x: r * phi.sin() * theta.cos(),
            y: r * phi.sin() * theta.sin(),
            z: r * phi.cos(),
        }
    }

    /// Returns a zero Vec3
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Returns the x axis unit vector
    pub fn x_hat() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Returns the y axis unit vector
    pub fn y_hat() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    /// Returns the z axis unit vector
    pub fn z_hat() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    /// Dot product with another Vec3
    pub fn dot(self, other: Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    /// Cross product with another Vec3
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Returns the square of the magnitude of the Vec3
    pub fn mag_squared(self) -> f64 {
        self.dot(self)
    }

    /// Returns the magnitude of the Vec3
    pub fn mag(self) -> f64 {
        self.mag_squared().sqrt()
    }

    /// Return the normalized version of the Vec3
    pub fn norm(self) -> Vec3 {
        self / self.mag()
    }
}

//---------------------------------------------------------------------------------------------------//
// Associated functions and methods of Matrix3.

impl Matrix3 {
    pub fn identity() -> Matrix3 {
        Matrix3::default()
    }

    pub fn cross_product_matrix(vector: Vec3) -> Matrix3 {
        Matrix3([
            [0.0, -vector.z, vector.y],
            [vector.z, 0.0, -vector.x],
            [-vector.y, vector.x, 0.0],
        ])
    }

    pub fn rotation_axis_angle(axis: Vec3, angle: f64) -> Matrix3 {
        let cross = Matrix3::cross_product_matrix(axis);

        Matrix3::identity() + angle.sin() * cross + (1.0 - angle.cos()) * cross * cross
    }
}

//---------------------------------------------------------------------------------------------------//
// Operator overloading on Vec3.

impl core::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl core::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl core::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl core::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl core::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl core::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl core::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl core::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl core::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl core::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

//---------------------------------------------------------------------------------------------------//
// Associated functions and methods of Matrix3.

//---------------------------------------------------------------------------------------------------//
// Operator overloading on Matrix3.

impl Default for Matrix3 {
    fn default() -> Self {
        Self([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    }
}

impl core::ops::Mul<Vec3> for Matrix3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self.0[0][0] + rhs.y * self.0[0][1] + rhs.z * self.0[0][2],
            y: rhs.x * self.0[1][0] + rhs.y * self.0[1][1] + rhs.z * self.0[1][2],
            z: rhs.x * self.0[2][0] + rhs.y * self.0[2][1] + rhs.z * self.0[2][2],
        }
    }
}

impl core::ops::Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: Matrix3) -> Self::Output {
        Matrix3([
            [
                self.0[0][0] * rhs.0[0][0]
                    + self.0[0][1] * rhs.0[1][0]
                    + self.0[0][2] * rhs.0[2][0],
                self.0[0][0] * rhs.0[0][1]
                    + self.0[0][1] * rhs.0[1][1]
                    + self.0[0][2] * rhs.0[2][1],
                self.0[0][0] * rhs.0[0][2]
                    + self.0[0][1] * rhs.0[1][2]
                    + self.0[0][2] * rhs.0[2][2],
            ],
            [
                self.0[1][0] * rhs.0[0][0]
                    + self.0[1][1] * rhs.0[1][0]
                    + self.0[1][2] * rhs.0[2][0],
                self.0[1][0] * rhs.0[0][1]
                    + self.0[1][1] * rhs.0[1][1]
                    + self.0[1][2] * rhs.0[2][1],
                self.0[1][0] * rhs.0[0][2]
                    + self.0[1][1] * rhs.0[1][2]
                    + self.0[1][2] * rhs.0[2][2],
            ],
            [
                self.0[2][0] * rhs.0[0][0]
                    + self.0[2][1] * rhs.0[1][0]
                    + self.0[2][2] * rhs.0[2][0],
                self.0[2][0] * rhs.0[0][1]
                    + self.0[2][1] * rhs.0[1][1]
                    + self.0[2][2] * rhs.0[2][1],
                self.0[2][0] * rhs.0[0][2]
                    + self.0[2][1] * rhs.0[1][2]
                    + self.0[2][2] * rhs.0[2][2],
            ],
        ])
    }
}

impl core::ops::Mul<Matrix3> for f64 {
    type Output = Matrix3;
    fn mul(self, rhs: Matrix3) -> Self::Output {
        Matrix3([
            [self * rhs.0[0][0], self * rhs.0[0][1], self * rhs.0[0][2]],
            [self * rhs.0[1][0], self * rhs.0[1][1], self * rhs.0[1][2]],
            [self * rhs.0[2][0], self * rhs.0[2][1], self * rhs.0[2][2]],
        ])
    }
}

impl core::ops::Mul<f64> for Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix3([
            [self.0[0][0] * rhs, self.0[0][1] * rhs, self.0[0][2] * rhs],
            [self.0[1][0] * rhs, self.0[1][1] * rhs, self.0[1][2] * rhs],
            [self.0[2][0] * rhs, self.0[2][1] * rhs, self.0[2][2] * rhs],
        ])
    }
}

impl core::ops::Add<Matrix3> for Matrix3 {
    type Output = Matrix3;
    fn add(self, rhs: Matrix3) -> Self::Output {
        Matrix3([
            [
                self.0[0][0] + rhs.0[0][0],
                self.0[0][1] + rhs.0[0][1],
                self.0[0][2] + rhs.0[0][2],
            ],
            [
                self.0[1][0] + rhs.0[1][0],
                self.0[1][1] + rhs.0[1][1],
                self.0[1][2] + rhs.0[1][2],
            ],
            [
                self.0[2][0] + rhs.0[2][0],
                self.0[2][1] + rhs.0[2][1],
                self.0[2][2] + rhs.0[2][2],
            ],
        ])
    }
}
