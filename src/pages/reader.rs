use crate::app;
use app::{ MangaApp, Screen, ReaderState };
use eframe::egui::Context;

pub fn show(ctx: &Context, app: &mut MangaApp) {
    let reader: &mut ReaderState = &mut app.reader_state;
    // let reader: &mut ReaderState = &mut app.reader_state;
    let scroll = ctx.input(|i| i.raw_scroll_delta.y); 

    if scroll < 0.0 {
        reader.next_page(ctx);
    }

    if scroll > 0.0 {
        reader.previous_page(ctx);
    }

    egui::TopBottomPanel::top("reader_toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("< Home").clicked() {
                app.current_page = Screen::Home;
                reader.save_progress();
            }

            ui.separator();

            let manga_title = reader.manga.title.clone();
            let chapter_title = reader.manga.chapters[reader.current_chapter].name.clone();
                
            ui.heading(format!("{manga_title} - {chapter_title}"));
        });
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        if let Some(texture) = &reader.texture {
            let available = ui.available_size();
            let image_size = texture.size_vec2();
            let scale = (available.x / image_size.x)
                .min(available.y / image_size.y);

            let desired_size = image_size * scale;
            ui.centered_and_justified(|ui| {
                ui.add(
                    egui::Image::new(texture)
                        .fit_to_exact_size(desired_size)
                );
            });
        }
    });
}