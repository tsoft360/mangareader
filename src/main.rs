mod app;
mod pages;

use app::MangaApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 800.0])
            .with_title("Manga Reader"),
        ..Default::default()
    };

    eframe::run_native(
        "Manga Reader",
        options,
        Box::new(|_| Ok(Box::new(MangaApp::default()))),
    )
}