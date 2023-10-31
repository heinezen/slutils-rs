// Copyright 2023-2023 the slutils-rs authors.

use std::collections::HashMap;
use std::fmt;

use crate::util::image::RGBAImageConvertible;
use crate::util::matrix::Matrix2D;
use crate::util::pixel::RGBAConvertible;

use super::definitions::SLP_FRAME_BOUNDS_SIZE;
use super::definitions::SLP_FRAME_CMD_OFFSET_SIZE;
use super::frame_info::SLPFrameInfo;
use super::pixel::PalettePixel;
use super::pixel::RGBAPixel;
use super::pixel::SLPPixelType;
use super::row_bound::SLPRowBound;
use super::row_bound::SLPRowBoundData;
use super::types::SLPRowOffset;
use super::unpack::UnpackFixedSize;
use super::unpack::UnpackFrameData;

/// SLP frame data.
pub struct SLPFrameData {
    /// Bounds table data.
    bounds_table: Vec<SLPRowBoundData>,
    /// Command table.
    cmd_table: Vec<SLPRowOffset>,
    /// Row commands data.
    row_data: Vec<Vec<u8>>,
}

/// SLP frame.
pub struct SLPFrame<T> {
    /// Frame data.
    data: Option<SLPFrameData>,
    /// Bounds table.
    bounds_table: Vec<SLPRowBound>,
    /// Command table.
    cmd_table: Vec<SLPRowOffset>,
    /// Pixels in the frame.
    pixels: Vec<Vec<T>>,
}

impl SLPFrame<PalettePixel> {
    /// Create a new SLP frame.
    ///
    /// # Arguments
    ///
    /// * `bounds_table` - Bounds table.
    /// * `cmd_table` - Command table.
    /// * `row_data` - Row data.
    ///
    /// # Returns
    ///
    /// New SLP frame.
    pub fn new(
        bounds_table: Vec<SLPRowBound>,
        cmd_table: Vec<SLPRowOffset>,
        row_data: Vec<Vec<PalettePixel>>,
    ) -> Self {
        Self {
            data: None,
            bounds_table,
            cmd_table,
            pixels: row_data,
        }
    }
}

/// Get the number of pixels following certain row commands.
///
/// Commands 0x01, 0x06, 0x07, 0x0A, 0x0B store the number of pixels either in the command itself
/// or in the next byte. If `cmd >> n` is not 0, this value is used. Otherwise, the next byte is
/// used.
///
/// # Arguments
///
/// * `buffer` - The buffer to read from.
/// * `cmd` - Row command.
/// * `n` - Number of bits to shift the command to the right.
/// * `pos` - Current position in the buffer.
///
/// # Returns
///
/// The number of pixels to read and the new position in the buffer.
fn cmd_or_next(buffer: &[u8], cmd: u8, n: u8, pos: usize) -> (u8, usize) {
    let packed_in_cmd = cmd >> n;

    if packed_in_cmd == 0 {
        let next = pos + 1;
        return (buffer[next], next);
    } else {
        return (packed_in_cmd, pos);
    }
}

impl UnpackFrameData<PalettePixel> for SLPFrame<PalettePixel> {
    fn from_buffer(buffer: &[u8], frame_info: &SLPFrameInfo) -> Self {
        let bounds_table = Self::decode_bounds_table(buffer, frame_info);
        let cmd_table = Self::decode_cmd_table(buffer, frame_info);
        let row_data = Self::decode_frame(buffer, frame_info, &bounds_table, &cmd_table);

        return Self::new(bounds_table, cmd_table, row_data);
    }

    // FIXME: This should return a result and use usize::try_from
    fn decode_bounds_table(buffer: &[u8], frame_info: &SLPFrameInfo) -> Vec<SLPRowBound> {
        let mut bounds_table = Vec::<SLPRowBound>::new();
        for j in 0..frame_info.data.height {
            let offset =
                frame_info.data.bounds_table_offset as usize + (j as usize) * SLP_FRAME_BOUNDS_SIZE;
            let bounds = SLPRowBound::from_buffer(buffer, offset);
            bounds_table.push(bounds);
        }

        return bounds_table;
    }

    // FIXME: This should return a result and use usize::try_from
    fn decode_cmd_table(buffer: &[u8], frame_info: &SLPFrameInfo) -> Vec<SLPRowOffset> {
        let mut row_offsets = Vec::<u32>::new();
        for j in 0..frame_info.data.height {
            let offset: usize = frame_info.data.cmd_table_offset as usize
                + (j as usize) * SLP_FRAME_CMD_OFFSET_SIZE;
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

    // FIXME: This should return a result and use usize::try_from
    fn decode_frame(
        buffer: &[u8],
        frame_info: &SLPFrameInfo,
        bounds_table: &[SLPRowBound],
        cmd_table: &[SLPRowOffset],
    ) -> Vec<Vec<PalettePixel>> {
        let mut row_data = Vec::<Vec<PalettePixel>>::new();
        for i in 0..frame_info.data.height {
            let row_offset = cmd_table.get(i as usize).unwrap();
            let bounds = bounds_table.get(i as usize).unwrap();

            let row = Self::decode_row(
                buffer,
                bounds,
                *row_offset as usize,
                frame_info.data.width as usize,
            );
            row_data.push(row);
        }

        return row_data;
    }

    fn decode_row(
        buffer: &[u8],
        bounds: &SLPRowBound,
        first_cmd_offset: usize,
        expected_size: usize,
    ) -> Vec<PalettePixel> {
        let mut row = Vec::<PalettePixel>::new();

        if bounds.full_row {
            for _ in 0..expected_size {
                row.push(PalettePixel::new(SLPPixelType::Transparent, 0));
            }
            return row;
        }

        for _ in 0..bounds.get_left() {
            row.push(PalettePixel::new(SLPPixelType::Transparent, 0));
        }

        let mut color_cmds = Self::decode_row_cmds(
            buffer,
            first_cmd_offset,
            expected_size - (bounds.get_left() + bounds.get_right()) as usize,
        );
        row.append(&mut color_cmds);

        for _ in 0..bounds.get_right() {
            row.push(PalettePixel::new(SLPPixelType::Transparent, 0));
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
            assert!(
                pixels.len() <= expected_size,
                "Expected {} pixels, but read {} without reaching end or row. dpos = {:#x}",
                expected_size,
                pixels.len(),
                dpos
            );

            cmd = *buffer.get(dpos).unwrap();

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
                        color = *buffer.get(dpos).unwrap();
                        pixels.push(PalettePixel::new(SLPPixelType::Palette, color));
                    }
                }
                // Lesser skip
                0b0000_0001 => {
                    (count, dpos) = cmd_or_next(buffer, cmd, 2, dpos);
                    for _ in 0..count {
                        pixels.push(PalettePixel::new(SLPPixelType::Transparent, 0));
                    }
                }
                _ => {
                    match lower_nibble {
                        // Big draw
                        0x02 => {
                            dpos += 1;
                            nextbyte = *buffer.get(dpos).unwrap();
                            pixel_count = ((higher_nibble << 4) + nextbyte) as u32;

                            for _ in 0..pixel_count {
                                dpos += 1;
                                color = *buffer.get(dpos).unwrap();
                                pixels.push(PalettePixel::new(SLPPixelType::Palette, color));
                            }
                        }
                        // Big skip
                        0x03 => {
                            dpos += 1;
                            nextbyte = *buffer.get(dpos).unwrap();
                            pixel_count = ((higher_nibble << 4) + nextbyte) as u32;

                            for _ in 0..pixel_count {
                                pixels.push(PalettePixel::new(SLPPixelType::Transparent, 0));
                            }
                        }
                        // Player color
                        0x06 => {
                            (count, dpos) = cmd_or_next(buffer, cmd, 4, dpos);
                            for _ in 0..count {
                                dpos += 1;
                                color = *buffer.get(dpos).unwrap();

                                pixels.push(PalettePixel::new(SLPPixelType::Player, color));
                            }
                        }
                        // fill palette color
                        0x07 => {
                            (count, dpos) = cmd_or_next(buffer, cmd, 4, dpos);

                            dpos += 1;
                            color = buffer.get(dpos).unwrap().clone();

                            for _ in 0..count {
                                pixels.push(PalettePixel::new(SLPPixelType::Palette, color));
                            }
                        }
                        // fill player color
                        0x0A => {
                            (count, dpos) = cmd_or_next(buffer, cmd, 4, dpos);

                            dpos += 1;
                            color = buffer.get(dpos).unwrap().clone();

                            for _ in 0..count {
                                pixels.push(PalettePixel::new(SLPPixelType::Player, color));
                            }
                        }
                        // shadow fill
                        0x0B => {
                            (count, dpos) = cmd_or_next(buffer, cmd, 4, dpos);

                            for _ in 0..count {
                                pixels.push(PalettePixel::new(SLPPixelType::Shadow, 0));
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
                                pixels.push(PalettePixel::new(SLPPixelType::Special1, 0));
                            }
                            // outline 1 multi draw
                            0x50 => {
                                dpos += 1;
                                pixel_count = buffer.get(dpos).unwrap().clone() as u32;

                                for _ in 0..pixel_count {
                                    pixels.push(PalettePixel::new(SLPPixelType::Special1, 0));
                                }
                            }
                            // outline 2 draw
                            0x60 => {
                                pixels.push(PalettePixel::new(SLPPixelType::Special2, 0));
                            }
                            // outline 2 multi draw
                            0x70 => {
                                dpos += 1;
                                pixel_count = buffer.get(dpos).unwrap().clone() as u32;

                                for _ in 0..pixel_count {
                                    pixels.push(PalettePixel::new(SLPPixelType::Special2, 0));
                                }
                            }
                            // dither
                            0x80 => {}
                            // premultiplied alpha
                            0x90 => {}
                            // original alpha
                            0xA0 => {}
                            _ => panic!(
                                "Unknown extended slp draw command: {cmd:#x} at dpos {dpos:#x}"
                            ),
                        },
                        _ => panic!("Unknown slp draw command: {cmd:#} at dpos {dpos:#}"),
                    }
                }
            }

            dpos += 1;
        }

        return pixels;
    }
}

impl RGBAImageConvertible for SLPFrame<PalettePixel> {
    fn to_rgba_matrix(&self) -> Matrix2D<[u8; 4]> {
        let height = self.pixels.len();
        let width = self.pixels.get(0).unwrap().len();
        let mut matrix = Matrix2D::<[u8; 4]>::zeros(width, height);

        for (i, row) in self.pixels.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                // TODO: lookup table from palette
                let rgba = pixel.to_rgba(HashMap::<usize, [u8; 4]>::new());
                matrix[(i, j)] = rgba;
            }
        }

        return matrix;
    }

    fn to_rgba_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

        for row in self.pixels.iter() {
            for pixel in row.iter() {
                // TODO: lookup table from palette
                let rgba = pixel.to_rgba(HashMap::<usize, [u8; 4]>::new());
                bytes.push(rgba[0]);
                bytes.push(rgba[1]);
                bytes.push(rgba[2]);
                bytes.push(rgba[3]);
            }
        }

        return bytes;
    }

    fn to_image(&self) -> image::RgbaImage {
        let height = self.pixels.len();
        let width = self.pixels.get(0).unwrap().len();
        let image = image::RgbaImage::from_raw(width as u32, height as u32, self.to_rgba_bytes());

        return image.unwrap();
    }
}

impl fmt::Display for SLPFrame<PalettePixel> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();

        out.push_str("| Row   | Start Offset | Bounds (left/right) |\n");
        out.push_str("|-------|--------------|---------------------|\n");

        for i in 0..self.pixels.len() {
            let start_offset = self.cmd_table.get(i).unwrap();
            let bounds = self.bounds_table.get(i).unwrap();

            let row = format!(
                "| {:<5} | {:<#12x} | {:>4} / {:<12} |\n",
                i,
                start_offset,
                bounds.get_left(),
                bounds.get_right()
            );
            out.push_str(row.as_str());
        }

        self.pixels.iter().for_each(|row| {
            out.push_str(
                row.iter()
                    .map(|p| format!("{p}"))
                    .collect::<String>()
                    .as_str(),
            );
            out.push('\n');
        });

        write!(f, "{out}")
    }
}

impl fmt::Display for SLPFrame<RGBAPixel> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();

        out.push_str("| Row   | Start Offset | Outline (left/right) |\n");
        out.push_str("|-------|--------------|----------------------|\n");

        for i in 0..self.pixels.len() {
            let start_offset = self.cmd_table.get(i).unwrap();
            let bounds = self.bounds_table.get(i).unwrap();

            let row = format!(
                "| {:<5} | {:<#12x} | {:>4} / {:<13} |\n",
                i,
                start_offset,
                bounds.get_left(),
                bounds.get_right()
            );
            out.push_str(row.as_str());
        }

        write!(f, "{}", out)
    }
}
