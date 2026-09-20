use lib_epub::epub::EpubDoc;
use std::path::{Path, PathBuf};

pub struct EpubReader {
    pub path: PathBuf,
    pub epub: EpubDoc<std::io::BufReader<std::fs::File>>,
    pub title: String,

    pub current_chapter: usize,
    pub chapter_content: String,
}

impl EpubReader {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref().to_path_buf();
        let mut epub;
        let mut chapter_content;

        if *path != *"" {
            epub = EpubDoc::new(&path)?;
            chapter_content = match epub.spine_current() {
                Some((content, _mime)) => String::from_utf8(content)?,
                None => String::new(),
            };
        } else {
            epub = EpubDoc::new("")?;
            chapter_content = String::new();
        }

        Ok(Self {
            path,
            epub,
            title: "".to_string(),
            current_chapter: 0,
            chapter_content,
        })
    }

    pub fn next_chapter(&mut self) -> bool {
        if let Some((content, _mime)) = self.epub.spine_next() {
            self.current_chapter += 1;

            self.chapter_content = 
                String::from_utf8(content).unwrap_or_default();

            true
        } else {
            false
        }
    }

    pub fn previous_chapter(&mut self) -> bool {
        if self.current_chapter == 0 {
            return false;
        }

        if let Some((content, _mime)) = self.epub.spine_prev() {
            self.current_chapter -= 1;

            self.chapter_content = 
                String::from_utf8(content).unwrap_or_default();

            true
        } else {
            false
        }
    }
}