use std::path::{ Path, PathBuf };

pub struct Chapter {
    pub name: String,
    pub pages: Vec<PathBuf>,
    pub current_page: usize,
}

impl Chapter {
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let name = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let mut pages = Vec::new();

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let image_path = entry.path();

            if image_path.is_file() {
                pages.push(image_path);
            }
        }

        pages.sort_by(|a, b| a.to_string_lossy().to_string().cmp(&b.to_string_lossy().to_string()));

        let current_page = 0;

        Ok(Self {
            name,
            pages,
            current_page,
        })
    }
}

pub struct Manga {
    pub title: String,
    pub chapters: Vec<Chapter>,
    pub current_chapter: usize,
}

impl Manga {
    pub fn scan_manga_folder(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref();

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

        chapters.sort_by(|a, b| a.name.cmp(&b.name));

        let current_chapter = 0;

        Ok(Self {
            title,
            chapters,
            current_chapter,
        })
    }
}