use std::path::{ Path, PathBuf };
use std::fs;
use serde::Deserialize;
use std::io::Result;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub komikku_id: String,
    pub number: String,
    pub title: String,
}

pub fn load_metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    let json = fs::read_to_string(path)?;
    let metadata: Metadata = serde_json::from_str(&json)?;
    Ok(metadata)
}

fn is_image(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("jpg" | "jpeg" | "png" | "webp")
    )
}

pub struct Chapter {
    pub name: String,
    pub pages: Vec<PathBuf>,
}

impl Chapter {
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let metadata_path = path.join("chapter.json");
        let metadata = load_metadata(metadata_path)?;
        let name = metadata.title.clone();

        let mut pages = Vec::new();

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let image_path = entry.path();

            if image_path.is_file() && is_image(&image_path) {
                pages.push(image_path);
            }
        }

        pages.sort_by(|a, b| {
            let a = a.file_stem().unwrap().to_str().unwrap();
            let b = b.file_stem().unwrap().to_str().unwrap();

            natord::compare(a, b)
        });

        Ok(Self {
            name,
            pages,
        })
    }
}

pub struct Manga {
    pub title: String,
    pub chapters: Vec<Chapter>,
}

impl Manga {
    pub fn scan_manga_folder(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref();

        if path.file_name() == Option::None {
            Ok(Self {
                title: "".to_string(),
                chapters: Vec::new(),
            })
        } else {
            let title = path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            let mut chapters = Vec::new();

            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let chapter_path = entry.path();

                if !chapter_path.is_dir() {
                    continue;
                }

                let chapter = Chapter::load(&chapter_path)?;
                chapters.push(chapter);
            }

            chapters.sort_by(|a, b| {
                let a = &a.name;
                let b = &b.name;

                natord::compare(&a, &b)
            });

            Ok(Self {
                title,
                chapters,
            })
        }
    }
}