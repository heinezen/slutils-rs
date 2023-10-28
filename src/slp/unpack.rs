// Copyright 2023-2023 the slutils-rs authors.

use super::frame_info::SLPFrameInfoData;
use super::row_bound::SLPRowBound;

pub trait UnpackFixedSize {
    fn from_buffer(buffer: &[u8], offset: usize) -> Self;
    fn from_bytes(bytes: &[u8]) -> Self;
}

pub trait UnpackFrameData<T> {
    fn from_buffer(buffer: &[u8], frame_info: &SLPFrameInfoData) -> Self;

    fn decode_outline_table(buffer: &[u8], frame_info: &SLPFrameInfoData) -> Vec<SLPRowBound>;

    fn decode_cmd_table(buffer: &[u8], frame_info: &SLPFrameInfoData) -> Vec<u32>;

    fn decode_frame(
        buffer: &[u8],
        frame_info: &SLPFrameInfoData,
        outline_table: &Vec<SLPRowBound>,
        cmd_table: &Vec<u32>,
    ) -> Vec<Vec<T>>;

    fn decode_row(
        buffer: &[u8],
        outline: &SLPRowBound,
        first_cmd_offset: usize,
        expected_size: usize,
    ) -> Vec<T>;

    fn decode_row_cmds(buffer: &[u8], first_cmd_offset: usize, expected_size: usize) -> Vec<T>;
}
