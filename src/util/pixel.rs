// Copyright 2023-2023 the slutils-rs authors.

use std::collections::HashMap;

/// Convert a pixel to a RGBA value.
pub trait RGBAConvertible {
	/// Convert a pixel to a RGBA value.
	fn to_rgba(
		&self,
		lookup: HashMap<usize, [u8; 4]>,
	) -> [u8; 4];
}
