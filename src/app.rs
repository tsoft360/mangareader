use eframe::egui;

use crate::pages;
use crate::reader::folder_reader::Manga;
use std::io::Result;

pub enum Screen {
    Home, 
    Library,
    Reader,
    Settings,
}

pub struct ReaderState {
    pub manga: Manga,
    pub current_chapter: usize,
    pub current_page: usize,
}

impl ReaderState {
    pub fn new(path: String) -> Result<Self> {
        Ok(Self {
            manga: Manga::scan_manga_folder(path)?,
            current_chapter: 0,
            current_page: 0,
        })
    }
}

pub struct MangaApp {
    pub current_page: Screen,
    pub current_manga: Option<String>,
    pub reader_state: Option<ReaderState>,
}

impl Default for MangaApp {
    fn default() -> Self {
        Self {
            current_page: Screen::Home,
            current_manga: Option::None,
            reader_state: Option::None,
        }
    }
}

impl eframe::App for MangaApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame
    ) {
        egui_extras::install_image_loaders(ctx);
        pages::show(ctx, self);
    }
}