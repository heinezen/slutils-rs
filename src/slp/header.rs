// Copyright 2023-2023 the slutils-rs authors.

use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt;
use std::{io::Cursor, string::String};

use super::types::SLPVersion;
use super::unpack::UnpackFixedSize;

/// Header data in an SLP file.
pub struct SLPHeaderData {
    /// Version string.
    pub version: SLPVersion,
    /// Number of frames.
    pub num_frames: u32,
    /// Comment string.
    pub comment: [u8; 24],
}

impl SLPHeaderData {
    /// Create a new SLP header.
    ///
    /// # Arguments
    ///
    /// * `version` - Version string.
    /// * `num_frames` - Number of frames.
    /// * `comment` - Comment string.
    ///
    /// # Returns
    ///
    /// New SLP header.
    pub fn new(version: SLPVersion, num_frames: u32, comment: [u8; 24]) -> Self {
        Self {
            version,
            num_frames,
            comment,
        }
    }
}

impl UnpackFixedSize for SLPHeaderData {
    fn from_buffer(buffer: &[u8], offset: usize) -> Self {
        let version: SLPVersion = buffer[offset..offset + 4].try_into().unwrap();

        let mut byte_reader = Cursor::new(&buffer[offset + 4..offset + 8]);
        let num_frames: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let comment: [u8; 24] = buffer[offset + 8..offset + 32].try_into().unwrap();

        return SLPHeaderData::new(version, num_frames, comment);
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let version = bytes[0..4].try_into().unwrap();

        let mut byte_reader = Cursor::new(&bytes[4..8]);
        let num_frames: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let comment = bytes[8..32].try_into().unwrap();

        return SLPHeaderData::new(version, num_frames, comment);
    }
}

/// Header in an SLP file.
pub struct SLPHeader {
    /// Header data.
    pub data: SLPHeaderData,
}

impl SLPHeader {
    /// Create a new SLP header.
    ///
    /// # Arguments
    ///
    /// * `version` - Version string.
    /// * `num_frames` - Number of frames.
    /// * `comment` - Comment string.
    ///
    /// # Returns
    ///
    /// New SLP header.
    pub fn new(version: SLPVersion, num_frames: u32, comment: [u8; 24]) -> Self {
        Self {
            data: SLPHeaderData::new(version, num_frames, comment),
        }
    }

    /// Create a new SLP header from existing header data.
    ///
    /// # Arguments
    ///
    /// * `data` - Header data.
    ///
    /// # Returns
    ///
    /// New SLP header.
    pub fn from_data(data: SLPHeaderData) -> Self {
        Self { data }
    }

    /// Get the version string.
    ///
    /// # Returns
    ///
    /// Version string.
    pub fn get_version(&self) -> String {
        return String::from_utf8(self.data.version.to_vec()).unwrap();
    }

    /// Get the number of frames.
    ///
    /// # Returns
    ///
    /// Number of frames.
    pub fn get_num_frames(&self) -> u32 {
        return self.data.num_frames;
    }

    /// Get the comment string.
    ///
    /// # Returns
    ///
    /// Comment string.
    pub fn get_comment(&self) -> String {
        return String::from_utf8(self.data.comment.to_vec()).unwrap();
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

pub struct SLP4HeaderData {
    version: SLPVersion,
    num_frames: u16,
    frame_type: u16,
    num_directions: u16,
    frames_per_direction: u16,
    palette_id: u32,
    offset_main: u32,
    offset_secondary: u32,
    pad: [u8; 8],
}

impl SLP4HeaderData {
    pub fn new(
        version: SLPVersion,
        num_frames: u16,
        frame_type: u16,
        num_directions: u16,
        frames_per_direction: u16,
        palette_id: u32,
        offset_main: u32,
        offset_secondary: u32,
        pad: [u8; 8],
    ) -> SLP4HeaderData {
        SLP4HeaderData {
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
