

use crate::data::{Lexeme, LMeta, LexError};

macro_rules! i {
    ($p:pat, end) => { Some( (Some($p), None) ) };
    ($p1:pat, $p2:pat) => { Some( (Some($p1), Some($p2)) ) };
    ($p:pat) => { Some( (Some($p), _) ) };
}

macro_rules! input {
    () => { &mut impl Iterator<Item = (Option<(usize, char)>, Option<(usize, char)>)> };
}

pub fn lex(input : &str) -> Result<Vec<Lexeme>, LexError> {
    let input = input.char_indices().map(|c| Some(c));
    let mut input = input.clone().zip(input.skip(1).chain(std::iter::once(None)));

    let mut comment = 0;

    let mut ret = vec![];
    loop {
        let x = input.next();

        if comment > 0 {
            match x {
                i!((_, '/'), (_, '*')) => { comment += 1; input.next(); },
                i!((_, '*'), (_, '/')) => { comment -= 1; input.next(); },
                _ => { },
            }
            continue;
        }

        match x {
            i!((_, c)) if c.is_whitespace() => { },
            i!((_, '/'), (_, '*')) => { comment += 1; input.next(); },
            i!((_, '/'), (_, '/')) => { skip_line(&mut input); },

            i!((index, ')')) => { ret.push(Lexeme::RParen(LMeta::single(index))); },
            i!((index, '(')) => { ret.push(Lexeme::LParen(LMeta::single(index))); },
            
            i!((index, '>')) => { ret.push(Lexeme::RAngle(LMeta::single(index))); },
            i!((index, '<')) => { ret.push(Lexeme::LAngle(LMeta::single(index))); },

            i!((index, '}')) => { ret.push(Lexeme::RCurl(LMeta::single(index))); },
            i!((index, '{')) => { ret.push(Lexeme::LCurl(LMeta::single(index))); },

            i!((index, ']')) => { ret.push(Lexeme::RSquare(LMeta::single(index))); },
            i!((index, '[')) => { ret.push(Lexeme::LSquare(LMeta::single(index))); },

            i!((index, '.')) => { ret.push(Lexeme::Dot(LMeta::single(index))); },
            i!((index, ',')) => { ret.push(Lexeme::Comma(LMeta::single(index))); },
            i!((index, ';')) => { ret.push(Lexeme::SemiColon(LMeta::single(index))); },
            i!((index, ':')) => { ret.push(Lexeme::Colon(LMeta::single(index))); },
            i!((start, '='), (end, '>')) => { 
                ret.push(Lexeme::RDoubleArrow(LMeta::multi(start, end)));
                input.next();
            },
            i!((index, '=')) => { ret.push(Lexeme::Equal(LMeta::single(index))); },
            i!((index, '$')) => { ret.push(Lexeme::Dollar(LMeta::single(index))); },
            i!((index, '^')) => { ret.push(Lexeme::Caret(LMeta::single(index))); },
            i!((start, '-'), (end, '>')) => { 
                ret.push(Lexeme::RArrow(LMeta::multi(start, end)));
                input.next();
            },

            i!((index, c)) if c.is_numeric() || c == '+' || c == '-' => {
                let num = lex_number(c, index, &mut input)?;
                ret.push(num);
            },
            i!((index, c)) if c.is_alphabetic() || c == '_' => {
                let sym = lex_symbol(c, index, &mut input)?;
                ret.push(sym);
            },
            i!((index, '"')) => {
                let s = lex_string(index, &mut input)?;
                ret.push(s);
            },
            i!((index, c)) => { return Err(LexError::UnexpectedToken(index, c)); },
            None => { break; },
            _ => unreachable!(),
        }
    }

    Ok(ret)
}

fn skip_line(input : input!()) {
    loop {
        match input.next() {
            i!(_, (_, c)) if c == '\n' || c == '\r' => { break; },
            None => { break; },
            _ => { },
        }
    }
}

fn lex_item(c : char, start : usize, target : fn(char) -> bool, input : input!()) -> Result<(LMeta, String), LexError> {
    let mut end = start;

    let mut ret = vec![c];
    loop {
        match input.next() {
            i!((index, c), (_, x)) if target(c) && !target(x) => {
                ret.push(c);
                end = index;
                break;
            },
            i!((index, c), end) => if target(c) {
                ret.push(c);
                end = index;
                break;
            },
            i!((_, c)) if target(c) => { ret.push(c); },
            i!(_) => { break; },
            None => { break; },
            _ => unreachable!(),
        }
    }

    Ok((LMeta::multi(start, end), ret.into_iter().collect::<String>()))
} 

fn lex_number(c : char, start : usize, input : input!()) -> Result<Lexeme, LexError> {
    fn target(x : char) -> bool {
        x.is_numeric() || x == '+' || x == '-' || x == '.' || x == 'E' || x == 'e'
    }

    let (meta, item) = lex_item(c, start, target, input)?;

    Ok(Lexeme::Number(meta, item))
}

fn lex_symbol(c : char, start : usize, input : input!()) -> Result<Lexeme, LexError> {
    fn target(x : char) -> bool {
        x.is_alphanumeric() || x == '_' 
    }

    let (meta, item) = lex_item(c, start, target, input)?;

    Ok(Lexeme::Symbol(meta, item))
}

fn lex_string(start : usize, input : input!()) -> Result<Lexeme, LexError> {
    let end;

    let mut ret = vec![];
    loop {
        match input.next() {
            i!((index, '"')) => { end = index; break; },
            i!((_, '\\'), (_, 't')) => { ret.push('\t'); input.next(); },
            i!((_, '\\'), (_, 'n')) => { ret.push('\n'); input.next(); },
            i!((_, '\\'), (_, 'r')) => { ret.push('\r'); input.next(); },
            i!((_, '\\'), (_, '0')) => { ret.push('\0'); input.next(); },
            i!((_, '\\'), (_, '\\')) => { ret.push('\\'); input.next(); },
            i!((_, '\\'), (_, '"')) => { ret.push('"'); input.next(); },
            i!((_, '\\'), (index, c)) => { return Err(LexError::UnexpectedEscapeInString(index, c)); },
            i!((_, c)) => { ret.push(c); },
            None => { return Err(LexError::EncounteredEndInString); },
            _ => unreachable!(),
        }
    }
    Ok(Lexeme::String(LMeta::multi(start, end), ret.into_iter().collect::<String>()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_handle_whitespace() {
        let output = lex(" \t \r \n ").unwrap();
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn should_handle_comment() {
        let output = lex(" // comment $ ").unwrap();
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn should_handle_block_comment() {
        let input = "

    /*
        block comment $

    */

";
        let output = lex(input).unwrap();
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn should_handle_nested_block_comment() {
        let input = "

    /*
        block comment $
        /* %%% */
        /* %%% */
        /* /* */ */
    */

";
        let output = lex(input).unwrap();
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn should_handle_nested_block_with_termination_in_comment() {
        let input = "

    /*
    // */

    77

";
        let output = lex(input).unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0].meta(), LMeta::multi(24, 25));
        assert_eq!(output[0].value(), "77");
    }

    #[test]
    fn should_lex_single_digit_and_nothing_else() {
        let input = "7";
        let output = lex(input).unwrap();
        assert_eq!(output.len(), 1);
        assert!(matches!(output[0], Lexeme::Number(_, _)));
        assert_eq!(output[0].meta(), LMeta::single(0));
        assert_eq!(output[0].value(), "7");
    }

    #[test]
    fn should_lex_numbers() {
        let input = "1234.5678E+90 +1234 -1234 +123e-456";
        let output = lex(input).unwrap();
        assert_eq!(output.len(), 4);
        assert!(matches!(output[0], Lexeme::Number(_, _)));
        assert_eq!(output[0].meta(), LMeta::multi(0, 12));
        assert_eq!(output[0].value(), "1234.5678E+90");

        assert!(matches!(output[1], Lexeme::Number(_, _)));
        assert_eq!(output[1].meta(), LMeta::multi(14, 18));
        assert_eq!(output[1].value(), "+1234");

        assert!(matches!(output[2], Lexeme::Number(_, _)));
        assert_eq!(output[2].meta(), LMeta::multi(20, 24));
        assert_eq!(output[2].value(), "-1234");

        assert!(matches!(output[3], Lexeme::Number(_, _)));
        assert_eq!(output[3].meta(), LMeta::multi(26, 34));
        assert_eq!(output[3].value(), "+123e-456");
    }

    #[test]
    fn should_lex_single_letter_and_nothing_else() {
        let input = "a";
        let output = lex(input).unwrap();
        assert_eq!(output.len(), 1);
        assert!(matches!(output[0], Lexeme::Symbol(_, _)));
        assert_eq!(output[0].meta(), LMeta::single(0));
        assert_eq!(output[0].value(), "a");
    }

    #[test]
    fn should_lex_symbols() {
        let input = "Symbol symb0l _sym_bol8 _1symboL";
        let output = lex(input).unwrap();
        assert_eq!(output.len(), 4);
        assert!(matches!(output[0], Lexeme::Symbol(_, _)));
        assert_eq!(output[0].meta(), LMeta::multi(0, 5));
        assert_eq!(output[0].value(), "Symbol");

        assert!(matches!(output[1], Lexeme::Symbol(_, _)));
        assert_eq!(output[1].meta(), LMeta::multi(7, 12));
        assert_eq!(output[1].value(), "symb0l");

        assert!(matches!(output[2], Lexeme::Symbol(_, _)));
        assert_eq!(output[2].meta(), LMeta::multi(14, 22));
        assert_eq!(output[2].value(), "_sym_bol8");

        assert!(matches!(output[3], Lexeme::Symbol(_, _)));
        assert_eq!(output[3].meta(), LMeta::multi(24, 31));
        assert_eq!(output[3].value(), "_1symboL");
    }

    #[test]
    fn should_lex_string() {
        let input = " \"string \\t \\n \\r \\0 \\\\ \\\" \"";
        let output = lex(input).unwrap();
        assert_eq!(output.len(), 1);
        assert!(matches!(output[0], Lexeme::String(_, _)));
        assert_eq!(output[0].meta(), LMeta::multi(1, 27));
        assert_eq!(output[0].value(), "string \t \n \r \0 \\ \" ");
    }

    #[test]
    fn should_lex_punctuation() {
        let input = " => -> () <> {} [] . , ; : = $ ^";
        let output = lex(input).unwrap();
        assert_eq!(output.len(), 17);
        assert!(matches!(output[0], Lexeme::RDoubleArrow(_)));
        assert!(matches!(output[1], Lexeme::RArrow(_)));
        assert!(matches!(output[2], Lexeme::LParen(_)));
        assert!(matches!(output[3], Lexeme::RParen(_)));
        assert!(matches!(output[4], Lexeme::LAngle(_)));
        assert!(matches!(output[5], Lexeme::RAngle(_)));
        assert!(matches!(output[6], Lexeme::LCurl(_)));
        assert!(matches!(output[7], Lexeme::RCurl(_)));
        assert!(matches!(output[8], Lexeme::LSquare(_)));
        assert!(matches!(output[9], Lexeme::RSquare(_)));
        assert!(matches!(output[10], Lexeme::Dot(_)));
        assert!(matches!(output[11], Lexeme::Comma(_)));
        assert!(matches!(output[12], Lexeme::SemiColon(_)));
        assert!(matches!(output[13], Lexeme::Colon(_)));
        assert!(matches!(output[14], Lexeme::Equal(_)));
        assert!(matches!(output[15], Lexeme::Dollar(_)));
        assert!(matches!(output[16], Lexeme::Caret(_)));
    }
}