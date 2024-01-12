
use crate::data::{ Lexeme, Ast, ParseError, LMeta };

enum Type {
    Paren,
    Angle,
    Curl,
    Square,
}

pub fn parse(input : Vec<Lexeme>) -> Result<Vec<Ast>, ParseError> {
    let mut input = input.into_iter();
    match parse_ast(&mut input)? {
        (None, ast) => Ok(ast),
        (Some(x), _) => Err(ParseError::NotAllInputConsumed(x.meta().start)),
    }
}

fn parse_ast(input : &mut impl Iterator<Item = Lexeme>) -> Result<(Option<Lexeme>, Vec<Ast>), ParseError> {

    let mut ret = vec![];
    let end = loop {
        match input.next() {
            Some(Lexeme::LParen(m)) => {
                let item = parse_bracket(Type::Paren, m.start, input)?;
                ret.push(item);
            },
            Some(Lexeme::LAngle(m)) => {
                let item = parse_bracket(Type::Angle, m.start, input)?;
                ret.push(item);
            },
            Some(Lexeme::LCurl(m)) => {
                let item = parse_bracket(Type::Curl, m.start, input)?;
                ret.push(item);
            },
            Some(Lexeme::LSquare(m)) => {
                let item = parse_bracket(Type::Square, m.start, input)?;
                ret.push(item);
            },
            Some(x @ Lexeme::RParen(_)) => { break Some(x); },
            Some(x @ Lexeme::RAngle(_)) => { break Some(x); },
            Some(x @ Lexeme::RCurl(_)) => { break Some(x); },
            Some(x @ Lexeme::RSquare(_)) => { break Some(x); },
            Some(l) => { ret.push(Ast::Lex(l)); },
            None => { break None; },
        }
    };

    Ok((end, ret))
}

fn parse_bracket(t : Type, initial : usize, input : &mut impl Iterator<Item = Lexeme>) -> Result<Ast, ParseError> {
    fn to_expected(t : Type) -> char {
        match t {
            Type::Paren => ')',
            Type::Angle => '>',
            Type::Curl => '}',
            Type::Square => ']',
        }
    }

    let (end, contents) = parse_ast(input)?;
    match (t, end) {
        (Type::Paren, Some(Lexeme::RParen(m))) => Ok(Ast::Paren(LMeta::multi(initial, m.end), contents)),
        (Type::Angle, Some(Lexeme::RAngle(m))) => Ok(Ast::Angle(LMeta::multi(initial, m.end), contents)),
        (Type::Curl, Some(Lexeme::RCurl(m))) => Ok(Ast::Curl(LMeta::multi(initial, m.end), contents)),
        (Type::Square, Some(Lexeme::RSquare(m))) => Ok(Ast::Square(LMeta::multi(initial, m.end), contents)),
        (t, Some(l)) => {
            let found = l.value().chars().nth(0).unwrap();
            let terminal = l.meta().start;
            Err(ParseError::MissingEndBracket { initial, terminal, found, expected: to_expected(t) })
        },
        (t, None) => Err(ParseError::EofInsteadOfEndBracket { initial, expected: to_expected(t) }),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::lexer::lex;

    #[test]
    fn should_parse_paren() {
        let input = "( 1 2 3 )";
        let tokens = lex(input).unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(ast.len(), 1);
        assert!(matches!(ast[0], Ast::Paren(_, _)));
    }

    #[test]
    fn should_parse_angle() {
        let input = "< 1 2 3 >";
        let tokens = lex(input).unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(ast.len(), 1);
        assert!(matches!(ast[0], Ast::Angle(_, _)));
    }

    #[test]
    fn should_parse_curl() {
        let input = "{ 1 2 3 }";
        let tokens = lex(input).unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(ast.len(), 1);
        assert!(matches!(ast[0], Ast::Curl(_, _)));
    }

    #[test]
    fn should_parse_square() {
        let input = "[ 1 2 3 ]";
        let tokens = lex(input).unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(ast.len(), 1);
        assert!(matches!(ast[0], Ast::Square(_, _)));
    }
}