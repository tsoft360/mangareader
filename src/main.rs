mod app;
mod pages;

use app::MangaApp;

fn main() -> eframe::Result<()> {
    let options = 
        eframe::NativeOptions::default();

    eframe::run_native(
        "Manga Reader",
        options,
        Box::new(|_| {
            Ok(Box::new(MangaApp::default()))
        }),
    )
}