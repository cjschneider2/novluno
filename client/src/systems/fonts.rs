use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

// Runs each EguiPrimaryContextPass frame until the context is available,
// then registers fonts once and stops.
pub fn setup_fonts(mut contexts: EguiContexts, mut done: Local<bool>) {
    if *done {
        return;
    }

    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "NotoSans".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/NotoSans-VariableFont_wdth,wght.ttf"
        ))
        .into(),
    );
    fonts.font_data.insert(
        "NotoSansKR".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/NotoSansKR-VariableFont_wght.ttf"
        ))
        .into(),
    );

    // NotoSans first for Latin, NotoSansKR as fallback for Korean glyphs.
    let proportional = fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default();
    proportional.insert(0, "NotoSans".to_owned());
    proportional.push("NotoSansKR".to_owned());

    match contexts.ctx_mut() {
        Ok(ctx) => {
            ctx.set_fonts(fonts);
            info!("setup_fonts: NotoSans + NotoSansKR registered successfully");
            *done = true;
        }
        Err(e) => {
            warn!("setup_fonts: egui context not ready ({e:?}), will retry next frame");
        }
    }
}
