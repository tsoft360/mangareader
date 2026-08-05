use crate::app;
use crate::reader::folder_reader;
use app::{ MangaApp, Screen };
use eframe::egui::Context;
use image::ImageReader;

pub fn show(ctx: &Context, app: &mut MangaApp) {
    let reader = app.reader_state.as_mut().unwrap();

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
        let mut manga = &reader.manga;
        let mut chapter = &manga.chapters[manga.current_chapter];
        let mut path = chapter.pages[chapter.current_page].to_string_lossy().to_string();


        match ImageReader::open(path)
            .unwrap()
            .decode()
        {
            Ok(image) => {

                let rgba = image.to_rgba8();

                let size = [
                    rgba.width() as usize,
                    rgba.height() as usize,
                ];

                let color_image =
                    egui::ColorImage::from_rgba_unmultiplied(
                        size,
                        &rgba,
                    );

                let texture =
                    ctx.load_texture(
                        "manga_page",
                        color_image,
                        egui::TextureOptions::default(),
                    );

                let available = ui.available_size();

                let image_size = texture.size_vec2();

                let scale = (available.x / image_size.x)
                    .min(available.y / image_size.y);

                let desired_size = image_size * scale;
                
                ui.centered_and_justified(|ui| {
                    ui.add(
                        egui::Image::new(&texture)
                            .fit_to_exact_size(desired_size)
                    );
                });
            }

            Err(e) => {
                ui.label(format!("Error: {}", e));
            }
        }

    });
}