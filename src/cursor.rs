use crate::table;

pub struct Cursor {
    pub row_num: usize,
    pub end_of_table: bool,
}

impl Cursor {
    pub fn start(table: &table::Table) -> Cursor {
        Cursor {
            row_num: 0,
            end_of_table: table.num_rows == 0,
        }
    }

    pub fn end(table: &table::Table) -> Cursor {
        Cursor {
            row_num: table.num_rows,
            end_of_table: true,
        }
    }

    pub fn advance(&mut self, table: &table::Table) {
        self.row_num += 1;
        if self.row_num >= table.num_rows {
            self.end_of_table = true;
        }
    }
}