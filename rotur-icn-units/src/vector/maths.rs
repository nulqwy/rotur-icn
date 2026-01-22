use std::ops;

use crate::number::Number;

use super::Vector;

impl Vector {
    pub fn length_sq(self) -> Number {
        self.x * self.x + self.y * self.y
    }

    pub fn length(self) -> Number {
        self.length_sq().sqrt()
    }

    pub fn dot_product(self, other: Self) -> Number {
        self.x * other.x + self.y * other.y
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
