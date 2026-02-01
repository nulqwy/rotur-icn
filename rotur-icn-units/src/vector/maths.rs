use std::ops;

use crate::number::Number;

use super::Vector;

impl Vector {
    pub fn new(v: Number) -> Self {
        Self { x: v, y: v }
    }

    pub fn new_from_length(length: Number, angle: Number) -> Self {
        Self::new_normal(angle) * length
    }

    pub fn new_normal(angle: Number) -> Self {
        let (y, x) = angle.sin_cos();
        Self { x, y }
    }

    pub fn length_sq(self) -> Number {
        self.dot(self)
    }

    pub fn length(self) -> Number {
        self.length_sq().sqrt()
    }

    pub fn angle(self) -> Number {
        self.y.atan2(self.x)
    }

    pub fn rotate_90_cc(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn rotate_90_cw(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn rotate(self, angle: Number) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    pub fn dot(self, other: Self) -> Number {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(self, other: Self) -> Number {
        self.x * other.y - self.y * other.x
    }

    pub fn powi(self, power: i32) -> Self {
        Self {
            x: self.x.powi(power),
            y: self.y.powi(power),
        }
    }

    pub fn powf(self, power: Number) -> Self {
        Self {
            x: self.x.powf(power),
            y: self.y.powf(power),
        }
    }

    pub fn clamp(self, min: Number, max: Number) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn normalise(self) -> Self {
        self / self.length()
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    // for conjugate
    pub fn conj_add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::MulAssign for Vector {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::Div for Vector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::DivAssign for Vector {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl ops::Add<Number> for Vector {
    type Output = Self;

    fn add(self, rhs: Number) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl ops::Add<Vector> for Number {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        rhs + self
    }
}

impl ops::AddAssign<Number> for Vector {
    fn add_assign(&mut self, rhs: Number) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl ops::Sub<Number> for Vector {
    type Output = Self;

    fn sub(self, rhs: Number) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl ops::Sub<Vector> for Number {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self - rhs.x,
            y: self - rhs.y,
        }
    }
}

impl ops::SubAssign<Number> for Vector {
    fn sub_assign(&mut self, rhs: Number) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl ops::Mul<Number> for Vector {
    type Output = Self;

    fn mul(self, rhs: Number) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<Vector> for Number {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<Number> for Vector {
    fn mul_assign(&mut self, rhs: Number) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Div<Number> for Vector {
    type Output = Self;

    fn div(self, rhs: Number) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::Div<Vector> for Number {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

impl ops::DivAssign<Number> for Vector {
    fn div_assign(&mut self, rhs: Number) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
