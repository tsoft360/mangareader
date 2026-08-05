use crate::app;
use app::{ MangaApp, Screen };
use eframe::egui::Context;
use downcast_rs::Downcast;

pub fn show(ctx: &Context, app: &mut MangaApp) {
    let reader = app.reader_state.as_any().unwrap();

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
        let available = ui.available_size();

        let image_size = texture.size_vec2();

        let scale = (available.x / image_size.x)
            .min(available.y / image_size.y);

        let desired_size = image_size * scale;

        if let Some(texture) = &reader.texture {
            ui.centered_and_justified(|ui| {
                ui.add(
                    egui::Image::new(&texture)
                        .fit_to_exact_size(desired_size)
                );
            });
        }

        let scroll = ctx.input(|i| i.raw_scroll_delta.y);
        

        if scroll < 0.0 {
            reader.next_page();
        }
    });
}