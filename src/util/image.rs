// Copyright 2023-2023 the slutils-rs authors.

use super::matrix::Matrix2D;

pub trait RGBAImageConvertible {
    /// Convert to a matrix of RGBA values.
    fn to_rgba_matrix(&self) -> Matrix2D<[u8; 4]>;

    /// Convert to a vector of bytes.
    fn to_rgba_bytes(&self) -> Vec<u8>;

    /// Convert to an RGBA image.
    fn to_image(&self) -> image::RgbaImage;
}
