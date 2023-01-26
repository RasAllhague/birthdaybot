use self::parser::ParserError;

pub mod birthday;
mod parser;

#[derive(Debug)]
pub enum CommandError {
    Db(sqlx::Error),
    Parser(ParserError),
}