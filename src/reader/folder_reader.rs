use std::path::PathBuf;

let manga_folder = PathBuf::from("/home/dragon/Documents/books/manga")

pub struct Chapter {
    pub name: String,
    pub pages: Vec<PathBuf>,
}

impl Chapter {
    pub fn load(&self) {
        
    }
}

pub struct Manga {
    pub title: String,
    pub chapters: Vec<Chapter>,
}

impl Manga {
    pub fn load(&self) {

    }
}

pub struct FolderReader {
    pub manga: Manga,

    pub current_chapter: usize,
    pub current_page: usize,
}

impl FolderReader {
    pub fn current_page_path(&self) -> Option<&Path> {
        self.manga
            .chapters
            .get(self.current_chapter)?
            .pages
            .get(self.current_page)
            .map(|p| p.as_path())
    }
}