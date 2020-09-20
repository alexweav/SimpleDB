use std::error::Error;
use std::fmt;

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
    Insert,
    Select,
}

impl Statement {
    pub fn parse(text: &str) -> Result<Statement, Box<dyn Error>> {
        if text.starts_with("insert") {
            Ok(Statement::Insert)
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
            Statement::Insert => println!("Insert"),
            Statement::Select => println!("Select"),
        }
    }
}
