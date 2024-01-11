
use crate::data::{ Lexeme, Ast, ParseError };

enum Type {
    Paren,
    Angle,
    Curl,
    Square,
}

pub fn parse(input : Vec<Lexeme>) -> Result<Vec<Ast>, ParseError> {
    let mut input = input.into_iter();

    let mut ret = vec![];
    loop {
        match input.next() {
            Some(Lexeme::LParen(m)) => {
                let item = parse_bracket(Type::Paren, &mut input)?;
                ret.push(item);
            }
        }
    }

    Ok(ret)
}


fn parse_bracket(t : Type, input : &mut impl Iterator<Item = Lexeme>) -> Result<Ast, ParseError> {

}

// TODO 
// * parse all brackets with stuff inside
