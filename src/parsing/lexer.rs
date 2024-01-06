
use crate::data::{Lexeme, LexError};

pub fn lex(input : &str) -> Result<Vec<Lexeme>, LexError> {

}


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