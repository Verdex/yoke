

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
            i!((index, c)) if c.is_numeric() || c == '+' || c == '-' => {
                let num = lex_number(c, index, &mut input)?;
                ret.push(num);
            },
            None => { break; },
            _ => todo!(),
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

fn lex_number(c : char, start : usize, input : input!()) -> Result<Lexeme, LexError> {
    fn target(x : char) -> bool {
        x.is_numeric() || x == '+' || x == '-' || x == '.' || x == 'E' || x == 'e'
    }

    let mut end = start;

    let mut ret = vec![c];
    loop {
        match input.next() {
            i!((index, c), (_, x)) if !target(x) => {
                ret.push(c);
                end = index;
                break;
            },
            i!((index, c), end) => {
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

    Ok(Lexeme::Number(LMeta::multi(start, end), ret.into_iter().collect::<String>()))
}


/*

pub (crate) fn parse_word(input : &mut Chars) -> Result<Box<str>, ParseError> {
    pat!(underscore: char => char = '_' => '_');

    fn parse_alpha(input : &mut Chars) -> Result<char, ParseError> {
        parser!(input => {
            init_symbol <= parse_any;
            where init_symbol.is_alphabetic();
            select init_symbol
        })
    }

    fn parse_init(input : &mut Chars) -> Result<char, ParseError> {
        alt!(input => parse_alpha; underscore)
    }

    fn parse_symbol_char(input : &mut Chars) -> Result<char, ParseError> {
        alt!(input => parse_alpha; parse_digit; underscore)
    }

    parser!(input => {
        init <= parse_init;
        rest <= * parse_symbol_char;
        select {
            let mut rest = rest;
            rest.insert(0, init);
            rest.into_iter().collect::<String>().into()
        } 
    })
}

pub (crate) fn parse_string(input : &mut Chars) -> Result<Box<str>, ParseError> {
    pat!(parse_n: char => char = 'n' => '\n');
    pat!(parse_r: char => char = 'r' => '\r');
    pat!(parse_t: char => char = 't' => '\t');
    pat!(parse_slash: char => char = '\\' => '\\');
    pat!(parse_zero: char => char = '0' => '\0');
    pat!(parse_quote: char => char = '"' => '"');

    fn parse_code(input : &mut Chars) -> Result<char, ParseError> {
        alt!(input => parse_n; parse_r; parse_t; parse_slash; parse_zero; parse_quote)
    }

    fn parse_escape(input : &mut Chars) -> Result<char, ParseError> {
        parser!(input => {
            _slash <= parse_slash;
            code <= ! parse_code;
            select code
        })
    }

    fn parse_any_but_quote(input : &mut Chars) -> Result<char, ParseError> {
        parser!(input => {
            any <= parse_any;
            where any != '"';
            select any
        })
    }

    fn parse_str_char(input : &mut Chars) -> Result<char, ParseError> {
        alt!(input => parse_escape; parse_any_but_quote)
    }

    parser!(input => {
        _start_quote <= parse_quote;
        str_chars <= * parse_str_char;
        _end_quote <= parse_quote;
        select str_chars.into_iter().collect::<String>().into()
    })
}

*/

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
        assert_eq!(output[0].meta(), LMeta::single(0));
        assert_eq!(output[0].value(), "7");
    }

}