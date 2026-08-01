use crate::app;
use app::{ MangaApp, Screen };
use eframe::egui::Context;

pub fn show(ctx: &Context, app: &mut MangaApp) {
    egui::TopBottomPanel::top("reader_toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("< Home").clicked() {
                app.current_page = Screen::Home;
            }

            ui.separator();

            let title = app
                .current_manga
                .as_deref()
                .unwrap_or("No manga loaded");
                
            ui.heading(title);
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            // ui.image(&texture)
            ui.heading("ok");
        });
    });
}