#[derive(Copy, Clone)]
pub enum Color {
    // Reds
    Salmon = 0xFA8072,
    Crimson = 0xDC143C,
    Red = 0xFF0000,
    DarkRed = 0x8B0000,

    // Pinks
    Pink = 0xFFC0CB,
    DeepPink = 0xFF1493,

    // Oranges
    Coral = 0xFF7F50,
    DarkOrange = 0xFF8C00,
    Orange = 0xFFA500,

    // Yellows
    Gold = 0xFFD700,
    Yellow = 0xFFFF00,
    LightYellow = 0xFFFFE0,

    // Purples
    Lavender = 0xE6E6FA,
    Plum = 0xDDA0DD,
    Violet = 0xEE82EE,
    Magenta = 0xFF00FF,
    DarkViolet = 0x9400D3,
    Purple = 0x800080,
    Indigo = 0x4B0082,

    // Greens
    Lime = 0x00FF00,
    LimeGreen = 0x32CD32,
    SeaGreen = 0x2E8B57,
    Green = 0x008000,
    DarkGreen = 0x006400,
    Olive = 0x808000,
    Teal = 0x008080,

    // Blues
    Cyan = 0x00FFFF,
    LightCyan = 0xE0FFFF,
    Turquoise = 0x40E0D0,
    SteelBlue = 0x4682B4,
    LightBlue = 0xADD8E6,
    SkyBlue = 0x87CEEB,
    DeepSkyBlue = 0x00BFFF,
    DodgerBlue = 0x1E90FF,
    CornflowerBlue = 0x6495ED,
    RoyalBlue = 0x4169E1,
    Blue = 0x0000FF,
    DarkBlue = 0x00008B,
    Navy = 0x000080,

    // Browns
    Cornsilk = 0xFFF8DC,
    Wheat = 0xF5DEB3,
    Tan	= 0xD2B48C,
    Goldenrod = 0xDAA520,
    SaddleBrown = 0x8B4513,
    Sienna = 0xA0522D,
    Brown = 0xA52A2A,
    Maroon = 0x800000,

    // Whites/Grays
    White = 0xFFFFFF,
    GhostWhite = 0xF8F8FF,
    WhiteSmoke = 0xF5F5F5,
    Ivory = 0xFFFFF0,
    LightGray = 0xD3D3D3,
    Gray = 0x808080,
    SlateGray = 0x708090,
    Black = 0x000000
}

impl Color {
    pub fn r(c: usize) -> usize {
        (c >> 16) & 0xFF
    }

    pub fn g(c: usize) -> usize {
        (c >> 8) & 0xFF
    }

    pub fn b(c: usize) -> usize {
        c & 0xFF
    }

    pub fn scale(c: usize, factor: f64) -> usize {
        let r = (Color::r(c) as f64 * factor).clamp(0.0, 255.0) as usize;
        let g = (Color::g(c) as f64 * factor).clamp(0.0, 255.0) as usize;
        let b = (Color::b(c) as f64 * factor).clamp(0.0, 255.0) as usize;
    
        (r << 16) | (g << 8) | b
    }

    pub fn add(a: usize, b: usize) -> usize {
        let ra = Color::r(a) as f64;
        let ga = Color::g(a) as f64;
        let ba = Color::b(a) as f64;
    
        let rb = Color::r(b) as f64;
        let gb = Color::g(b) as f64;
        let bb = Color::b(b) as f64;
        
        let r = (ra + rb).clamp(0.0, 255.0) as usize;
        let g = (ga + gb).clamp(0.0, 255.0) as usize;
        let b = (ba + bb).clamp(0.0, 255.0) as usize;
    
        (r << 16) | (g << 8) | b
    }
}