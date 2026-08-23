use eframe::egui;

use crate::pages;
use crate::reader::folder_reader::Manga;
use crate::pages::library::LibraryEntry;
use image::{ImageReader};

pub enum Screen {
    Home, 
    Library,
    Reader,
    Settings,
}

pub struct ReaderState {
    pub manga: Manga,
    pub path: String,
    pub current_chapter: usize,
    pub current_page: usize,
    pub texture: Option<egui::TextureHandle>,
}

impl ReaderState {
    pub fn new(path: String) -> Self {
        let manga = Manga::scan_manga_folder(path).expect("ok");
        if manga.title != "" {
            let path = manga.chapters[0].pages[0].to_string_lossy().to_string();
            Self {
                manga,
                path,
                current_chapter: 0,
                current_page: 0,
                texture: Option::None,
            }
        } else {
            let path = "".to_string();
            Self {
                manga,
                path,
                current_chapter: 0,
                current_page: 0,
                texture: Option::None,
            }
        }
    }

    pub fn next_page(&mut self) {
        self.current_page += 1;
        if self.current_page >= self.manga.chapters[self.current_chapter].pages.len() {
            self.next_chapter();
        } else {
            self.path = self.manga.chapters[self.current_chapter].pages[self.current_page].to_string_lossy().to_string();
        }
    }

    pub fn previous_page(&mut self) {
        if self.current_page <= 0 {
            self.previous_chapter();
        } else {
            self.current_page -= 1;
            self.path = self.manga.chapters[self.current_chapter].pages[self.current_page].to_string_lossy().to_string();
        }
    }

    pub fn next_chapter(&mut self) {
        self.current_chapter += 1;
        self.current_page = 0;
        self.path = self.manga.chapters[self.current_chapter].pages[self.current_page].to_string_lossy().to_string();
    }

    pub fn previous_chapter(&mut self) {
        if self.current_chapter != 0 {
            self.current_chapter -= 1;
            self.current_page = self.manga.chapters[self.current_chapter].pages.len() - 1;
            self.path = self.manga.chapters[self.current_chapter].pages[self.current_page].to_string_lossy().to_string();
        }
    }

    pub fn load_texture(&mut self, ctx: &egui::Context) {
        let image = ImageReader::open(self.path.clone()).unwrap().decode();

        let rgba = image.unwrap().to_rgba8();

        let size = [
            rgba.width() as usize,
            rgba.height() as usize,
        ];

        let color_image =
            egui::ColorImage::from_rgba_unmultiplied(
                size,
                &rgba,
            );

        self.texture =
            Some(ctx.load_texture(
                "manga_page",
                color_image,
                egui::TextureOptions::default(),
            ));
    }
}

pub struct MangaApp {
    pub current_page: Screen,
    pub reader_state: ReaderState,
    pub library: Vec<LibraryEntry>,
}

impl Default for MangaApp {
    fn default() -> Self {
        Self {
            current_page: Screen::Home,
            reader_state: ReaderState::new("".to_string()),
            library: Vec::new(),
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