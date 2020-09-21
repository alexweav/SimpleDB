const TABLE_MAX_PAGES: usize = 256;

pub struct Table {
    num_rows: usize,
    pages: Vec<Vec<u8>>
}

impl Table {
    pub fn new() -> Table {
        Table {
            num_rows: 0,
            pages: vec![Vec::new(); TABLE_MAX_PAGES],
        }
    }
}
