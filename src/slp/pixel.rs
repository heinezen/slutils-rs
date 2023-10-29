// Copyright 2023-2023 the slutils-rs authors.

use std::fmt;

/// Pixel type in an SLP frame.
pub enum SLPPixelType {
    /// 8-bit palette index
    PALETTE,
    /// Shadow color
    SHADOW,
    /// Transparency
    TRANSPARENT,
    /// non-outline Player color
    PLAYER,
    /// Player color outline color
    SPECIAL1,
    /// Black outline color
    SPECIAL2,

    /// Shadow color used in SLPv4
    SHADOWv4,
    /// non-outline Player color used in SLPv4
    PLAYERv4,
}

/// Pixel in an SLP frame using palette indices for colors.
pub struct PalettePixel {
    /// Pixel type
    pub pixel_type: SLPPixelType,
    /// Palette index
    pub index: u8,
}

impl PalettePixel {
    /// Create a new palette pixel.
    ///
    /// # Arguments
    ///
    /// * `pixel_type` - Pixel type
    /// * `index` - Palette index
    ///
    /// # Returns
    /// New palette pixel.
    pub fn new(pixel_type: SLPPixelType, index: u8) -> Self {
        Self { pixel_type, index }
    }
}

impl fmt::Display for PalettePixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.pixel_type {
            SLPPixelType::PALETTE => write!(f, "{:x}", self.index),
            SLPPixelType::SHADOW => write!(f, "SS"),
            SLPPixelType::TRANSPARENT => write!(f, "TT"),
            SLPPixelType::PLAYER => write!(f, "PP"),
            SLPPixelType::SPECIAL1 => write!(f, "LL"),
            SLPPixelType::SPECIAL2 => write!(f, "XX"),
            SLPPixelType::SHADOWv4 => write!(f, "SS"),
            SLPPixelType::PLAYERv4 => write!(f, "PP"),
        }
    }
}

/// Pixel in an SLP frame using RGBA colors.
pub struct RGBAPixel {
    /// Pixel type
    pub pixel_type: SLPPixelType,
    /// Red color component
    pub r: u8,
    /// Green color component
    pub g: u8,
    /// Blue color component
    pub b: u8,
    /// Alpha color component
    pub a: u8,
}

impl RGBAPixel {
    /// Create a new RGBA pixel.
    ///
    /// # Arguments
    ///
    /// * `pixel_type` - Pixel type
    /// * `r` - Red color component
    /// * `g` - Green color component
    /// * `b` - Blue color component
    /// * `a` - Alpha color component
    ///
    /// # Returns
    /// New RGBA pixel.
    pub fn new(pixel_type: SLPPixelType, r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            pixel_type,
            r,
            g,
            b,
            a,
        }
    }
}
