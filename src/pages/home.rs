use crate::app;
use app::{ MangaApp, Screen };
use eframe::egui::*;
use crate::app::ReaderState;

pub fn show(ctx: &Context, app: &mut MangaApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(250.0);

            ui.heading(
                egui::RichText::new(
                    "📚 Manga Reader"
                )
                .size(40.0)
                .strong(),
            );

            ui.label("personal manga tracker and reader");

            ui.add_space(40.0);

            ui.set_width(250.0);

            if ui.add_sized([250.0, 45.0], Button::new("Continue Reading")).clicked() {
                app.reader_state = ReaderState::new("/home/dragon/.local/share/komikku/mangadex/You Look Better Crying".to_string());
                app.reader_state.load_texture(ctx);
                app.current_page = Screen::Reader;
            }
            
            ui.add_space(10.0);

            if ui.add_sized([250.0, 45.0], Button::new("Library")).clicked() {
                app.current_page = Screen::Library;
            }

            ui.add_space(10.0);

            if ui.add_sized([250.0, 45.0], Button::new("Settings")).clicked() {
                app.current_page = Screen::Settings;
            }

            ui.add_space(10.0);

            if ui.add_sized([250.0, 45.0], Button::new("Quit")).clicked() {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        });
    });
}