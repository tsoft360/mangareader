use eframe::egui;

use crate::pages;

pub enum Screen {
    Home, 
    Library,
    Reader,
    Settings,
}

pub struct MangaApp {
    pub current_page: Screen,
    pub current_manga: Option<String>,
}

impl Default for MangaApp {
    fn default() -> Self {
        Self {
            current_page: Screen::Home,
            current_manga: Option::None,
        }
    }
}

impl eframe::App for MangaApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame
    ) {
        pages::show(ctx, self);
    }
}