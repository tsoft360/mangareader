use crate::app;
use app::{MangaApp, Screen};
use eframe::egui::Context;

pub fn show(ctx: &Context, app: &mut MangaApp) {
    let reader = &app.epub_reader;

    egui::TopBottomPanel::top("epub_reader_toolbar").show(ctx, |ui| {
        if ui.button("< Home").clicked() {
            app.current_page = Screen::Home;
            // save progress
        }

        ui.separator();
    });
}