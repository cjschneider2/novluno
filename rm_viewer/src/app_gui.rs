use std::path::PathBuf;
use std::fs::read_dir;

use gtk::*;

use app_error::AppError;
use app_data::AppData;

pub struct AppGui {
    pub main_window: Window,
    pub image: Image,
    pub status_bar: Statusbar,
    pub file_list_store: ListStore,
    pub file_tree_view: TreeView,
    pub file_tree_selection: TreeSelection,
    pub open_folder_button: Button,
    pub file_chooser_dialog: FileChooserDialog,
}

impl AppGui {
    pub fn new(builder: &Builder) -> Result<AppGui, AppError> {
        let err = || { return Err(AppError::Str("Builder error"));};

        let main_window = builder.get_object("MainWindow").unwrap();
        let image = builder.get_object("Image").unwrap();
        let file_list_store = builder.get_object("FileListStore").unwrap();
        let file_tree_view = builder.get_object("FileTreeView").unwrap();
        let file_tree_selection = builder.get_object("FileTreeSelection").unwrap();
        let open_folder_button = builder.get_object("OpenFolderButton").unwrap();

        let app_gui: AppGui = AppGui {
            main_window,
            image,
            status_bar,
            file_list_store,
            file_tree_view,
            file_tree_selection,
            open_folder_button,
            file_chooser_dialog: {
                let dialog = FileChooserDialog::new(
                    Some("Choose a folder:"),
                    Some(&Window::new(WindowType::Popup)),
                    FileChooserAction::SelectFolder);
                dialog.add_button("Cancel", ResponseType::Cancel.into());
                dialog.add_button("Select", ResponseType::Ok.into());
                dialog
            },
        };

        app_gui.file_tree_view.set_headers_visible(false);
        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
        app_gui.file_tree_view.append_column(&column);

        Ok(app_gui)
    }

    pub fn update_status(&self, app_data: &AppData) {
        const MSG_ID: i32 = 42;
        let status = &self.status_bar;
        status.remove_all(42);
        let status_str =
            &format!(" Resource: [{}/{}], W: {} H: {} X: {} Y:{}",
                     app_data.resource_idx + 1,
                     app_data.resource_total,
                     app_data.width,
                     app_data.height,
                     app_data.offset_x,
                     app_data.offset_y);
        status.push(42, status_str);
    }

    pub fn load_new_folder(&self, app_data: &AppData) -> Result<(), AppError> {
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
