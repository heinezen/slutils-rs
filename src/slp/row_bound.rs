// Copyright 2023-2023 the slutils-rs authors.

use super::unpack::UnpackFixedSize;

pub struct SLPRowBound {
    pub left: u16,
    pub right: u16,
    pub full_row: bool,
}

impl SLPRowBound {
    pub fn new(left: u16, right: u16, full_row: bool) -> Self {
        Self {
            left,
            right,
            full_row,
        }
    }
}

impl UnpackFixedSize for SLPRowBound {
    fn from_buffer(buffer: &[u8], offset: usize) -> Self {
        let left = u16::from_le_bytes([buffer[offset], buffer[offset + 1]]);
        let right = u16::from_le_bytes([buffer[offset + 2], buffer[offset + 3]]);

        if left == 0x8000 || right == 0x8000 {
            // row is completely transparent
            return SLPRowBound::new(0, 0, true);
        }

        return SLPRowBound::new(left, right, false);
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let left = u16::from_le_bytes([bytes[0], bytes[1]]);
        let right = u16::from_le_bytes([bytes[2], bytes[3]]);

        if left == 0x8000 || right == 0x8000 {
            // row is completely transparent
            return SLPRowBound::new(0, 0, true);
        }

        return SLPRowBound::new(left, right, false);
    }
}
