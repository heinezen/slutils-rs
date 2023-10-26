// Copyright 2023-2023 the slutils-rs authors.

pub struct SLPOutline {
    left: u16,
    right: u16,
}

pub struct SLPFrameData {
    outline_table: Vec<SLPOutline>,
    cmd_table: Vec<u32>,
    pixel_data: Vec<u8>,
}
