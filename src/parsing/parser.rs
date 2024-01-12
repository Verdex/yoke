
use crate::data::{ Lexeme, Ast, ParseError };

enum Type {
    Paren,
    Angle,
    Curl,
    Square,
}

pub fn parse(input : Vec<Lexeme>) -> Result<Vec<Ast>, ParseError> {
    let mut input = input.into_iter();
    parse_ast(&mut input)
}

fn parse_ast(input : &mut impl Iterator<Item = Lexeme>) -> Result<Vec<Ast>, ParseError> {
    let mut input = input.into_iter();

    let mut ret = vec![];
    loop {
        match input.next() {
            Some(Lexeme::LParen(m)) => {
                let item = parse_bracket(Type::Paren, &mut input)?;
                ret.push(item);
            },
            Some(Lexeme::LAngle(m)) => {
                let item = parse_bracket(Type::Angle, &mut input)?;
                ret.push(item);
            },
            Some(Lexeme::LCurl(m)) => {
                let item = parse_bracket(Type::Curl, &mut input)?;
                ret.push(item);
            },
            Some(Lexeme::LSquare(m)) => {
                let item = parse_bracket(Type::Square, &mut input)?;
                ret.push(item);
            },
            Some(l) => { ret.push(Ast::Lex(l)); },
            None => { break; },
        }
    }

    Ok(ret)
}

fn parse_bracket(t : Type, input : &mut impl Iterator<Item = Lexeme>) -> Result<Ast, ParseError> {
    todo!()
}