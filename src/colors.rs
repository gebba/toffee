
#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn RGB(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: 255,
        }
    }

    pub fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

// Color constant definitions

#[allow(dead_code)]
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

#[allow(dead_code)]
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

#[allow(dead_code)]
pub const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

#[allow(dead_code)]
pub const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};

#[allow(dead_code)]
pub const BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};

#[allow(dead_code)]
pub const YELLOW: Color = Color {
    r: 255,
    g: 255,
    b: 0,
    a: 255,
};
