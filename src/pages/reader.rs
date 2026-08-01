use crate::app;
use app::MangaApp;
use eframe::egui::Context;

pub fn show(ctx: &Context, _app: &mut MangaApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        
    });
}