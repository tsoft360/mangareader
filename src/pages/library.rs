use crate::app;
use app::{MangaApp, Screen, ReaderState};
use eframe::egui;
use std::path::{PathBuf, Path};
use std::fs::{self, File};
use sha2::{Digest, Sha256};
use std::io::{self, Read, Write};
use zip::ZipArchive;

pub enum BookType {
    Images,
    Epub,
    Unsure,
}

fn check_book_type(path: &Path) -> BookType {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                return BookType::Images;
            }

            if path.is_file()
                && path.extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("epub"))
            {
                return BookType::Epub;
            }
        }
    }

    BookType::Unsure
}

fn get_cover_path(entry: &Path, file_type: &BookType) -> Option<PathBuf> {
    match file_type {
        BookType::Images => {
            let cover = entry.join("cover.jpg");

            if cover.exists() {
                Some(cover)
            } else {
                None
            }
        },

        BookType::Epub => {
            let cover = extract_cover_image(entry);
            Some(cover.expect("51,13 library.rs"))
        },

        BookType::Unsure => None,
    }
}

fn extract_cover_image(epub_path: &Path) -> io::Result<PathBuf> {
    let cache_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("koma")
        .join("covers");

    fs::create_dir_all(&cache_dir);

    let mut hasher = Sha256::new();
    hasher.update(epub_path.to_string_lossy().as_bytes());

    let hash = format!("{:x}", hasher.finalize());

    let cache_path = cache_dir.join(format!("{}.jpg", hash));

    if cache_path.exists() {
        return Ok(cache_path);
    }

    let file = File::open(epub_path)?;
    let mut archive = ZipArchive::new(file)?;

    let mut cover_index = None;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = file.name().to_lowercase();

        if name.contains("cover.")
            && (
                name.ends_with(".jpg")
                    || name.ends_with(".jpeg")
                    || name.ends_with(".png")
                    || name.ends_with(".webp")
            )
        {
            cover_index = Some(i);
            break;
        }
    }

    let index = cover_index.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Could not find cover image in EPUB",
        )
    })?;

    let mut cover = archive.by_index(index)?;

    let mut data = Vec::new();
    cover.read_to_end(&mut data)?;

    let mut output = File::create(&cache_path)?;
    output.write_all(&data)?;

    println!("{}", cache_path.to_string_lossy().to_string());

    Ok(cache_path)
}


pub struct LibraryEntry {
    pub title: String,
    pub path: PathBuf,
    pub cover_path: PathBuf,
    pub cover: Option<egui::TextureHandle>,
    pub file_type: BookType,
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

        let file_type = check_book_type(&entry.path().to_path_buf());

        let cover_path = get_cover_path(&entry.path(), &file_type);

        library.push(LibraryEntry {
            title,
            cover_path: cover_path.expect("174,14 library.rs"),
            path: entry.path(),
            cover: None,
            file_type,
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
                    let response = ui.add(
                        egui::Button::image(
                            egui::Image::new(texture)
                                .fit_to_exact_size(egui::vec2(150.0, 225.0)),
                        )
                    );

                    if response.clicked() {
                        app.reader_state = ReaderState::new(manga.path.to_string_lossy().to_string(), 0, 0);
                        app.current_page = Screen::Reader;
                        app.reader_state.load_texture(ctx);
                    }
                }


                ui.label(&manga.title);
                ui.end_row();
            }
        });
    });
}