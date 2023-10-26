// Copyright 2023-2023 the slutils-rs authors.

use std::mem::size_of;

use crate::slp::{frame_data::SLPFrameData, frame_info::SLPFrameInfo, header::SLPHeader};

use super::header::Unpack;

pub struct SLPFile {
    pub header: SLPHeader,
    frame_info: Vec<SLPFrameInfo>,
    frame_data: Vec<SLPFrameData>,
}

pub fn parse_slp(bytes: Vec<u8>) -> SLPFile {
    let header = SLPHeader::from_buffer(&bytes, 0);

    let mut frame_info = Vec::<SLPFrameInfo>::new();
    let mut frame_data = Vec::<SLPFrameData>::new();

    SLPFile {
        header,
        frame_info,
        frame_data,
    }
}
