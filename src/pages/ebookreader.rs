use crate::app;
use app::{MangaApp, Screen};
use eframe::egui::Context;

pub fn show(ctx: &Context, app: &mut MangaApp) {
    let reader = &mut app.epub_reader;

    let scroll = ctx.input(|i| i.raw_scroll_delta.y);

    if scroll < 0.0 {
        reader.next_chapter();
    }

    if scroll > 0.0 {
        reader.previous_chapter();
    }

    egui::TopBottomPanel::top("epub_reader_toolbar").show(ctx, |ui| {
        if ui.button("< Home").clicked() {
            app.current_page = Screen::Home;
            // save progress
        }

        ui.separator();
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(reader.chapter_content.clone());
    });
}