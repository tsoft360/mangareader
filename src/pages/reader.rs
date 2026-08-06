use crate::app;
use app::{ MangaApp, Screen };
use eframe::egui::Context;

pub fn show(ctx: &Context, app: &mut MangaApp) {
    let reader = &mut app.reader_state;

    egui::TopBottomPanel::top("reader_toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("< Home").clicked() {
                app.current_page = Screen::Home;
            }

            ui.separator();

            let title = reader.manga.title.clone();
                
            ui.heading(title);
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

        let scroll = ctx.input(|i| i.raw_scroll_delta.y);
        

        if scroll < 0.0 {
            reader.next_page();
            reader.load_texture(ctx);
        }

        if scroll > 0.0 {
            reader.previous_page();
            reader.load_texture(ctx);
        }
    });
}