// Copyright 2023-2023 the slutils-rs authors.

use std::fmt;
use std::mem::size_of;

use super::frame_info;
use super::pixel::PalettePixel;
use super::pixel::PixelType;
use super::pixel::RGBAPixel;
use super::unpack::UnpackFixedSize;
use super::unpack::UnpackFrameData;

pub struct SLPFrameBound {
    pub left: u16,
    pub right: u16,
}

impl SLPFrameBound {
    pub fn new(left: u16, right: u16) -> Self {
        Self { left, right }
    }
}

impl UnpackFixedSize for SLPFrameBound {
    fn from_buffer(buffer: &[u8], offset: usize) -> Self {
        let left = u16::from_le_bytes([buffer[offset], buffer[offset + 1]]);
        let right = u16::from_le_bytes([buffer[offset + 2], buffer[offset + 3]]);

        return SLPFrameBound::new(left, right);
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let left = u16::from_le_bytes([bytes[0], bytes[1]]);
        let right = u16::from_le_bytes([bytes[2], bytes[3]]);

        return SLPFrameBound::new(left, right);
    }
}

pub struct SLPFrameData<T> {
    bounds_table: Vec<SLPFrameBound>,
    cmd_table: Vec<u32>,
    row_data: Vec<Vec<T>>,
}

impl SLPFrameData<PalettePixel> {
    pub fn new(
        outline_table: Vec<SLPFrameBound>,
        cmd_table: Vec<u32>,
        row_data: Vec<Vec<PalettePixel>>,
    ) -> Self {
        Self {
            bounds_table: outline_table,
            cmd_table,
            row_data: row_data,
        }
    }
}

fn cmd_or_next(buffer: &[u8], cmd: u8, n: u8, pos: usize) -> (u8, usize) {
    let packed_in_cmd = cmd >> n;

    if packed_in_cmd != 0 {
        return (packed_in_cmd, pos);
    } else {
        let next = pos + 1;
        return (buffer[next], next);
    }
}

impl UnpackFrameData<PalettePixel> for SLPFrameData<PalettePixel> {
    fn from_buffer(buffer: &[u8], frame_info: &frame_info::SLPFrameInfo) -> Self {
        let outline_table = SLPFrameData::<PalettePixel>::decode_outline_table(buffer, frame_info);
        let cmd_table = SLPFrameData::<PalettePixel>::decode_cmd_table(buffer, frame_info);
        let row_data = SLPFrameData::<PalettePixel>::decode_frame(
            buffer,
            frame_info,
            &outline_table,
            &cmd_table,
        );

        return SLPFrameData::new(outline_table, cmd_table, row_data);
    }

    fn decode_outline_table(
        buffer: &[u8],
        frame_info: &super::frame_info::SLPFrameInfo,
    ) -> Vec<SLPFrameBound> {
        let mut outline_table = Vec::<SLPFrameBound>::new();
        for j in 0..frame_info.height {
            let offset = frame_info.outline_table_offset as usize
                + (j as usize) * size_of::<SLPFrameBound>();
            let outline = SLPFrameBound::from_buffer(&buffer, offset);
            outline_table.push(outline);
        }
        return outline_table;
    }

    fn decode_cmd_table(buffer: &[u8], frame_info: &frame_info::SLPFrameInfo) -> Vec<u32> {
        let mut row_offsets = Vec::<u32>::new();
        for j in 0..frame_info.height {
            let offset: usize = frame_info.cmd_table_offset as usize + (j as usize) * 4;
            let row = u32::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]);
            row_offsets.push(row);
        }
        return row_offsets;
    }

    fn decode_frame(
        buffer: &[u8],
        frame_info: &frame_info::SLPFrameInfo,
        outline_table: &Vec<SLPFrameBound>,
        cmd_table: &Vec<u32>,
    ) -> Vec<Vec<PalettePixel>> {
        let mut row_data = Vec::<Vec<PalettePixel>>::new();
        for i in 0..frame_info.height {
            let row_offset = cmd_table.get(i as usize).unwrap();
            let outline = outline_table.get(i as usize).unwrap();

            let row = SLPFrameData::<PalettePixel>::decode_row(
                buffer,
                outline,
                *row_offset as usize,
                frame_info.width as usize,
            );
            row_data.push(row);
        }

        return row_data;
    }

    fn decode_row(
        buffer: &[u8],
        outline: &SLPFrameBound,
        first_cmd_offset: usize,
        expected_size: usize,
    ) -> Vec<PalettePixel> {
        let mut row = Vec::<PalettePixel>::new();

        for _ in 0..outline.left {
            row.push(PalettePixel::new(PixelType::TRANSPARENT, 0));
        }

        let mut color_cmds = SLPFrameData::<PalettePixel>::decode_row_cmds(
            buffer,
            first_cmd_offset,
            expected_size - (outline.left + outline.right) as usize,
        );
        row.append(&mut color_cmds);

        for _ in 0..outline.right {
            row.push(PalettePixel::new(PixelType::TRANSPARENT, 0));
        }

        return row;
    }

    fn decode_row_cmds(
        buffer: &[u8],
        first_cmd_offset: usize,
        expected_size: usize,
    ) -> Vec<PalettePixel> {
        let mut pixels = Vec::<PalettePixel>::new();

        let mut dpos: usize = first_cmd_offset;
        let mut count: u8;

        let mut cmd: u8;
        let mut nextbyte: u8;
        let mut color: u8;
        let mut pixel_count: u32;

        let mut lower_nibble: u8;
        let mut higher_nibble: u8;
        let mut lowest_crumb: u8;

        let mut eor: bool = false;

        while !eor {
            if pixels.len() > expected_size {
                panic!("Expected {} pixels, got {}", expected_size, pixels.len());
            }

            cmd = buffer.get(dpos).unwrap().clone();

            lower_nibble = cmd & 0x0F;
            higher_nibble = cmd & 0xF0;
            lowest_crumb = cmd & 0b0000_0011;

            if lower_nibble == 0x0F {
                // End of row
                eor = true;
                continue;
            }

            match lowest_crumb {
                // Lesser draw
                0b0000_0000 => {
                    pixel_count = (cmd >> 2) as u32;
                    for _ in 0..pixel_count {
                        dpos += 1;
                        color = buffer.get(dpos).unwrap().clone();
                        pixels.push(PalettePixel::new(PixelType::PALETTE, color));
                    }
                }
                // Lesser skip
                0b0000_0001 => {
                    (count, dpos) = cmd_or_next(buffer, cmd, 2, dpos);
                    for _ in 0..count {
                        pixels.push(PalettePixel::new(PixelType::TRANSPARENT, 0));
                    }
                }
                _ => {
                    match lower_nibble {
                        // Big draw
                        0x02 => {
                            dpos += 1;
                            nextbyte = buffer.get(dpos).unwrap().clone();
                            pixel_count = ((higher_nibble << 4) + nextbyte) as u32;

                            for _ in 0..pixel_count {
                                dpos += 1;
                                color = buffer.get(dpos).unwrap().clone();
                                pixels.push(PalettePixel::new(PixelType::PALETTE, color));
                            }
                        }
                        // Big skip
                        0x03 => {
                            dpos += 1;
                            nextbyte = buffer.get(dpos).unwrap().clone();
                            pixel_count = ((higher_nibble << 4) + nextbyte) as u32;

                            for _ in 0..pixel_count {
                                pixels.push(PalettePixel::new(PixelType::TRANSPARENT, 0));
                            }
                        }
                        // Player color
                        0x06 => {
                            (count, dpos) = cmd_or_next(buffer, cmd, 4, dpos);
                            for _ in 0..count {
                                dpos += 1;
                                color = buffer.get(dpos).unwrap().clone();

                                pixels.push(PalettePixel::new(PixelType::PLAYER, color));
                            }
                        }
                        // fill palette color
                        0x07 => {
                            (count, dpos) = cmd_or_next(buffer, cmd, 4, dpos);

                            dpos += 1;
                            color = buffer.get(dpos).unwrap().clone();

                            for _ in 0..count {
                                pixels.push(PalettePixel::new(PixelType::PALETTE, color));
                            }
                        }
                        // fill player color
                        0x0A => {
                            (count, dpos) = cmd_or_next(buffer, cmd, 4, dpos);

                            dpos += 1;
                            color = buffer.get(dpos).unwrap().clone();

                            for _ in 0..count {
                                pixels.push(PalettePixel::new(PixelType::PLAYER, color));
                            }
                        }
                        // shadow fill
                        0x0B => {
                            (count, dpos) = cmd_or_next(buffer, cmd, 4, dpos);

                            for _ in 0..count {
                                pixels.push(PalettePixel::new(PixelType::SHADOW, 0));
                            }
                        }
                        // Extended command
                        0x0E => match higher_nibble {
                            // xflip on
                            0x00 => {}
                            // xflip off
                            0x10 => {}
                            // switch to normal table
                            0x20 => {}
                            // switch to alternate table
                            0x30 => {}
                            // outline 1 draw
                            0x40 => {
                                pixels.push(PalettePixel::new(PixelType::SPECIAL1, 0));
                            }
                            // outline 1 multi draw
                            0x50 => {
                                dpos += 1;
                                pixel_count = buffer.get(dpos).unwrap().clone() as u32;

                                for _ in 0..pixel_count {
                                    pixels.push(PalettePixel::new(PixelType::SPECIAL1, 0));
                                }
                            }
                            // outline 2 draw
                            0x60 => {
                                pixels.push(PalettePixel::new(PixelType::SPECIAL2, 0));
                            }
                            // outline 2 multi draw
                            0x70 => {
                                dpos += 1;
                                pixel_count = buffer.get(dpos).unwrap().clone() as u32;

                                for _ in 0..pixel_count {
                                    pixels.push(PalettePixel::new(PixelType::SPECIAL2, 0));
                                }
                            }
                            // dither
                            0x80 => {}
                            // premultiplied alpha
                            0x90 => {}
                            // original alpha
                            0xA0 => {}
                            _ => panic!(
                                "Unknown extended slp draw command: {:#x} at dpos {:#x}",
                                cmd, dpos
                            ),
                        },
                        _ => panic!("Unknown slp draw command: {:#x} at dpos {:#x}", cmd, dpos),
                    }
                }
            }

            dpos += 1;
        }

        return pixels;
    }
}

impl fmt::Display for SLPFrameData<PalettePixel> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();

        out.push_str("| Row   | Start Offset | Outline (left/right) |\n");
        out.push_str("|-------|--------------|----------------------|\n");

        for i in 0..self.row_data.len() {
            let start_offset = self.cmd_table.get(i).unwrap();
            let outline = self.bounds_table.get(i).unwrap();

            let row = format!(
                "| {:<5} | {:<#12x} | {:>4} / {:<13} |\n",
                i, start_offset, outline.left, outline.right
            );
            out.push_str(row.as_str());
        }

        write!(f, "{}", out)
    }
}

impl fmt::Display for SLPFrameData<RGBAPixel> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();

        out.push_str("| Row   | Start Offset | Outline (left/right) |\n");
        out.push_str("|-------|--------------|----------------------|\n");

        for i in 0..self.row_data.len() {
            let start_offset = self.cmd_table.get(i).unwrap();
            let outline = self.bounds_table.get(i).unwrap();

            let row = format!(
                "| {:<5} | {:<#12x} | {:>4} / {:<13} |\n",
                i, start_offset, outline.left, outline.right
            );
            out.push_str(row.as_str());
        }

        write!(f, "{}", out)
    }
}
