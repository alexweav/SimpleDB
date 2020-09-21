use std::convert::TryInto;
use std::fmt;
use std::str;

pub const USERNAME_COLUMN_WIDTH: usize = 32;
pub const EMAIL_COLUMN_WIDTH: usize = 255;

pub const ID_OFFSET: usize = 0;
pub const ID_COLUMN_WIDTH: usize = std::mem::size_of::<u32>();
pub const USERNAME_OFFSET: usize = ID_OFFSET + ID_COLUMN_WIDTH;
pub const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_COLUMN_WIDTH;

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

    pub fn get_username(&self) -> &str {
        str::from_utf8(&self.username).unwrap()
    }

    pub fn get_email(&self) -> &str {
        str::from_utf8(&self.email).unwrap()
    }

    pub fn serialize(&self, destination: &mut [u8]) {
        let row_width = ID_COLUMN_WIDTH + USERNAME_COLUMN_WIDTH + EMAIL_COLUMN_WIDTH;
        if destination.len() < row_width {
            panic!("Target buffer too small for row!");
        }
        destination[ID_OFFSET..ID_OFFSET+ID_COLUMN_WIDTH].copy_from_slice(&self.id.to_ne_bytes());
        destination[USERNAME_OFFSET..USERNAME_OFFSET+USERNAME_COLUMN_WIDTH].copy_from_slice(&self.username);
        destination[EMAIL_OFFSET..EMAIL_OFFSET+EMAIL_COLUMN_WIDTH].copy_from_slice(&self.email);
    }

    pub fn deserialize(&self, destination: &[u8]) -> Row {
        let id = u32::from_ne_bytes(destination[ID_OFFSET..ID_OFFSET+ID_COLUMN_WIDTH].try_into().unwrap());
        let mut username = vec![0; USERNAME_COLUMN_WIDTH];
        username[..USERNAME_COLUMN_WIDTH].copy_from_slice(&destination[USERNAME_OFFSET..USERNAME_OFFSET+USERNAME_COLUMN_WIDTH]);
        let mut email = vec![0; EMAIL_COLUMN_WIDTH];
        email[..EMAIL_COLUMN_WIDTH].copy_from_slice(&destination[EMAIL_OFFSET..EMAIL_OFFSET+EMAIL_COLUMN_WIDTH]);
        Row {
            id: id,
            username: username,
            email: email,
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.id,
            self.get_username(),
            self.get_email()
        )
    }
}

fn prefill_string_column(value: &str, size: usize) -> Vec<u8> {
    let mut column_data = vec![0; size];
    // TODO: handle when value is too long for the buffer
    // TODO: remember that utf8 graphemes may span multiple bytes
    column_data[..value.len()].copy_from_slice(value.as_bytes());
    column_data
}
