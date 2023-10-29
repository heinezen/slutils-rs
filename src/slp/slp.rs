// Copyright 2023-2023 the slutils-rs authors.

use super::definitions::SLP_FRAME_INFO_SIZE;
use super::definitions::SLP_HEADER_SIZE;
use super::frame::SLPFrame;
use super::frame_info::FrameType;
use super::frame_info::SLPFrameInfo;
use super::frame_info::SLPFrameInfoData;
use super::header::SLPHeader;
use super::pixel::PalettePixel;
use super::unpack::UnpackFixedSize;
use super::unpack::UnpackFrameData;

pub struct SLPFile {
    pub header: SLPHeader,
    pub frame_infos: Vec<SLPFrameInfo>,
    pub frame_datas: Vec<SLPFrame<PalettePixel>>,
}

pub fn parse_slp(bytes: Vec<u8>) -> SLPFile {
    let header = SLPHeader::from_buffer(&bytes, 0);

    let mut frame_infos = Vec::<SLPFrameInfo>::new();
    for i in 0..header.num_frames {
        let offset = SLP_HEADER_SIZE + (i as usize) * SLP_FRAME_INFO_SIZE;
        let info_data = SLPFrameInfoData::from_buffer(&bytes, offset);
        frame_infos.push(SLPFrameInfo::from_data(
            info_data,
            FrameType::MAIN,
            header.version,
        ));
    }

    let mut frame_datas = Vec::<SLPFrame<PalettePixel>>::new();
    for i in 0..header.num_frames {
        let frame_info = frame_infos.get(i as usize).unwrap();
        let frame = SLPFrame::from_buffer(&bytes, frame_info);
        frame_datas.push(frame);
    }

    SLPFile {
        header,
        frame_infos,
        frame_datas,
    }
}
