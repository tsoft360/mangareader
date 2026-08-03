use crate::app;
use app::{ MangaApp, Screen };
use eframe::egui::Context;

pub fn load_texture(
    ctx: &Context,
    path: &std::path::Path,
) -> Result<egui::TextureHandle, image::ImageError> {
    let image = image::open(path);
    let rgba = image.to_rgba8();
    let size = [
        rgba.width() as usize,
        rgba.height() as usize,
    ];

    let pixels = rgba.into_raw();
    let color_image = 
        egui::ColorImage::from_rgba_unmultiplied(
            size,
            &pixels,
        );

    return Ok(
        ctx.load_texture(
            path.display().to_string(),
            color_image,
            Default::default().
        );
    );
}

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