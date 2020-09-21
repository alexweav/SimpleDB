use crate::row;

const TABLE_MAX_PAGES: usize = 256;
const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / row::ROW_SIZE;

pub struct Table {
    pub num_rows: usize,
    pages: Vec<Vec<u8>>
}

impl Table {
    pub fn new() -> Table {
        Table {
            num_rows: 0,
            pages: vec![Vec::new(); TABLE_MAX_PAGES],
        }
    }

    pub fn row_slot(&mut self, row_num: usize) -> &mut [u8] {
        let page_num = row_num / ROWS_PER_PAGE;
        if self.pages[page_num].is_empty() {
            self.pages[page_num] = vec![0; PAGE_SIZE];
        }
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * row::ROW_SIZE;
        &mut self.pages[page_num][byte_offset..]
    }
}
