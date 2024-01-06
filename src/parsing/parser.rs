
use crate::data::Ast;
use crate::data::ParseError;

// TODO: Delete
fn x<T>() -> Result<T, ParseError> { unreachable!() }

fn parse(input : &str) -> Result<Ast, ParseError> {
    x()
}