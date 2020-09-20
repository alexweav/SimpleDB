use std::fmt;

pub const USERNAME_COLUMN_WIDTH: usize = 32;
pub const EMAIL_COLUMN_WIDTH: usize = 255;

#[derive(Debug)]
pub struct Row {
    pub id: u32,
    pub username: Vec<u8>,
    pub email: Vec<u8>,
}

impl Row {
    pub fn new(id: u32, username: &str, email: &str) -> Row {
        let row = Row {
            id: id,
            username: prefill_string_column(username, USERNAME_COLUMN_WIDTH),
            email: prefill_string_column(email, EMAIL_COLUMN_WIDTH),
        };
        row
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

fn prefill_string_column(value: &str, size: usize) -> Vec<u8> {
    let mut column_data = vec![0; size];
    // TODO: handle when value is too long for the buffer
    // TODO: remember that utf8 graphemes may span multiple bytes
    column_data[..value.len()].copy_from_slice(value.as_bytes());
    column_data
}