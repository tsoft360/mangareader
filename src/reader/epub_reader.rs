use std::fs::File;
use std::path::{Path, PathBuf};

use scraper::{Html, Selector};

use quick_xml::events::Event;
use quick_xml::Reader;
use zip::ZipArchive;

fn html_to_text(html: &str) -> String {
    let document = Html::parse_document(html);
    let selector = Selector::parse("body").unwrap();

    let mut text = String::new();

    for body in document.select(&selector) {
        text.push_str(&body.text().collect::<Vec<_>>().join(" "));
    }

    text
}

pub struct EpubReader {
    pub path: PathBuf,
    pub chapters: Vec<String>,
    pub current_page: usize,
}

impl EpubReader {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref().to_path_buf();

        let file = File::open(&path)?;
        let mut archive = ZipArchive::new(file)?;

        let mut chapters = Vec::new();

        for i in 0..archive.len() {
            let file = archive.by_index(i)?;

            let name = file.name().to_string();

            if name.ends_with(".xhtml") || name.ends_with(".html") {
                chapters.push(name);
            }
        }

        println!("Found {} XHTML files", chapters.len());

        Ok(Self {
            path,
            chapters,
            current_page: 0,
        })
    }

    pub fn load_chapter(
        &self,
        chapter: usize,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let file = File::open(&self.path)?;
        let mut archive = ZipArchive::new(file)?;

        let chapter_path = &self.chapters[chapter];

        let mut chapter_file = archive.by_name(chapter_path)?;

        let mut html = String::new();

        std::io::Read::read_to_string(
            &mut chapter_file,
            &mut html,
        )?;

        Ok(html_to_text(&html))
    }

    fn find_opf(
        archive: &mut ZipArchive<File>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut container = archive.by_name("META-INF/container.xml")?;

        let mut xml = String::new();
        std::io::Read::read_to_string(&mut container, &mut xml)?;

        let mut reader = Reader::from_str(&xml);

        loop {
            match reader.read_event()? {
                Event::Empty(e) | Event::Start(e) => {
                    if e.name().as_ref() == b"rootfile" {
                        for attribute in e.attributes() {
                            let attribute = attribute?;

                            if attribute.key.as_ref() == b"full-path" {
                                return Ok(
                                    String::from_utf8(attribute.value.to_vec())?
                                );
                            }
                        }
                    }
                }

                Event::Eof => break,

                _ => {}
            }
        }

        Err("Could not find OPF file".into())
    }

    fn get_spine(
        archive: &mut ZipArchive<File>,
        opf_path: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut opf = archive.by_name(opf_path)?;

        let mut xml = String::new();
        std::io::Read::read_to_string(&mut opf, &mut xml)?;

        let mut reader = Reader::from_str(&xml);

        let mut manifest = std::collections::HashMap::new();
        let mut spine = Vec::new();

        loop {
            match reader.read_event()? {
                Event::Empty(e) | Event::Start(e) => {
                    match e.name().as_ref() {
                        b"item" => {
                            let mut id = None;
                            let mut href = None;

                            for attribute in e.attributes() {
                                let attribute = attribute?;

                                match attribute.key.as_ref() {
                                    b"id" => {
                                        id = Some(
                                            String::from_utf8(
                                                attribute.value.to_vec()
                                            )?
                                        );
                                    }

                                    b"href" => {
                                        href = Some(
                                            String::from_utf8(
                                                attribute.value.to_vec()
                                            )?
                                        );
                                    }

                                    _ => {}
                                }
                            }

                            if let (Some(id), Some(href)) = (id, href) {
                                manifest.insert(id, href);
                            }
                        }

                        b"itemref" => {
                            for attribute in e.attributes() {
                                let attribute = attribute?;

                                if attribute.key.as_ref() == b"idref" {
                                    let id = String::from_utf8(
                                        attribute.value.to_vec()
                                    )?;

                                    if let Some(href) = manifest.get(&id) {
                                        spine.push(href.clone());
                                    }
                                }        
                            }
                        }

                        _ => {}
                    }
                }

                Event::Eof => break,

                _ => {}
            }
        }

        Ok(spine)
    }
}