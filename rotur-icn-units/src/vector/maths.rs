use std::ops;

use crate::number::Number;

use super::Vector;

impl Vector {
    pub fn new_from_length(length: Number, angle: Number) -> Self {
        Self::new_normal(angle) * length
    }

    pub fn new_normal(angle: Number) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
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

    pub fn clamp(self, min: Number, max: Number) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
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

impl ops::Add<Number> for Vector {
    type Output = Self;

    fn add(self, rhs: Number) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
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

impl ops::DivAssign<Number> for Vector {
    fn div_assign(&mut self, rhs: Number) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
