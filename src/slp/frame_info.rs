// Copyright 2023-2023 the slutils-rs authors.

use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt;
use std::io::Cursor;

use crate::slp::unpack::UnpackFixedSize;

use super::types::SLPVersion;

/// SLP frame type.
pub enum SLPFrameType {
    MAIN,
    SHADOW,
}

impl fmt::Display for SLPFrameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SLPFrameType::MAIN => write!(f, "MAIN"),
            SLPFrameType::SHADOW => write!(f, "SHADOW"),
        }
    }
}

/// Frame info data in an SLP file.
pub struct SLPFrameInfoData {
    /// Offset of the command table.
    pub cmd_table_offset: u32,
    /// Offset of the bounds table.
    pub bounds_table_offset: u32,
    /// Offset of the palette.
    palette_offset: u32,
    /// Properties.
    properties: u32,
    /// Width of the frame.
    pub width: i32,
    /// Height of the frame.
    pub height: i32,
    /// X coordinate of the hotspot.
    hotspot_x: i32,
    /// Y coordinate of the hotspot.
    hotspot_y: i32,
}

impl SLPFrameInfoData {
    /// Create a new SLP frame info.
    ///
    /// # Arguments
    ///
    /// * `cmd_table_offset` - Offset of the command table.
    /// * `bounds_table_offset` - Offset of the bounds table.
    /// * `palette_offset` - Offset of the palette.
    /// * `properties` - Properties.
    /// * `width` - Width of the frame.
    /// * `height` - Height of the frame.
    /// * `hotspot_x` - X coordinate of the hotspot.
    /// * `hotspot_y` - Y coordinate of the hotspot.
    ///
    /// # Returns
    ///
    /// New SLP frame info.
    pub fn new(
        cmd_table_offset: u32,
        bounds_table_offset: u32,
        palette_offset: u32,
        properties: u32,
        width: i32,
        height: i32,
        hotspot_x: i32,
        hotspot_y: i32,
    ) -> Self {
        Self {
            cmd_table_offset,
            bounds_table_offset,
            palette_offset,
            properties,
            width,
            height,
            hotspot_x,
            hotspot_y,
        }
    }
}

impl UnpackFixedSize for SLPFrameInfoData {
    fn from_buffer(buffer: &[u8], offset: usize) -> Self {
        let mut byte_reader = Cursor::new(&buffer[offset..offset + 4]);
        let cmd_table_offset: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&buffer[offset + 4..offset + 8]);
        let bounds_table_offset: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&buffer[offset + 8..offset + 12]);
        let palette_offset: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&buffer[offset + 12..offset + 16]);
        let properties: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&buffer[offset + 16..offset + 20]);
        let width: i32 = byte_reader.read_i32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&buffer[offset + 20..offset + 24]);
        let height: i32 = byte_reader.read_i32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&buffer[offset + 24..offset + 28]);
        let hotspot_x: i32 = byte_reader.read_i32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&buffer[offset + 28..offset + 32]);
        let hotspot_y: i32 = byte_reader.read_i32::<LittleEndian>().unwrap();

        return SLPFrameInfoData::new(
            cmd_table_offset,
            bounds_table_offset,
            palette_offset,
            properties,
            width,
            height,
            hotspot_x,
            hotspot_y,
        );
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut byte_reader = Cursor::new(&bytes[0..4]);
        let cmd_table_offset: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&bytes[4..8]);
        let bounds_table_offset: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&bytes[8..12]);
        let palette_offset: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&bytes[12..16]);
        let properties: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&bytes[16..20]);
        let width: i32 = byte_reader.read_i32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&bytes[20..24]);
        let height: i32 = byte_reader.read_i32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&bytes[24..28]);
        let hotspot_x: i32 = byte_reader.read_i32::<LittleEndian>().unwrap();

        let mut byte_reader = Cursor::new(&bytes[28..32]);
        let hotspot_y: i32 = byte_reader.read_i32::<LittleEndian>().unwrap();

        return SLPFrameInfoData::new(
            cmd_table_offset,
            bounds_table_offset,
            palette_offset,
            properties,
            width,
            height,
            hotspot_x,
            hotspot_y,
        );
    }
}

impl fmt::Display for SLPFrameInfoData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cmd_table_offset: {:#x}\nbounds_table_offset: {:#x}\npalette_offset: {}\nproperties: {:#x}\nwidth: {}\nheight: {}\nhotspot_x: {}\nhotspot_y: {}",
            self.cmd_table_offset,
            self.bounds_table_offset,
            self.palette_offset,
            self.properties,
            self.width,
            self.height,
            self.hotspot_x,
            self.hotspot_y
        )
    }
}

/// Frame info in an SLP file.
pub struct SLPFrameInfo {
    /// Frame info data.
    pub data: SLPFrameInfoData,
    /// Frame type.
    pub frame_type: SLPFrameType,
    /// SLP version.
    pub slp_version: SLPVersion,
}

impl SLPFrameInfo {
    /// Create a new SLP frame info.
    ///
    /// # Arguments
    ///
    /// * `cmd_table_offset` - Offset of the command table.
    /// * `bounds_table_offset` - Offset of the bounds table.
    /// * `palette_offset` - Offset of the palette.
    /// * `properties` - Properties.
    /// * `width` - Width of the frame.
    /// * `height` - Height of the frame.
    /// * `hotspot_x` - X coordinate of the hotspot.
    /// * `hotspot_y` - Y coordinate of the hotspot.
    /// * `frame_type` - Frame type.
    /// * `slp_version` - SLP version.
    ///
    /// # Returns
    ///
    /// New SLP frame info.
    pub fn new(
        cmd_table_offset: u32,
        bounds_table_offset: u32,
        palette_offset: u32,
        properties: u32,
        width: i32,
        height: i32,
        hotspot_x: i32,
        hotspot_y: i32,
        frame_type: SLPFrameType,
        slp_version: SLPVersion,
    ) -> Self {
        Self {
            data: SLPFrameInfoData::new(
                cmd_table_offset,
                bounds_table_offset,
                palette_offset,
                properties,
                width,
                height,
                hotspot_x,
                hotspot_y,
            ),
            frame_type,
            slp_version,
        }
    }

    /// Create a new SLP frame info from existing frame info data.
    ///
    /// # Arguments
    ///
    /// * `data` - Frame info data.
    /// * `frame_type` - Frame type.
    /// * `slp_version` - SLP version.
    ///
    /// # Returns
    ///
    /// New SLP frame info.
    pub fn from_data(
        data: SLPFrameInfoData,
        frame_type: SLPFrameType,
        slp_version: SLPVersion,
    ) -> Self {
        Self {
            data,
            frame_type,
            slp_version,
        }
    }
}

impl fmt::Display for SLPFrameInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "data: {}\nframe_type: {:?}\nslp_version: {:?}",
            self.data,
            self.frame_type.to_string(),
            self.slp_version
        )
    }
}
