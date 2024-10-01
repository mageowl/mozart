#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color::from_hex_rgb(0x000000);
    pub const RED: Color = Color::from_hex_rgb(0xff0000);
    pub const GREEN: Color = Color::from_hex_rgb(0x00ff00);
    pub const BLUE: Color = Color::from_hex_rgb(0x0000ff);
    pub const WHITE: Color = Color::from_hex_rgb(0xffffff);
    pub const TRANSPARENT: Color = Color::from_hex_rgba(0);

    #[inline(always)]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// A hexadecimal color in the form of 0xRRGGBBAA.
    pub const fn from_hex_rgba(hex: u32) -> Self {
        let a = hex & 0xff;
        let mut color = Self::from_hex_rgb(hex >> 8);
        color.a = a as u8;
        color
    }

    pub const fn from_hex_rgb(hex: u32) -> Self {
        Self {
            r: (hex & 0xff0000 >> 16) as u8,
            g: (hex & 0x00ff00 >> 8) as u8,
            b: (hex & 0x0000ff) as u8,
            a: 255,
        }
    }
}

impl Into<Color> for [u8; 4] {
    fn into(self) -> Color {
        Color {
            r: self[0],
            g: self[1],
            b: self[2],
            a: self[3],
        }
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Into<Color> for [f32; 4] {
    fn into(self) -> Color {
        Color {
            r: (self[0] * 255.).round().abs() as u8,
            g: (self[0] * 255.).round().abs() as u8,
            b: (self[0] * 255.).round().abs() as u8,
            a: (self[0] * 255.).round().abs() as u8,
        }
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
            self.a as f32 / 255.,
        ]
    }
}

impl Into<(f32, f32, f32, f32)> for Color {
    fn into(self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
            self.a as f32 / 255.,
        )
    }
}

impl Into<Color> for (f32, f32, f32, f32) {
    fn into(self) -> Color {
        Color {
            r: (self.0 * 255.).round().abs() as u8,
            g: (self.0 * 255.).round().abs() as u8,
            b: (self.0 * 255.).round().abs() as u8,
            a: (self.0 * 255.).round().abs() as u8,
        }
    }
}
