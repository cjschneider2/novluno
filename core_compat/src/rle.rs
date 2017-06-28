//! This module has the methods for decoding the Redmoon Online RLE files and
//! storing / exporting them into various formats.

use std::str::from_utf8;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian as LE;

use util::pixel::Pixel;
use error::*;

#[derive(Debug)]
pub struct Resource {
    pub file_num: Option<u32>,
    pub index: u32,
    pub offset: u32,
    pub len: u32,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
    pub unknown_1: u32,
    pub unknown_2: u32,
    pub unknown_3: u32,
    pub unknown_4: u32,
    pub image: Vec<Pixel>,
}

impl Resource {
    pub fn new() -> Resource {
        Resource {
            file_num: None,
            index: 0,
            offset: 0,
            len: 0,
            offset_x: 0,
            offset_y: 0,
            width: 0,
            height: 0,
            unknown_1: 0,
            unknown_2: 0,
            unknown_3: 0,
            unknown_4: 0,
            image: Vec::new(),
        }
    }
}

pub struct ResourceFile {
    pub name: String,
    pub file_number: u32,
    pub resources: Vec<Resource>,
}

impl ResourceFile {
    pub fn new() -> ResourceFile {
        ResourceFile {
            name: String::new(),
            file_number: 0,
            resources: Vec::new(),
        }
    }

    pub fn load(file_number: u32, data: &[u8]) -> Result<ResourceFile, Error> {

        let mut cursor = Cursor::new(data);
        let mut resource_file = ResourceFile::new();

        // filetype string: needs to equal "Resource File\n"
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

        for (idx, offset) in resource_offsets.iter().enumerate() {
            let mut resource = Resource::new();

            cursor.seek(SeekFrom::Start(*offset as u64))?;

            // resource id's
            resource.file_num = Some(file_number);
            resource.index = idx as u32;
            resource.offset = *offset;

            // read the resource header
            resource.len = cursor.read_u32::<LE>()?;
            resource.offset_x = cursor.read_u32::<LE>()?;
            resource.offset_y = cursor.read_u32::<LE>()?;
            resource.width = cursor.read_u32::<LE>()?;
            resource.height = cursor.read_u32::<LE>()?;
            resource.unknown_1 = cursor.read_u32::<LE>()?;
            resource.unknown_2 = cursor.read_u32::<LE>()?;
            resource.unknown_3 = cursor.read_u32::<LE>()?;
            resource.unknown_4 = cursor.read_u32::<LE>()?;

            // Pre-fill the image buffer with 0's
            if resource.width < 8000 && resource.height < 8000 {
                let total_px = resource.width * resource.height;
                for _ in 0..total_px {
                    resource.image.push(Pixel::new_empty());
                }
            } else {
                // println!("oversized resource: W: {}, H: {}",
                //          resource.width,
                //          resource.height);
                resource.image.push(Pixel::new_empty());
                continue;
            }

            // read the rest of the image data
            let mut x = 0i32;
            let mut y = 0i32;
            'image: loop {
                let entry_type = cursor.read_u8().unwrap();
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
                            let idx: usize = (y as usize * resource.width as usize) + x as usize;
                            let pixel = &mut resource.image[idx];
                            pixel.r = r as u8;
                            pixel.g = g as u8;
                            pixel.b = b as u8;
                            pixel.a = 255u8;
                            x += 1;
                        }
                    }
                    0x02 => {
                        /* Move `x` pos */
                        let pixels = cursor.read_i32::<LE>()?;
                        x += pixels / 2;
                    }
                    0x03 => {
                        /* Next line */
                        y += 1;
                    }
                    _ => {
                        return Err(Error::UnknownOffsetTypeAt(cursor.position()));
                    }
                }
            }
            resource_file.resources.push(resource);
        }
        Ok(resource_file)
    }
}

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
        let data = include_bytes!("../../data/RLEs/Chr/C00/c0000000.rle");
        let rle = ResourceFile::load(0, data).unwrap();
    }
    #[test]
    fn test_c0000042_rle() {
        let data = include_bytes!("../../data/RLEs/Chr/C00/c0000042.rle");
        let rle = ResourceFile::load(42, data).unwrap();
    }
}
