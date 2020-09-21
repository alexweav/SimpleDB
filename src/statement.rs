use std::error::Error;
use std::fmt;

use crate::row::Row;
use crate::table::Table;

/// Represents an error when parsing commands.
#[derive(Debug, Clone)]
struct ParseError {
    message: String,
}

impl ParseError {
    /// Creates a parse error from an error message stored in a `String`.
    pub fn from_string(message: String) -> ParseError {
        ParseError { message: message }
    }

    /// Creates a parse error from an error message stored in a `&str`.
    pub fn from_str(message: &str) -> ParseError {
        ParseError {
            message: String::from(message),
        }
    }
}

impl fmt::Display for ParseError {
    /// Writes the parse error.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {}

/// A parsed statement in our query language.
pub enum Statement {
    Insert(Row),
    Select,
}

impl Statement {
    /// Parses a statement.
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

    /// Executes a parsed statement.
    pub fn execute(&self, table: &mut Table) {
        match self {
            Statement::Insert(row) => {
                let row_slot = table.row_slot(table.num_rows);
                row.serialize(row_slot);
                table.num_rows += 1;
            }
            Statement::Select => {
                for i in 0..table.num_rows {
                    let row_slot = table.row_slot(i);
                    println!("{}", Row::deserialize(row_slot));
                }
            }
        }
    }
}

/// Parses an insert command. Returns Err() if the parse fails.
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
