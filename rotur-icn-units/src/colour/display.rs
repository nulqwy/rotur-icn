use std::fmt;

use super::Colour;

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b)
    }
}
