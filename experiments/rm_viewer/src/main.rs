#![allow(dead_code, unused_extern_crates)]

extern crate core_compat;
extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;

mod constants;
mod app_data;
mod app_gui;
mod app_error;
mod read_rle;

use std::cell::RefCell;
use std::rc::Rc;
use std::path::PathBuf;

use gtk::*;
// use gdk::prelude::*;
// use gdk::enums::key;

use app_data::AppData;
use app_gui::AppGui;

// include the UI file
const RLE_VIEWER_UI: &'static str = include_str!("rle_viewer.ui");

fn main() {

    // init gtk
    gtk::init().expect("Failed to initialize GTK");

    // Window && widget creation
    let builder = Builder::new_from_string(RLE_VIEWER_UI);

    // setup application gui
    let app_gui = AppGui::new(&builder).unwrap();

    // Setup application data
    let mut app_data = AppData::new();
    app_data.current_path.push("~/redmoon_data/RLEs/Obj");
    let _ = app_gui.load_new_folder(&app_data);

    let app_gui = Rc::new(RefCell::new(app_gui));
    let app_data = Rc::new(RefCell::new(app_data));

    // Update initial widget info
    app_gui.borrow().update_status(&app_data.borrow());

    // show all widgets
    app_gui.borrow().main_window.show_all();

    // close window event
    app_gui.borrow().main_window.connect_delete_event( |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Key press events
    app_gui.borrow().main_window.connect_key_press_event({
        let _app_gui = app_gui.clone();
        let _app_data = app_data.clone();
        move |_widget, key| {
            let idx = _app_data.borrow().resource_idx as isize;
            let total = _app_data.borrow().resource_total as isize;
            let next = idx + 1;
            let prev = idx - 1;
            match key.get_keyval() {
                Right => if next < total {
                    _app_data.borrow_mut().load_rle_at_idx(next as usize);
                },
                Left  => if prev >= 0    {
                    _app_data.borrow_mut().load_rle_at_idx(prev as usize);
                },
                _ => {}
            };
            _app_gui.borrow().update_status(&_app_data.borrow());
            Inhibit(false)
        }
    });

    // Image draw event
    app_gui.borrow().image.connect_draw({
        let _app_data = app_data.clone();
        move |_widget, canvas| {
            let __app_data = _app_data.borrow();
            if let Some(ref pixbuf) = __app_data.pixbuf {
                let x = __app_data.offset_x as f64;
                let y = __app_data.offset_y as f64;
                canvas.set_source_pixbuf(&pixbuf, x, y);
                canvas.paint();
            }
            Inhibit(false)
        }
    });

    // load folder button pressed
    app_gui.borrow().open_folder_button.connect_clicked({
        let _app_gui = app_gui.clone();
        let _app_data = app_data.clone();
        move |_widget| {
            let __app_gui = _app_gui.borrow();
            __app_gui.tree_selection.unselect_all();
            __app_gui.list_store.clear();
            let mut path: Option<PathBuf> = None;
            if __app_gui.file_chooser_dialog.run() == ResponseType::Ok.into() {
                if let Some(folder) = __app_gui.file_chooser_dialog
                    .get_current_folder() {
                        path = Some(folder);
                    }
            }
            if let Some(p) = path {
                _app_data.borrow_mut().current_path = p;
                let _result = __app_gui.load_new_folder(&_app_data.borrow());
            }
            __app_gui.file_chooser_dialog.hide();
        }
    });

    app_gui.borrow().file_tree_selection.connect_changed({
        let _app_gui = app_gui.clone();
        let _app_data = app_data.clone();
        move |_widget| {
            let mut file = _app_data.borrow().current_path.clone();
            let ref selection = _app_gui.borrow().file_tree_selection;
            if let Some((model, iter)) = selection.get_selected() {
                file.push(model.get_value(&iter, 0).get::<&str>().unwrap());
            }
            match read_rle_from_file(&file) {
                Ok(rle) => {
                    _app_data.borrow_mut().resource_total = rle.resources.len();
                    _app_data.borrow_mut().resource_idx = 0;
                    _app_data.borrow_mut().current_file = file.clone();
                    _app_data.borrow_mut().resource_file = Some(rle)
                },
                Err(e)  => {
                     println!("{:?}", e);
                     _app_data.borrow_mut().resource_file = None;
                },
            }
        }
    });

    // hand over main loop to GTK
    gtk::main();

}
