//! Provides a 3-dimensional vector object, and associated functions.
//!
//! Includes functions for things like polar coordinates, dot products,
//! cross products, affine transformations, etc.

//---------------------------------------------------------------------------------------------------//
// The Vec3 type.

/// A 3d euclidean vector
#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

//---------------------------------------------------------------------------------------------------//
// Associated functions and methods.

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

    /// Returns a zero Vec3
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
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

    /// Perform an affine transformation on the Vec2.
    ///
    /// Uses a [[f64; 3]; 3] as the matrix, and another Vec3 as the translation.
    ///
    /// y = Ax + b
    ///
    /// A = [ [f64, f64, f64],
    ///       [f64, f64, f64],
    ///       [f64, f64, f64] ]
    ///
    /// b = Vec3
    pub fn affine_transformation(self, a: [[f64; 3]; 3], b: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * a[0][0] + self.y * a[0][1] + self.z * a[0][2] + b.x,
            y: self.x * a[1][0] + self.y * a[1][1] + self.z * a[1][2] + b.y,
            z: self.x * a[2][0] + self.z * a[2][1] + self.z * a[2][2] + b.z,
        }
    }
}

//---------------------------------------------------------------------------------------------------//
// Operator overloading.

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
