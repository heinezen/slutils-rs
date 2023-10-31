// Copyright 2023-2023 the slutils-rs authors.

use crate::slp::unpack::UnpackFixedSize;

/// Bounds data for a row in an SLP frame.
pub struct SLPRowBoundData {
	/// Number of transparent pixels prepended on the left.
	pub left: u16,
	/// Number of transparent pixels appended on the right.
	pub right: u16,
}

impl SLPRowBoundData {
	/// Create a new SLP row bound.
	///
	/// # Arguments
	///
	/// * `left` - Number of transparent pixels prepended on the left.
	/// * `right` - Number of transparent pixels appended on the right.
	///
	/// # Returns
	///
	/// New SLP row bound.
	pub fn new(
		left: u16,
		right: u16,
	) -> Self {
		Self { left, right }
	}
}

/// Bounds for a row in an SLP frame.
pub struct SLPRowBound {
	/// Bounds data.
	pub data: SLPRowBoundData,
	/// true if the row is completely transparent, else false.
	pub full_row: bool,
}

impl SLPRowBound {
	/// Create a new SLP row bound.
	///
	/// # Arguments
	///
	/// * `left` - Number of transparent pixels prepended on the left.
	/// * `right` - Number of transparent pixels appended on the right.
	/// * `full_row` - true if the row is completely transparent, else false.
	///
	/// # Returns
	///
	/// New SLP row bound.
	pub fn new(
		left: u16,
		right: u16,
		full_row: bool,
	) -> Self {
		Self {
			data: SLPRowBoundData::new(left, right),
			full_row,
		}
	}

	/// Create a new SLP row bound from existing bounds data.
	///
	/// # Arguments
	///
	/// * `data` - Bounds data.
	/// * `full_row` - true if the row is completely transparent, else false.
	///
	/// # Returns
	///
	/// New SLP row bound.
	pub fn from_data(
		data: SLPRowBoundData,
		full_row: bool,
	) -> Self {
		Self { data, full_row }
	}

	pub const fn get_left(&self) -> u16 {
		return self.data.left;
	}

	pub const fn get_right(&self) -> u16 {
		return self.data.right;
	}
}

impl UnpackFixedSize for SLPRowBound {
	fn from_buffer(
		buffer: &[u8],
		offset: usize,
	) -> Self {
		let left = u16::from_le_bytes([buffer[offset], buffer[offset + 1]]);
		let right = u16::from_le_bytes([buffer[offset + 2], buffer[offset + 3]]);

		if left == 0x8000 || right == 0x8000 {
			// row is completely transparent
			return Self::new(0, 0, true);
		}

		Self::new(left, right, false)
	}

	fn from_bytes(bytes: &[u8]) -> Self {
		let left = u16::from_le_bytes([bytes[0], bytes[1]]);
		let right = u16::from_le_bytes([bytes[2], bytes[3]]);

		if left == 0x8000 || right == 0x8000 {
			// row is completely transparent
			return Self::new(0, 0, true);
		}

		return Self::new(left, right, false);
	}
}
