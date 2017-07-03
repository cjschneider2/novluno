extern crate gdk_pixbuf;

use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use std::fs::read_dir;

use gtk::*;
use gdk_pixbuf::*;

use redmoon::*;
use app_error::AppError;

pub struct ApplicationGui {
    pub main_window: Window,
    pub image: Image,
    pub status_bar: Statusbar,
    pub file_list_store: ListStore,
    pub file_tree_view: TreeView,
    pub file_tree_selection: TreeSelection,
    pub open_folder_button: Button,
    pub file_chooser_dialog: FileChooserDialog,
}

impl ApplicationGui {

    pub fn update_status(&self, app_data: &ApplicationData) {
        const MSG_ID: i32 = 42;
        let status = &self.status_bar;
        status.remove_all(42);
        status.push(42,
                    &format!(" Resource: [{}/{}], W: {} H: {} X: {} Y:{}",
                            app_data.resource_idx + 1,
                            app_data.resource_total,
                            app_data.width,
                            app_data.height,
                            app_data.offset_x,
                            app_data.offset_y,
                            ));
    }

    pub fn load_new_folder(&self, app_data: &ApplicationData) -> Result<(), AppError> {
        let mut file_list: Vec<PathBuf> = Vec::new();
        if app_data.current_path.is_dir() {
        let dir = read_dir(&app_data.current_path)?;
            for entry in dir {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    file_list.push(path);
                }
            }
        }
        file_list.sort();
        for file in file_list {
            let next = self.file_list_store.append();
            if let Some(name) = file.file_name() {
                let name: &str = match name.to_str() {
                    Some(name) => name,
                    None => return Err(AppError::StringConversion),
                };
                self.file_list_store.set_value(&next, 0, &name.to_value());
            }
        }
        Ok(())
    }
}

pub struct ApplicationData {
    pub current_path: PathBuf,
    pub current_file: PathBuf,
    pub resource_file: Option<rle::ResourceFile>,
    pub resource_idx: isize,
    pub resource_total: usize,
    pub height: usize,
    pub width: usize,
    pub offset_x: usize,
    pub offset_y: usize,
    pub pixbuf: Option<Pixbuf>,
}

impl ApplicationData {

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
            self.pixbuf = Some(gdk_pixbuf::Pixbuf::new_from_vec(
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


}

pub fn read_rle_from_bytes(data: &[u8]) -> Result<rle::ResourceFile, AppError> {
    let rle = rle::ResourceFile::load(0, data)?;
    Ok(rle)
}

pub fn read_rle_from_file(file: &PathBuf) -> Result<rle::ResourceFile, AppError> {
    let mut file = File::open(file)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let rle = rle::ResourceFile::load(0, &data)?;
    Ok(rle)
}
