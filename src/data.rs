
#[derive(Debug)]
pub enum Lexeme { 

}

#[derive(Debug)]
pub enum Ast {

}

#[derive(Debug)]
pub enum ParseError {

}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for ParseError { }

#[derive(Debug)]
pub enum LexError {

}

impl std::fmt::Display for LexError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for LexError { }