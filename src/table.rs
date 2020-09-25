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
        /*Table {
            num_rows: 0,
            pages: vec![Vec::new(); pager::TABLE_MAX_PAGES],
        }*/
        let pager = pager::Pager::open(filename).unwrap();
        let num_rows = pager.file_length / row::ROW_SIZE;
        Table {
            num_rows: num_rows,
            pager: pager,
        }
    }

    /// Gives a slice of the memory-mapped page table corresponding to a specific row.
    pub fn row_slot(&mut self, row_num: usize) -> &mut [u8] {
        let page_num = row_num / ROWS_PER_PAGE;
        /*if self.pages[page_num].is_empty() {
            self.pages[page_num] = vec![0; PAGE_SIZE];
        }*/
        let page = self.pager.get_page(page_num);
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * row::ROW_SIZE;
        &mut page[byte_offset..]
    }
}

/*impl Drop for Table {
    fn drop (&mut self) {
        let num_full_pages = self.num_rows / ROWS_PER_PAGE;
    }
}*/
