// Copyright 2023-2023 the slutils-rs authors.

use std::mem::size_of;

use super::frame_data::SLPFrameBound;
use super::frame_data::SLPFrameData;
use super::frame_info::SLPFrameInfo;
use super::header::SLPHeader;
use super::pixel::PalettePixel;
use super::unpack::UnpackFixedSize;
use super::unpack::UnpackFrameData;

pub struct SLPFile {
    pub header: SLPHeader,
    pub frame_infos: Vec<SLPFrameInfo>,
    pub frame_datas: Vec<SLPFrameData<PalettePixel>>,
}

pub fn parse_slp(bytes: Vec<u8>) -> SLPFile {
    let header = SLPHeader::from_buffer(&bytes, 0);

    let mut frame_infos = Vec::<SLPFrameInfo>::new();
    for i in 0..header.num_frames {
        let offset = size_of::<SLPHeader>() + (i as usize) * size_of::<SLPFrameInfo>();
        frame_infos.push(SLPFrameInfo::from_buffer(&bytes, offset));
    }

    let mut frame_datas = Vec::<SLPFrameData<PalettePixel>>::new();
    for i in 0..header.num_frames {
        let frame_info = frame_infos.get(i as usize).unwrap();

        let mut row_offsets = Vec::<u32>::new();
        let cmd_tbl_offset: u32 = frame_info.cmd_table_offset;
        for j in 0..frame_info.height {
            let offset = cmd_tbl_offset as usize + (j as usize) * 4;
            let row = u32::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ]);
            row_offsets.push(row);
        }

        let mut outline_table = Vec::<SLPFrameBound>::new();
        let outline_tbl_offset: u32 = frame_info.outline_table_offset;
        for j in 0..frame_info.height {
            let offset = outline_tbl_offset as usize + (j as usize) * size_of::<SLPFrameBound>();
            let outline = SLPFrameBound::from_buffer(&bytes, offset);
            outline_table.push(outline);
        }

        let mut row_data = Vec::<Vec<u8>>::new();
        for row_offset in row_offsets.clone() {
            let start = row_offset as usize;
            let mut end = start;
            while bytes[end] != 0x0F {
                end += 1;
            }
            let row = bytes[start..end].to_vec();
            row_data.push(row);
        }

        let frame = SLPFrameData::from_buffer(&bytes, frame_info);
        frame_datas.push(frame);
    }

    SLPFile {
        header,
        frame_infos,
        frame_datas,
    }
}
