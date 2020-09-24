use std::fs;
use std::io;
use std::io::prelude::*;

pub const PAGE_SIZE: usize = 4096;
pub const TABLE_MAX_PAGES: usize = 256;

pub struct Pager {
    file: fs::File,
    pub file_length: usize,
    /// An in-memory page table storing the data.
    /// Each element is a memory page, containing contiguously mapped and packed rows.
    pages: Vec<Vec<u8>>,
}

impl Pager {
    pub fn open(filename: &str) -> std::io::Result<Pager> {
        let file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(filename)?;
        let length = file.metadata()?.len();
        Ok(Pager {
            file: file,
            file_length: length as usize,
            pages: vec![Vec::new(); TABLE_MAX_PAGES],
        })
    }

    /// Reads a page with in-memory caching. The cached page is never flushed.
    pub fn get_page(&mut self, page_num: usize) -> &mut Vec<u8> {
        // TODO: if page_num > TABLE_MAX_PAGES, panic or error
        if self.pages[page_num].is_empty() {
            // Cache miss.
            self.pages[page_num] = vec![0; PAGE_SIZE];
            let num_total_pages = {
                let mut pages = self.file_length / PAGE_SIZE;

                // We might save a partial page at the end of the file.
                if self.file_length % PAGE_SIZE != 0 {
                    pages += 1;
                }
                pages
            };

            if page_num <= num_total_pages {
                let page_start = page_num * PAGE_SIZE;
                self.file
                    .seek(io::SeekFrom::Start(page_start as u64))
                    .unwrap();
                self.file.read(&mut self.pages[page_num]).unwrap();
            }
        }

        &mut self.pages[page_num]
    }
}
