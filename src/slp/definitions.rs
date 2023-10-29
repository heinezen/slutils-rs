// Copyright 2023-2023 the slutils-rs authors.

/// Size of the SLP version field (in bytes).
pub const SLP_VERSION_SIZE: usize = 4;

/// Size of the SLP header struct (in bytes).
pub const SLP_HEADER_SIZE: usize = 32;

/// Size of the SLP frame info struct (in bytes).
pub const SLP_FRAME_INFO_SIZE: usize = 32;

/// Size of a SLP frame bound struct (in bytes).
pub const SLP_FRAME_BOUNDS_SIZE: usize = 4;

/// Size of an SLP frame command offset field (in bytes).
pub const SLP_FRAME_CMD_OFFSET_SIZE: usize = 4;
