// Copyright 2023-2023 the slutils-rs authors.

use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt;
use std::{io::Cursor, string::String};

use crate::slp::unpack::UnpackFixedSize;

pub struct SLPHeader {
    pub version: [u8; 4],
    pub num_frames: u32,
    pub comment: [u8; 24],
}

impl SLPHeader {
    pub fn new(version: [u8; 4], num_frames: u32, comment: [u8; 24]) -> SLPHeader {
        SLPHeader {
            version,
            num_frames,
            comment,
        }
    }

    pub fn get_version(&self) -> String {
        return String::from_utf8(self.version.to_vec()).unwrap();
    }

    pub fn get_num_frames(&self) -> u32 {
        return self.num_frames;
    }

    pub fn get_comment(&self) -> String {
        return String::from_utf8(self.comment.to_vec()).unwrap();
    }
}

impl UnpackFixedSize for SLPHeader {
    fn from_buffer(buffer: &[u8], offset: usize) -> Self {
        let version: [u8; 4] = buffer[offset..offset + 4].try_into().unwrap();

        let mut byte_reader = Cursor::new(&buffer[offset + 4..offset + 8]);
        let num_frames: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let comment: [u8; 24] = buffer[offset + 8..offset + 32].try_into().unwrap();

        return SLPHeader::new(version, num_frames, comment);
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let version = bytes[0..4].try_into().unwrap();

        let mut byte_reader = Cursor::new(&bytes[4..8]);
        let num_frames: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let comment = bytes[8..32].try_into().unwrap();

        return SLPHeader::new(version, num_frames, comment);
    }
}

impl fmt::Display for SLPHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "version: {}\nnum_frames: {}\ncomment: {}",
            self.get_version(),
            self.get_num_frames(),
            self.get_comment()
        )
    }
}

pub struct SLP4Header {
    version: [u8; 4],
    num_frames: u16,
    frame_type: u16,
    num_directions: u16,
    frames_per_direction: u16,
    palette_id: u32,
    offset_main: u32,
    offset_secondary: u32,
    pad: [u8; 8],
}

impl SLP4Header {
    pub fn new(
        version: [u8; 4],
        num_frames: u16,
        frame_type: u16,
        num_directions: u16,
        frames_per_direction: u16,
        palette_id: u32,
        offset_main: u32,
        offset_secondary: u32,
        pad: [u8; 8],
    ) -> SLP4Header {
        SLP4Header {
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
