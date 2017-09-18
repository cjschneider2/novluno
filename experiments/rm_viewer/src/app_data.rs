use std::path::PathBuf;

use gdk_pixbuf::*;

use core_compat::entity::resource_file::ResourceFile;

pub struct AppData {
    pub current_path: PathBuf,
    pub current_file: PathBuf,
    pub resource_file: Option<ResourceFile>,
    pub resource_idx: isize,
    pub resource_total: usize,
    pub height: usize,
    pub width: usize,
    pub offset_x: usize,
    pub offset_y: usize,
    pub pixbuf: Option<Pixbuf>,
}

impl AppData {
    pub fn new() -> AppData {
        AppData {
            current_path: PathBuf::new(),
            current_file: PathBuf::new(),
            resource_file: None,
            resource_idx: 0,
            resource_total: 0,
            height: 0,
            width: 0,
            offset_x: 0,
            offset_y: 0,
            pixbuf: None,
        }
    }

    pub fn load_rle_at_idx(&mut self, idx: usize) {
        if self.resource_file.is_some() {
            let resource_file = self.resource_file.as_ref().unwrap();
            if idx > resource_file.resources.len() {
                return;
            }
            let img_vec = {
                let resource = &resource_file.resources[idx];
                self.width = resource.width as usize;
                self.height = resource.height as usize;
                self.offset_x = resource.offset_x as usize;
                self.offset_y = resource.offset_y as usize;
                let mut vec = Vec::<u8>::new();
                for pixel in resource.image.iter() {
                    vec.push(pixel.r);
                    vec.push(pixel.g);
                    vec.push(pixel.b);
                    vec.push(pixel.a);
                }
                vec
            };
            self.pixbuf = Some(Pixbuf::new_from_vec(
                img_vec,
                0, /* GDK_COLORSPACE_RGB */
                true,
                8, /* color depth */
                self.width as i32,
                self.height as i32,
                (self.width * 4) as i32) /* row-stride */);
            self.resource_idx = idx as isize;
        }
    }
} // end impl ApplicationData
