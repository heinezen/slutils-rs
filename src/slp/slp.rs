// Copyright 2023-2023 the slutils-rs authors.

use super::definitions::SLP_FRAME_INFO_SIZE;
use super::definitions::SLP_HEADER_SIZE;
use super::frame::SLPFrame;
use super::frame_info::SLPFrameInfo;
use super::frame_info::SLPFrameInfoData;
use super::frame_info::SLPFrameType;
use super::header::SLPHeader;
use super::header::SLPHeaderData;
use super::pixel::PalettePixel;
use super::unpack::UnpackFixedSize;
use super::unpack::UnpackFrameData;

/// SLP file.
pub struct SLPFile {
    /// SLP header.
    pub header: SLPHeader,
    /// SLP frame infos.
    pub frame_infos: Vec<SLPFrameInfo>,
    /// SLP frames.
    pub frames: Vec<SLPFrame<PalettePixel>>,
}

/// Parse a single SLP file.
///
/// # Arguments
///
/// * `bytes` - The bytes of the SLP file.
///
/// # Returns
///
/// The parsed SLP file.
pub fn parse_slp(bytes: Vec<u8>) -> SLPFile {
    let header_data = SLPHeaderData::from_buffer(&bytes, 0);
    let header = SLPHeader::from_data(header_data);

    let mut frame_infos = Vec::<SLPFrameInfo>::new();
    for i in 0..header.get_num_frames() {
        let offset = SLP_HEADER_SIZE + (i as usize) * SLP_FRAME_INFO_SIZE;
        let info_data = SLPFrameInfoData::from_buffer(&bytes, offset);
        frame_infos.push(SLPFrameInfo::from_data(
            info_data,
            SLPFrameType::MAIN,
            header.data.version,
        ));
    }

    let mut frame_datas = Vec::<SLPFrame<PalettePixel>>::new();
    for i in 0..header.get_num_frames() {
        let frame_info = frame_infos.get(i as usize).unwrap();
        let frame = SLPFrame::from_buffer(&bytes, frame_info);
        frame_datas.push(frame);
    }

    SLPFile {
        header,
        frame_infos,
        frames: frame_datas,
    }
}
