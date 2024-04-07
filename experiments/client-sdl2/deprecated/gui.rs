use imgui;
use imgui::*;

use game::Game;

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

pub fn show_file_list(ui: &imgui::Ui) {
    ui.window(im_str!("RLE Files"))
        .size((300.0, 300.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            if ui.collapsing_header(im_str!("RLE")).build() {
                ui.tree_node(im_str!("Tree"))
                    .build(|| for i in 0..5 {
                        ui.tree_node(im_str!("Child {}", i))
                            .build(|| {
                                ui.text(im_str!("blah blah"));
                                ui.same_line(0.0);
                                if ui.small_button(im_str!("print")) {
                                    println!("Child {} pressed", i);
                                }
                            });
                    });
            };
        });
}

pub fn show_game_manager_states(ui: &imgui::Ui, game: &Game) {
    ui.window(im_str!("Game State"))
        .size((300.0, 300.0), ImGuiSetCond_FirstUseEver)
        .build( || {
            ui.text(im_str!("map_manager: objects: {}", game.map_manager.get_count()));
            ui.text(im_str!("data_manager: objects: {}", game.data_manager.get_count()));
            ui.text(im_str!("sprite_manager: objects: {}", game.sprite_manager.get_count()));
            ui.text(im_str!("list_manager: objects: {}", game.list_manager.get_count()));
    });
}