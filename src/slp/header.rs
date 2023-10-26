// Copyright 2023-2023 the slutils-rs authors.

use byteorder::{LittleEndian, ReadBytesExt};
use std::{io::Cursor, string::String};

pub struct SLPHeader {
    pub version: String,
    pub num_frames: u32,
    pub comment: String,
}

pub trait Unpack {
    fn from_buffer(buffer: &[u8], offset: usize) -> Self;
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl SLPHeader {
    pub fn new(version: String, num_frames: u32, comment: String) -> SLPHeader {
        SLPHeader {
            version,
            num_frames,
            comment,
        }
    }
}

impl Unpack for SLPHeader {
    fn from_buffer(buffer: &[u8], offset: usize) -> Self {
        let version: String = String::from_utf8(buffer[offset..offset + 4].to_vec()).unwrap();

        let mut byte_reader = Cursor::new(&buffer[offset + 4..offset + 8]);
        let num_frames: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let comment = String::from_utf8(buffer[offset + 8..offset + 32].to_vec()).unwrap();

        return SLPHeader::new(version, num_frames, comment);
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let version: String = String::from_utf8(bytes[0..4].to_vec()).unwrap();

        let mut byte_reader = Cursor::new(&bytes[4..8]);
        let num_frames: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let comment = String::from_utf8(bytes[8..32].to_vec()).unwrap();

        return SLPHeader::new(version, num_frames, comment);
    }
}

pub struct SLPHeader4 {
    version: String,
    num_frames: u16,
    frame_type: u16,
    num_directions: u16,
    frames_per_direction: u16,
    palette_id: u32,
    offset_main: u32,
    offset_secondary: u32,
    pad: [u8; 8],
}

impl SLPHeader4 {
    pub fn new(
        version: String,
        num_frames: u16,
        frame_type: u16,
        num_directions: u16,
        frames_per_direction: u16,
        palette_id: u32,
        offset_main: u32,
        offset_secondary: u32,
        pad: [u8; 8],
    ) -> SLPHeader4 {
        SLPHeader4 {
            version,
            num_frames,
            frame_type,
            num_directions,
            frames_per_direction,
            palette_id,
            offset_main,
            offset_secondary,
            pad,
        }
    }
}
