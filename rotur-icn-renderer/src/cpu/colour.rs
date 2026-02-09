use rotur_icn_units::Colour;

#[derive(Debug, Clone, Copy)]
pub struct InternalColour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl InternalColour {
    pub fn to_bytes(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl From<Colour> for InternalColour {
    fn from(Colour { r, g, b, a }: Colour) -> Self {
        Self { r, g, b, a }
    }
}
