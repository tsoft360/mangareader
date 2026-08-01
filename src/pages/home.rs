use crate::app;
use app::MangaApp;
use eframe::egui::Context;

pub fn show(ctx: &Context, _app: &mut MangaApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {

            ui.heading(
                egui::RichText::new(
                    "📚 Manga Reader"
                )
                .size(40.0)
            );

            ui.add_space(20.0);

            if ui.button("Continue Reading").clicked() {
                app.page = Page::Reader;
            }

            if ui.button("Library").clicked() {
                app.page = Page::Library;
            }

            if ui.button("Open Folder").clicked() {
                // Open folder dialog later
            }

            if ui.button("Settings").clicked() {
                app.page = Page::Settings;
            }
        });
    });
}