// Copyright 2023-2023 the slutils-rs authors.

use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt;
use std::io::Cursor;

use crate::slp::unpack::UnpackFixedSize;

pub enum SLPType {
    MAIN,
    SHADOW,
}

pub struct SLPFrameInfoData {
    pub cmd_table_offset: u32,
    pub outline_table_offset: u32,
    palette_offset: u32,
    properties: u32,
    pub width: i32,
    pub height: i32,
    hotspot_x: i32,
    hotspot_y: i32,
}

impl SLPFrameInfoData {
    pub fn new(
        cmd_table_offset: u32,
        outline_table_offset: u32,
        palette_offset: u32,
        properties: u32,
        width: i32,
        height: i32,
        hotspot_x: i32,
        hotspot_y: i32,
    ) -> SLPFrameInfoData {
        SLPFrameInfoData {
            cmd_table_offset,
            outline_table_offset,
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
        let outline_table_offset: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

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
            outline_table_offset,
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
        let outline_table_offset: u32 = byte_reader.read_u32::<LittleEndian>().unwrap();

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
            outline_table_offset,
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
            "cmd_table_offset: {:#x}\noutline_table_offset: {:#x}\npalette_offset: {}\nproperties: {:#x}\nwidth: {}\nheight: {}\nhotspot_x: {}\nhotspot_y: {}",
            self.cmd_table_offset,
            self.outline_table_offset,
            self.palette_offset,
            self.properties,
            self.width,
            self.height,
            self.hotspot_x,
            self.hotspot_y
        )
    }
}
