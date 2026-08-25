use crate::app;
use crate::pages::library;
use app::{ MangaApp, Screen };
use eframe::egui::*;
use crate::app::ReaderState;
use crate::progress;

pub fn show(ctx: &Context, app: &mut MangaApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(250.0);

            ui.heading(
                egui::RichText::new(
                    "📚 Koma"
                )
                .size(40.0)
                .strong(),
            );

            ui.label("personal manga tracker and reader");

            ui.add_space(40.0);

            ui.set_width(250.0);

            if ui.add_sized([250.0, 45.0], Button::new("Continue Reading")).clicked() {
                let progress = progress::load()?;
                app.reader_state = ReaderState::new(progress.manga_path.to_string_lossy().to_string());
                app.reader_state.current_chapter = progress.chapter;
                app.reader_state.current_page = progress.page;
                app.reader_state.load_texture(ctx);
                app.current_page = Screen::Reader;
            }
            
            ui.add_space(10.0);

            if ui.add_sized([250.0, 45.0], Button::new("Library")).clicked() {
                app.library = library::scan_library("/home/dragon/.koma/Library");
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