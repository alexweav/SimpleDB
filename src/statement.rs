use std::error::Error;
use std::fmt;

use crate::table::Row;

#[derive(Debug, Clone)]
struct ParseError {
    message: String,
}

impl ParseError {
    pub fn from_string(message: String) -> ParseError {
        ParseError { message: message }
    }

    pub fn from_str(message: &str) -> ParseError {
        ParseError {
            message: String::from(message),
        }
    }
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
            parse_insert(text)
        } else if text.starts_with("select") {
            Ok(Statement::Select)
        } else {
            Err(Box::new(ParseError::from_string(format!(
                "Unrecognized keyword at start of {:?}",
                text
            ))))
        }
    }

    pub fn execute(&self) {
        match self {
            Statement::Insert(row) => println!("Inserted {}", row),
            Statement::Select => println!("Select"),
        }
    }
}

fn parse_insert(text: &str) -> Result<Statement, Box<dyn Error>> {
    let mut tokens = text.split(' ');
    tokens.next().unwrap();
    let id = tokens
        .next()
        .ok_or(ParseError::from_str("Field missing from row: \"id\""))?
        .parse::<u32>()?;
    let username = tokens
        .next()
        .ok_or(ParseError::from_str("Field missing from row: \"username\""))?;
    let email = tokens
        .next()
        .ok_or(ParseError::from_str("Field missing from row: \"email\""))?;
    Ok(Statement::Insert(Row::new(id, username, email)))
}
