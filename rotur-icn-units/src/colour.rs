mod display;

// reversed order for a noop u32 -> Colour
#[cfg(target_endian = "little")]
#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub a: u8,
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

#[cfg(target_endian = "big")]
#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Colour {
    fn default() -> Self {
        Self {
            r: 0xff,
            g: 0x00,
            b: 0xff,
            a: 0xff,
        }
    }
}

impl Colour {
    pub const ZERO: Self = Self {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0x00,
    };

    pub const BLACK: Self = Self {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xff,
    };

    pub const WHITE: Self = Self {
        r: 0xff,
        g: 0xff,
        b: 0xff,
        a: 0xff,
    };

    pub fn from_u32_with_alpha(value: u32) -> Self {
        #[cfg(target_endian = "little")]
        let [a, b, g, r] = value.to_le_bytes();

        #[cfg(target_endian = "big")]
        let [r, g, b, a] = value.to_be_bytes();

        Self { r, g, b, a }
    }
}

impl TryFrom<u32> for Colour {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        #[cfg(target_endian = "little")]
        let [b, g, r, overflow] = value.to_le_bytes();

        #[cfg(target_endian = "big")]
        let [overflow, r, g, b] = value.to_be_bytes();

        if overflow != 0 {
            return Err(());
        }

        Ok(Self { r, g, b, a: 0xff })
    }
}

impl From<Colour> for u32 {
    fn from(value: Colour) -> Self {
        #[cfg(target_endian = "little")]
        let n = u32::from_le_bytes([value.b, value.g, value.r, 0x00]);

        #[cfg(target_endian = "big")]
        let n = u32::from_be_bytes([0x00, value.r, value.g, value.b]);

        n
    }
}
