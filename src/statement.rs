use std::error::Error;
use std::fmt;

use crate::table::Row;

#[derive(Debug, Clone)]
struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {}

pub enum Statement {
    Insert(Row),
    Select,
}

impl Statement {
    pub fn parse(text: &str) -> Result<Statement, Box<dyn Error>> {
        if text.starts_with("insert") {
            Ok(Statement::Insert(Row::new(0, "abc", "def")))
        } else if text.starts_with("select") {
            Ok(Statement::Select)
        } else {
            Err(Box::new(ParseError{
                message: format!("Unrecognized keyword at start of {:?}", text),
            }))
        }
    }

    pub fn execute(&self) {
        match self {
            Statement::Insert(row) => println!("Insert {}", row),
            Statement::Select => println!("Select"),
        }
    }
}


