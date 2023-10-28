// Copyright 2023-2023 the slutils-rs authors.

use super::{frame_data::SLPFrameBound, frame_info::SLPFrameInfo};

pub trait UnpackFixedSize {
    fn from_buffer(buffer: &[u8], offset: usize) -> Self;
    fn from_bytes(bytes: &[u8]) -> Self;
}

pub trait UnpackFrameData<T> {
    fn from_buffer(buffer: &[u8], frame_info: &SLPFrameInfo) -> Self;

    fn decode_outline_table(buffer: &[u8], frame_info: &SLPFrameInfo) -> Vec<SLPFrameBound>;

    fn decode_cmd_table(buffer: &[u8], frame_info: &SLPFrameInfo) -> Vec<u32>;

    fn decode_frame(
        buffer: &[u8],
        frame_info: &SLPFrameInfo,
        outline_table: &Vec<SLPFrameBound>,
        cmd_table: &Vec<u32>,
    ) -> Vec<Vec<T>>;

    fn decode_row(
        buffer: &[u8],
        outline: &SLPFrameBound,
        first_cmd_offset: usize,
        expected_size: usize,
    ) -> Vec<T>;

    fn decode_row_cmds(buffer: &[u8], first_cmd_offset: usize, expected_size: usize) -> Vec<T>;
}
