use imgui;
use imgui::*;

pub fn show_gui_test(ui: &imgui::Ui) {
    ui.window(im_str!("Novluno Test Window"))
        .size((300.0, 300.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text(im_str!("please ignore"));
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!("Mouse Position: ({:.2}, {:.2})", mouse_pos.0, mouse_pos.1));
        });
}