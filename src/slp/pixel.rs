// Copyright 2023-2023 the slutils-rs authors.

pub enum PixelType {
    /// 8-bit palette index
    PALETTE,
    /// Shadow color
    SHADOW,
    /// Transparency
    TRANSPARENT,
    /// non-outline Player color
    PLAYER,
    /// Black color (unused?)
    BLACK,
    /// Player color outline color
    SPECIAL1,
    /// Black outline color
    SPECIAL2,

    /// Shadow color used in SLPv4
    SHADOWv4,
    /// non-outline Player color used in SLPv4
    PLAYERv4,
}

pub struct PalettePixel {
    pub pixel_type: PixelType,
    pub index: u8,
}

impl PalettePixel {
    pub fn new(pixel_type: PixelType, index: u8) -> Self {
        Self { pixel_type, index }
    }
}

pub struct RGBAPixel {
    pub pixel_type: PixelType,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBAPixel {
    pub fn new(pixel_type: PixelType, r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            pixel_type,
            r,
            g,
            b,
            a,
        }
    }
}
