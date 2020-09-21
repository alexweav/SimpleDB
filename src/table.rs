use crate::row;

const TABLE_MAX_PAGES: usize = 256;
const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / row::ROW_SIZE;

/// A table in the database.
pub struct Table {
    /// The number of rows in the table.
    pub num_rows: usize,
    /// An in-memory page table storing the data.
    /// Each element is a memory page, containing contiguously mapped and packed rows.
    pages: Vec<Vec<u8>>,
}

impl Table {
    /// Instantiates an empty table.
    pub fn new() -> Table {
        Table {
            num_rows: 0,
            pages: vec![Vec::new(); TABLE_MAX_PAGES],
        }
    }

    /// Gives a slice of the memory-mapped page table corresponding to a specific row.
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
