//! This module has the methods for decoding the Redmoon Online RLE files and
//! storing / exporting them into various formats.

use std::str::from_utf8;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian as LE;

use crate::error::Error;
use crate::utility::pixel::Pixel;
use crate::entity::resource::Resource;
use crate::entity::resource_file::ResourceFile;

pub fn parse_rle(file_number: u32, data: &[u8]) -> Result<ResourceFile, Error> {
    let mut cursor = Cursor::new(data);
    let mut resource_file = ResourceFile::new();

    // file type string: needs to equal "Resource File\n"
    let (file_type, _rest) = if data.len() >= 14 {
        data.split_at(14)
    } else {
        return Err(Error::MissingRleIdentifier);
    };
    let file_type: &str = from_utf8(file_type)?;

    if file_type != "Resource File\0" {
        return Err(Error::MissingRleIdentifier);
    }

    // start reading after the "Resource file string"
    cursor.seek(SeekFrom::Start(14u64))?;

    // unknown_1: 4 Unknown bytes; (next free offset?)
    let tmp = cursor.read_u32::<LE>()?;

    // total_resources: 4 bytes (u32)
    let total_resources = cursor.read_u32::<LE>()?;

    // resource_offsets: [total_resources; u32]
    let mut resource_offsets = Vec::<u32>::new();
    for idx in 0..total_resources {
        let val = cursor.read_u32::<LE>()?;
        resource_offsets.push(val);
    }

    /*
    dbg!(file_number);
    println!("Loading {} resources at offsets:{:?}", total_resources, resource_offsets);
    dbg!(data.len());
    */

    for (idx, offset) in resource_offsets.iter().enumerate() {
        let offset = match *offset {
            0 => {
                // we'd skip 0 (null) offsets as I think they are just placeholders in the file
                // but we can't ignore them in the resource offset list because the index of the
                // resource is important.
                continue;
            }
            offset => offset
        };

        let mut resource = Resource::new();
        cursor.seek(SeekFrom::Start(offset as u64))?;

        // resource id's
        resource.file_num = file_number;
        resource.index = idx as u32;
        resource.offset = offset;

        // read the resource header
        resource.len = cursor.read_u32::<LE>()?;
        resource.offset_x = cursor.read_i32::<LE>()?;
        resource.offset_y = cursor.read_i32::<LE>()?;
        resource.width = cursor.read_i32::<LE>()?;
        resource.height = cursor.read_i32::<LE>()?;
        resource.unknown_1 = cursor.read_u32::<LE>()?;
        resource.unknown_2 = cursor.read_u32::<LE>()?;
        resource.unknown_3 = cursor.read_u32::<LE>()?;
        resource.unknown_4 = cursor.read_u32::<LE>()?;

        // Pre-fill the image buffer with 0's
        if resource.width < 8000 && resource.width > 0
            && resource.height < 8000 && resource.height > 0 {
            let total_px = resource.width * resource.height * 4 /* bytes / pixel */;
            for _ in 0..total_px {
                resource.image_raw.push(0x0);
            }
        } else {
            println!("wrongly sized resource: ({}, {})", resource.width, resource.height);
            // dbg!(&(resource.file_num, resource.index));
            // oversized resource
            resource.image_raw.push(0xFF); // R
            resource.image_raw.push(0xFF); // G
            resource.image_raw.push(0xFF); // B
            resource.image_raw.push(0xFF); // A
            continue;
        }

        // read the rest of the image data
        let mut x = 0i32;
        let mut y = 0i32;
        'image: loop {
            let entry_type = cursor.read_u8().unwrap();
            // println!("RLE Entry Type:{} @ offset: `{}`",
            //          entry_type,
            //          cursor.position());
            match entry_type {
                0x00 => {
                    /* End resource marker */
                    break 'image;
                }
                0x01 => {
                    /* Paint pixels */
                    let pixels = cursor.read_u32::<LE>()?;
                    for p in 0..pixels {
                        let data = cursor.read_u16::<LE>()?;
                        let (r, g, b) = format_r5g6b5_norm(data);
                        let _y = y * 4 * resource.width as i32;
                        let _x = x * 4;
                        let idx: usize = _y as usize + _x as usize;
                        resource.image_raw[idx] = r;
                        resource.image_raw[idx + 1] = g;
                        resource.image_raw[idx + 2] = b;
                        resource.image_raw[idx + 3] = 0xFF;

                        x += 1;
                    }
                }
                0x02 => {
                    /* Move `x` pos */
                    let pixels = cursor.read_i32::<LE>()?;
                    x += pixels / 2; // NOTE: the two is probably a u16 jump?
                }
                0x03 => {
                    /* Next line */
                    y += 1;
                }
                val => {
                    /*
                    println!("{:?}: dumping rest of resource into image", Error::UnknownOffsetTypeAt(cursor.position(), val));
                    if let Some(&next_offset) = resource_offsets.get(idx + 1) {
                        let rest_bytes: usize = (next_offset as usize) - (cursor.position() as usize);
                        for b_idx in 0..rest_bytes {
                            let _idx = b_idx as usize;
                            let data = cursor.read_u16::<LE>()?;
                            let (r, g, b) = format_r5g6b5_norm(data);
                            resource.image_raw[_idx] = r;
                            resource.image_raw[_idx + 1] = g;
                            resource.image_raw[_idx + 2] = b;
                            resource.image_raw[_idx + 3] = 0xFF;
                        }
                    }
                    */
                    return Err(Error::UnknownOffsetTypeAt(cursor.position(), val));
                }
            }
        }
        resource_file.resources.push(resource);
    }
    Ok(resource_file)
}

/// The pixels in the RLE files are saved as normalized 5,6,5 bit normalized RGB colors.
/// Magenta is sometimes used in the images as an alpha colour but it is relatively rare; it is
/// usually just enough to set the default colour to be transparent and "paint" over the pixels
/// with the actual colour.
// TODO: There is probably a quicker way to do this conversion without the FP mult & div ...
// TODO: Create type for r5g6b5 normalized colors and don't convert (OpenGL & DX can do this)
fn format_r5g6b5_norm(d: u16) -> (u8, u8, u8) {
    let b = ((d & 0x1F) as f32 / 31.0) * 255.0;
    let g = (((d >> 5) & 0x3F) as f32 / 63.0) * 255.0;
    let r = (((d >> 11) & 0x1F) as f32 / 31.0) * 255.0;
    (r as u8, g as u8, b as u8)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c0000000_rle() {
        let data = include_bytes!("../../../data/RLEs/Chr/C00/c0000000.rle");
        let rle = parse_rle(0, data).unwrap();
    }

    #[test]
    fn test_c0000042_rle() {
        let data = include_bytes!("../../../data/RLEs/Chr/C00/c0000042.rle");
        let rle = parse_rle(42, data).unwrap();
    }

    #[test]
    fn test_c0200188_rle() {
        let data = include_bytes!("../../../data/RLEs/Chr/C02/c0200188.rle");
        let rle = parse_rle(188, data).unwrap();
    }

    #[test]
    fn test_ico_00000_rle() {
        let data = include_bytes!("../../../data/RLEs/Ico/ico00000.rle");
        let rle = parse_rle(0, data).unwrap();
    }
}
