// Copyright 2023-2023 the slutils-rs authors.

use std::{collections::HashMap, fmt};

use crate::util::pixel::RGBAConvertible;

/// Pixel type in an SLP frame.
pub enum SLPPixelType {
	/// 8-bit palette index
	Palette,
	/// Shadow color
	Shadow,
	/// Transparency
	Transparent,
	/// non-outline Player color
	Player,
	/// Player color outline color
	Special1,
	/// Black outline color
	Special2,

	/// Shadow color used in SLPv4
	ShadowV4,
	/// non-outline Player color used in SLPv4
	PlayerV4,
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
	pub fn new(
		pixel_type: SLPPixelType,
		index: u8,
	) -> Self {
		Self { pixel_type, index }
	}
}

impl RGBAConvertible for PalettePixel {
	fn to_rgba(
		&self,
		_lookup: HashMap<usize, [u8; 4]>,
	) -> [u8; 4] {
		match self.pixel_type {
			SLPPixelType::Palette => [self.index, self.index, self.index, 255],
			SLPPixelType::Transparent => [0, 0, 0, 0],
			SLPPixelType::Shadow | SLPPixelType::ShadowV4 => [0, 0, 0, 100],
			SLPPixelType::Player | SLPPixelType::PlayerV4 => [0, self.index, 0, 254],
			SLPPixelType::Special1 => [0, 0, 0, 252],
			SLPPixelType::Special2 => [0, 0, 0, 250],
		}
	}
}

impl fmt::Display for PalettePixel {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		match self.pixel_type {
			SLPPixelType::Palette => write!(f, "{:x}", self.index),
			SLPPixelType::Shadow | SLPPixelType::ShadowV4 => write!(f, "SS"),
			SLPPixelType::Transparent => write!(f, "TT"),
			SLPPixelType::Player | SLPPixelType::PlayerV4 => write!(f, "PP"),
			SLPPixelType::Special1 => write!(f, "LL"),
			SLPPixelType::Special2 => write!(f, "XX"),
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
	pub fn new(
		pixel_type: SLPPixelType,
		r: u8,
		g: u8,
		b: u8,
		a: u8,
	) -> Self {
		Self {
			pixel_type,
			r,
			g,
			b,
			a,
		}
	}
}
