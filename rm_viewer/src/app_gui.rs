use std::path::PathBuf;
use std::fs::read_dir;

use gtk::*;

use app_error::AppError;
use app_data::AppData;

pub struct AppGui {
    pub main_window: Window,
    pub status_bar: Statusbar,
    pub list_store: ListStore,
    pub tree_view: TreeView,
    pub tree_selection: TreeSelection,
    pub notebook: Notebook,
    pub open_folder_button: Button,
    pub settings_button: Button,
    pub file_chooser_dialog: FileChooserDialog,
}

impl AppGui {
    pub fn new(builder: &Builder) -> Result<AppGui, AppError> {
        // get builder objects
        let main_window = builder.get_object("MainWindow").unwrap();
        let status_bar = builder.get_object("StatusBar").unwrap();
        let notebook = builder.get_object("Notebook").unwrap();
        let list_store = builder.get_object("TreeListStore").unwrap();
        let tree_view = builder.get_object("TreeView").unwrap();
        let tree_selection = builder.get_object("TreeSelection").unwrap();
        let open_folder_button = builder.get_object("LoadDataDirButton").unwrap();
        let settings_button = builder.get_object("SettingsButton").unwrap();
        let welcome_text: TextView = builder.get_object("WelcomeText").unwrap();
        // create gui object
        let app_gui: AppGui = AppGui {
            main_window,
            notebook,
            status_bar,
            list_store,
            tree_view,
            tree_selection,
            open_folder_button,
            settings_button,
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
        // GUI configuration
        // -- Config tree-view
        app_gui.tree_view.set_headers_visible(false);
        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
        app_gui.tree_view.append_column(&column);
        // -- Config Notebook
        let label = Label::new(Some("Welcome!"));
        app_gui.notebook.append_page(&welcome_text, Some(&label));
        // return
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
            let next = self.list_store.append();
            if let Some(name) = file.file_name() {
                let name: &str = match name.to_str() {
                    Some(name) => name,
                    None => return Err(AppError::StringConversion),
                };
                self.list_store.set_value(&next, 0, &name.to_value());
            }
        }
        Ok(())
    }
}
