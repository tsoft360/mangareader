use std::fs::File;
use std::path::{Path, PathBuf};

use quick_xml::events::Event;
use quick_xml::Reader;
use zip::ZipArchive;

pub struct EpubReader {
    pub path: PathBuf,
    pub pages: Vec<Vec<u8>>,
    pub current_page: usize,
}

impl EpubReader {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref().to_path_buf();

        let file = File::open(&path)?;
        let mut archive = ZipArchive::new(file)?;

        let opf_path = find_opf(&mut archive)?;

        println!("OPF: {}", opf_path);

        let spine = get_spine(&mut archive, &opf_path)?;

        println!("Spine:");

        for item in &spine {
            println!("  {}", item);
        }

        Ok(Self {
            path,
            pages: Vec::new(),
            current_page: 0,
        })
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
    }

    Err("Could not find OPF file".into())
}