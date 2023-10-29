// Copyright 2023-2023 the slutils-rs authors.

use super::frame_info::SLPFrameInfo;
use super::row_bound::SLPRowBound;

/// Unpack a fixed size object in a file.
pub trait UnpackFixedSize {
    /// Create a new object from an offset inside a buffer.
    ///
    /// The buffer can be the whole file for example.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to read from.
    /// * `offset` - The offset inside the buffer to read from.
    ///
    /// # Returns
    ///
    /// New object created from the read data.
    fn from_buffer(buffer: &[u8], offset: usize) -> Self;

    /// Create a new object from a slice of bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The slice of bytes to read from.
    ///
    /// # Returns
    ///
    /// New object created from the read data.
    fn from_bytes(bytes: &[u8]) -> Self;
}

/// Unpack a frame data in a file.
pub trait UnpackFrameData<T> {
    /// Create a new frame from a buffer. The buffer should be the whole file.
    /// Start offsets are read from the frame info.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to read from.
    /// * `frame_info` - Frame metedata.
    ///
    /// # Returns
    ///
    /// New frame created from the read data.
    fn from_buffer(buffer: &[u8], frame_info: &SLPFrameInfo) -> Self;

    /// Decode the row bounds table.
    ///
    /// Ths will read exactly `frame_info.height` bounds from the buffer,
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to read from.
    /// * `frame_info` - Frame metedata.
    ///
    /// # Returns
    ///
    /// List of row bounds.
    fn decode_bounds_table(buffer: &[u8], frame_info: &SLPFrameInfo) -> Vec<SLPRowBound>;

    /// Decode the command offsets table.
    ///
    /// This will read exactly `frame_info.height` offsets from the buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to read from.
    /// * `frame_info` - Frame metedata.
    ///
    /// # Returns
    ///
    /// List of command offsets.
    fn decode_cmd_table(buffer: &[u8], frame_info: &SLPFrameInfo) -> Vec<u32>;

    /// Decode the draw commands in the frame data.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to read from.
    /// * `frame_info` - Frame metedata.
    ///
    /// # Returns
    ///
    /// Matrix of decoded pixels.
    fn decode_frame(
        buffer: &[u8],
        frame_info: &SLPFrameInfo,
        bounds_table: &Vec<SLPRowBound>,
        cmd_table: &Vec<u32>,
    ) -> Vec<Vec<T>>;

    /// Decode a single row in the frame.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to read from.
    /// * `bounds` - Row bounds.
    /// * `first_cmd_offset` - Offset of the row's first command in the buffer.
    /// * `expected_size` - Expected size of the row.
    ///
    /// # Returns
    ///
    /// Decoded pixels for the row.
    fn decode_row(
        buffer: &[u8],
        bounds: &SLPRowBound,
        first_cmd_offset: usize,
        expected_size: usize,
    ) -> Vec<T>;

    /// Decode the commands for a single row in the frame.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to read from.
    /// * `first_cmd_offset` - Offset of the row's first command in the buffer.
    /// * `expected_size` - Expected size of the row.
    ///
    /// # Returns
    ///
    /// Decoded pixels for the row commands.
    fn decode_row_cmds(buffer: &[u8], first_cmd_offset: usize, expected_size: usize) -> Vec<T>;
}
