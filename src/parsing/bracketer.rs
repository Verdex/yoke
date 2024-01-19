
use crate::data::{ Lexeme, Bracket, BracketError, LMeta };

enum Type {
    Paren,
    Angle,
    Curl,
    Square,
}

pub fn bracket(input : Vec<Lexeme>) -> Result<Vec<Bracket>, BracketError> {
    let mut input = input.into_iter();
    match parse_ast(&mut input)? {
        (None, ast) => Ok(ast),
        (Some(x), _) => Err(BracketError::NotAllInputConsumed(x.meta().start)),
    }
}

fn parse_ast(input : &mut impl Iterator<Item = Lexeme>) -> Result<(Option<Lexeme>, Vec<Bracket>), BracketError> {

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
            Some(l) => { ret.push(Bracket::Lex(l)); },
            None => { break None; },
        }
    };

    Ok((end, ret))
}

fn parse_bracket(t : Type, initial : usize, input : &mut impl Iterator<Item = Lexeme>) -> Result<Bracket, BracketError> {
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
        (Type::Paren, Some(Lexeme::RParen(m))) => Ok(Bracket::Paren(LMeta::multi(initial, m.end), contents)),
        (Type::Angle, Some(Lexeme::RAngle(m))) => Ok(Bracket::Angle(LMeta::multi(initial, m.end), contents)),
        (Type::Curl, Some(Lexeme::RCurl(m))) => Ok(Bracket::Curl(LMeta::multi(initial, m.end), contents)),
        (Type::Square, Some(Lexeme::RSquare(m))) => Ok(Bracket::Square(LMeta::multi(initial, m.end), contents)),
        (t, Some(l)) => {
            let found = l.value().chars().nth(0).unwrap();
            let terminal = l.meta().start;
            Err(BracketError::MissingEndBracket { initial, terminal, found, expected: to_expected(t) })
        },
        (t, None) => Err(BracketError::EofInsteadOfEndBracket { initial, expected: to_expected(t) }),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::lexer::lex;

    #[test]
    fn should_bracket_paren() {
        let input = "( 1 2 3 )";
        let tokens = lex(input).unwrap();
        let ast = bracket(tokens).unwrap();
        assert_eq!(ast.len(), 1);
        assert!(matches!(ast[0], Bracket::Paren(_, _)));
    }

    #[test]
    fn should_bracket_angle() {
        let input = "< 1 2 3 >";
        let tokens = lex(input).unwrap();
        let ast = bracket(tokens).unwrap();
        assert_eq!(ast.len(), 1);
        assert!(matches!(ast[0], Bracket::Angle(_, _)));
    }

    #[test]
    fn should_bracket_curl() {
        let input = "{ 1 2 3 }";
        let tokens = lex(input).unwrap();
        let ast = bracket(tokens).unwrap();
        assert_eq!(ast.len(), 1);
        assert!(matches!(ast[0], Bracket::Curl(_, _)));
    }

    #[test]
    fn should_bracket_square() {
        let input = "[ 1 2 3 ]";
        let tokens = lex(input).unwrap();
        let ast = bracket(tokens).unwrap();
        assert_eq!(ast.len(), 1);
        assert!(matches!(ast[0], Bracket::Square(_, _)));
    }

    #[test]
    fn should_bracket_paren_in_paren() {
        let input = "( 1 ( 2 3 ) )";
        let tokens = lex(input).unwrap();
        let ast = bracket(tokens).unwrap();
        assert_eq!(ast.len(), 1);
        assert!(matches!(ast[0], Bracket::Paren(_, _)));
        let items = match &ast[0] {
            Bracket::Paren(_, items) => items,
            _ => unreachable!(),
        };
        assert_eq!(items.len(), 2);
        assert!(matches!(items[0], Bracket::Lex(Lexeme::Number(_, _))));
        assert!(matches!(items[1], Bracket::Paren(_, _)));
    }
}