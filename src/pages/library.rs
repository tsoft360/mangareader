use crate::app;
use app::{MangaApp, Screen};
use eframe::egui;
use std::path::{PathBuf, Path};

pub struct LibraryEntry {
    pub title: String,
    pub path: PathBuf,
    pub cover_path: PathBuf,
    pub cover: Option<egui::TextureHandle>,
} 

impl LibraryEntry {
    pub fn load_cover(&mut self, ctx: &egui::Context) -> image::ImageResult<()> {
        if self.cover.is_some() {
            return Ok(());
        }

        let image = image::open(&self.cover_path)?;
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

        self.cover = Some(
            ctx.load_texture(
                &self.title,
                color_image,
                Default::default(),
            )
        );

        Ok(())
    }
}

pub fn scan_library(path: impl AsRef<Path>) -> Vec<LibraryEntry> {
    let mut library = Vec::new();

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();

        if !entry.path().is_dir() {
            continue;
        }

        let title = entry.file_name().to_string_lossy().to_string();

        library.push(LibraryEntry {
            title,
            cover_path: entry.path().join("cover.jpg"),
            path: entry.path(),
            cover: None,
        });
    }

    library
}

pub fn show(ctx: &egui::Context, app: &mut MangaApp) {
    egui::TopBottomPanel::top("library toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("< Home").clicked() {
                app.current_page = Screen::Home;
            }
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::Grid::new("library").show(ui, |ui| {
            for manga in &mut app.library {
                manga.load_cover(ctx).ok();

                if let Some(texture) = &manga.cover {
                    ui.add(
                        egui::Image::new(texture)
                            .fit_to_exact_size(egui::vec2(150.0, 225.0)),
                    );
                }

                ui.label(&manga.title);
                ui.end_row();
            }
        });
    });
}