
use std::str::CharIndices;

use crate::data::{Lexeme, LMeta, LexError};

macro_rules! i {
    ($p1:pat, $p2:pat) => { Some( (Some($p1), Some($p2)) ) };
}

pub fn lex(input : &str) -> Result<Vec<Lexeme>, LexError> {
    let input = input.char_indices().map(|c| Some(c));
    let initial = input.clone().chain(std::iter::once(None));
    let mut input = initial.zip(input.skip(1));

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
            i!((_, c), _) if c.is_whitespace() => { },
            i!((_, '/'), (_, '*')) => { comment += 1; input.next(); },
            i!((_, '/'), (_, '/')) => { skip_line(&mut input); },
            //Some(((_, '/'), (_, '/'))) => { skip_line(&mut input); },
            None => { break; },
            _ => todo!(),
        }
    }

    Ok(ret)
}

fn skip_line(input : &mut impl Iterator<Item = (Option<(usize, char)>, Option<(usize, char)>)>) {
    loop {
        match input.next() {
            i!(_, (_, c)) if c == '\n' || c == '\r' => { break; },
            None => { break; },
            _ => { },
        }
    }
}

/*fn lex_number(input : &mut CharIndices) -> Result<Lexeme, LexError> {

}*/


/*
pub (crate) fn parse_whitespace(input : &mut Chars) -> Result<(), ParseError> {
    fn space(input : &mut Chars) -> Result<(), ParseError> {
        parser!( input => {
            x <= parse_any;
            where x.is_whitespace();
            select ()
        })
    }

    parser!( input => {
        _x <= * space;
        select ()
    })
}

pub (crate) fn parse_digit(input : &mut Chars) -> Result<char, ParseError> {
    parser!(input => {
        num <= parse_any;
        where num.is_digit(10);
        select num
    })
}

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
    fn should_handle_nested_block_with_termination_in_comment() { // TODO need to add something to lex here
        let input = "

    /*
    // */

";
        let output = lex(input).unwrap();
        assert_eq!(output.len(), 0);
    }
}