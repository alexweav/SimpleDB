use crate::cursor;
use crate::pager;
use crate::row;

const ROWS_PER_PAGE: usize = pager::PAGE_SIZE / row::ROW_SIZE;

/// A table in the database.
pub struct Table {
    /// The number of rows in the table.
    pub num_rows: usize,
    pager: pager::Pager,
}

impl Table {
    /// Instantiates an empty table.
    pub fn new(filename: &str) -> Table {
        let pager = pager::Pager::open(filename).unwrap();
        let num_rows = pager.file_length / row::ROW_SIZE;
        Table {
            num_rows: num_rows,
            pager: pager,
        }
    }

    /// Gives a slice of the memory-mapped page table corresponding to a cursor.
    pub fn cursor_value(&mut self, cursor: &cursor::Cursor) -> &mut [u8] {
        let row_num = cursor.row_num;
        let page_num = row_num / ROWS_PER_PAGE;
        let page = self.pager.get_page(page_num);
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * row::ROW_SIZE;
        &mut page[byte_offset..]
    }

    pub fn flush(&mut self) {
        for page_idx in 0..self.pager.get_num_pages() {
            self.pager.flush(page_idx);
        }
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        self.flush();
    }
}
