
use crate::data::{LMeta, Lexeme};

pub struct LexProcessor<T, F, const N : usize> {
    input : T,
    f : F,
    pattern : [Pattern; N],
    match_buffer : Vec<Lexeme>,
}

impl<T : Iterator<Item = Lexeme>, F : FnMut(Vec<Lexeme>) -> Vec<Lexeme>, const N : usize>
    Iterator for LexProcessor<T, F, N> {

    type Item = Vec<Lexeme>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.pattern.len() == 0 {
                match self.input.next() {
                    None => { return None; },
                    Some(l) => { return Some(vec![l]); },
                }
            }
            if self.match_buffer.len() == self.pattern.len() && pattern_match(&self.pattern, &self.match_buffer) {
                return Some((self.f)(std::mem::replace(&mut self.match_buffer, vec![])));
            }
            match self.input.next() {
                None if self.match_buffer.len() == 0 => { break None; },
                None if self.match_buffer.len() < self.pattern.len() => { 
                    return Some(std::mem::replace(&mut self.match_buffer, vec![]));
                },
                None if self.match_buffer.len() > self.pattern.len() => { 
                    return Some(vec![self.match_buffer.remove(0)]);
                },
                None if pattern_match(&self.pattern, &self.match_buffer) => { 
                    return Some((self.f)(std::mem::replace(&mut self.match_buffer, vec![])));
                },
                None => {
                    return Some(std::mem::replace(&mut self.match_buffer, vec![]));
                },
                Some(l) => {
                    self.match_buffer.push(l);
                    if self.match_buffer.len() > self.pattern.len() {
                        return Some(vec![self.match_buffer.remove(0)]);
                    }
                    else if self.match_buffer.len() < self.pattern.len() {
                        continue;
                    }
                    else if pattern_match(&self.pattern, &self.match_buffer) {
                        return Some((self.f)(std::mem::replace(&mut self.match_buffer, vec![])));
                    }
                }
            }
        }
    }
}

fn pattern_match(pattern : &[Pattern], data : &[Lexeme]) -> bool {
    let pds = pattern.iter().zip(data.iter());
    for pd in pds {
        match pd {
            (Pattern::Wild, _) => { },
            (Pattern::Pred(f), d) if f(d) => { },
            (Pattern::Exact(l), d) if l.lmatch(d) => { },
            _ => { return false; },
        }
    }
    true
}

#[derive(Debug)]
pub enum Pattern {
    Wild,
    Exact(Lexeme),
    Pred(fn(&Lexeme) -> bool),
}

pub fn grouper<T : Iterator<Item = Lexeme>, S : AsRef<str>, const N : usize>(pattern : [Pattern; N], label : S, input : T) -> impl Iterator<Item = Lexeme> { 

    let label = label.as_ref().to_string();

    let f = move |ls : Vec<Lexeme>| {
        let start = ls.first().unwrap().meta().start;
        let end = ls.last().unwrap().meta().end;
        vec![Lexeme::Group(LMeta::multi(start, end), label.clone(), ls)]
    };

    LexProcessor { input, f, pattern, match_buffer: vec![] }.flatten()
}

pub fn process< T : Iterator<Item = Lexeme>
              , F : FnMut(Vec<Lexeme>) -> Vec<Lexeme>
              , const N : usize
              >(pattern : [Pattern; N], f : F, input : T) -> LexProcessor<T, F, N> {

    LexProcessor { input, f, pattern, match_buffer: vec![] }
}

pub fn r_paren() -> Lexeme { Lexeme::RParen(LMeta::new()) } 
pub fn l_paren() -> Lexeme { Lexeme::LParen(LMeta::new()) } 
pub fn r_angle() -> Lexeme { Lexeme::RAngle(LMeta::new()) } 
pub fn l_angle() -> Lexeme { Lexeme::LAngle(LMeta::new()) } 
pub fn r_curl() -> Lexeme { Lexeme::RCurl(LMeta::new()) } 
pub fn l_curl() -> Lexeme { Lexeme::LCurl(LMeta::new()) } 
pub fn r_square() -> Lexeme { Lexeme::RSquare(LMeta::new()) } 
pub fn l_square() -> Lexeme { Lexeme::LSquare(LMeta::new()) } 
pub fn punct(c : char) -> Lexeme { Lexeme::Punct(LMeta::new(), c) }
pub fn group<S : AsRef<str>>(label : S, ls : Vec<Lexeme>) -> Lexeme { Lexeme::Group(LMeta::new(), label.as_ref().to_string(), ls) }
pub fn string<S : AsRef<str>>(s : S) -> Lexeme { Lexeme::String(LMeta::new(), s.as_ref().to_string()) }
pub fn number<S : AsRef<str>>(s : S) -> Lexeme { Lexeme::Number(LMeta::new(), s.as_ref().to_string()) }
pub fn symbol<S : AsRef<str>>(s : S) -> Lexeme { Lexeme::Symbol(LMeta::new(), s.as_ref().to_string()) }

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::Lexeme;
    use crate::parsing::lexer;

    #[test]
    fn should_process() {
        let input = "1 2 3";
        let tokens = lexer::lex(&input).unwrap();
        let output = process([Pattern::Wild], |mut ls| { ls.push(number("0")); ls }, tokens.into_iter()).flatten().collect::<Vec<_>>();
        assert_eq!(output.len(), 6);
        assert!(matches!(output[0], Lexeme::Number(_, _)));
        assert!(matches!(output[1], Lexeme::Number(_, _)));
        assert!(matches!(output[2], Lexeme::Number(_, _)));
        assert!(matches!(output[3], Lexeme::Number(_, _)));
        assert!(matches!(output[4], Lexeme::Number(_, _)));
        assert!(matches!(output[5], Lexeme::Number(_, _)));

        if let Lexeme::Number(_, n) = &output[0] { 
            assert_eq!(n, "1");
        }
        if let Lexeme::Number(_, n) = &output[1] { 
            assert_eq!(n, "0");
        }
        if let Lexeme::Number(_, n) = &output[2] { 
            assert_eq!(n, "2");
        }
        if let Lexeme::Number(_, n) = &output[3] { 
            assert_eq!(n, "0");
        }
        if let Lexeme::Number(_, n) = &output[4] { 
            assert_eq!(n, "3");
        }
        if let Lexeme::Number(_, n) = &output[5] { 
            assert_eq!(n, "0");
        }
    }

    #[test]
    fn should_group_with_zero_length_input() {
        let input = "";
        let tokens = lexer::lex(&input).unwrap();
        let output = grouper([Pattern::Wild], "label", tokens.into_iter()).collect::<Vec<_>>();
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn should_group_with_zero_length_pattern() {
        let input = "1 2 3";
        let tokens = lexer::lex(&input).unwrap();
        let output = grouper([], "label", tokens.into_iter()).collect::<Vec<_>>();
        assert_eq!(output.len(), 3);
        assert!(matches!(output[0], Lexeme::Number(_, _)));
        assert!(matches!(output[1], Lexeme::Number(_, _)));
        assert!(matches!(output[2], Lexeme::Number(_, _)));
    }

    #[test]
    fn should_group_with_wild_pattern() {
        let input = "1 2 3";
        let tokens = lexer::lex(&input).unwrap();
        let output = grouper([Pattern::Wild, Pattern::Wild], "label", tokens.into_iter()).collect::<Vec<_>>();
        assert_eq!(output.len(), 2);
        assert!(matches!(output[0], Lexeme::Group(_, _, _)));
        assert!(matches!(output[1], Lexeme::Number(_, _)));

        if let Lexeme::Group(meta, label, ls) = &output[0] {
            assert_eq!(meta.start, 0);
            assert_eq!(meta.end, 2);
            assert_eq!(label, "label");
            assert_eq!(ls.len(), 2);
        }

        if let Lexeme::Number(_, n) = &output[1] { 
            assert_eq!(n, "3");
        }
    }

    #[test]
    fn should_group_with_exact_pattern() {
        let input = "1 2 3 4";
        let tokens = lexer::lex(&input).unwrap();
        let output = grouper([Pattern::Exact(number("1")), Pattern::Wild], "label", tokens.into_iter()).collect::<Vec<_>>();
        assert_eq!(output.len(), 3);
        assert!(matches!(output[0], Lexeme::Group(_, _, _)));
        assert!(matches!(output[1], Lexeme::Number(_, _)));
        assert!(matches!(output[2], Lexeme::Number(_, _)));

        if let Lexeme::Group(meta, label, ls) = &output[0] {
            assert_eq!(meta.start, 0);
            assert_eq!(meta.end, 2);
            assert_eq!(label, "label");
            assert_eq!(ls.len(), 2);
        }

        if let Lexeme::Number(_, n) = &output[1] { 
            assert_eq!(n, "3");
        }

        if let Lexeme::Number(_, n) = &output[2] { 
            assert_eq!(n, "4");
        }
    }

    #[test]
    fn should_group_with_pred_pattern() {
        fn odd(l : &Lexeme) -> bool {
            match l {
                Lexeme::Number(_, x) => x.parse::<u8>().unwrap() % 2 == 1,
                _ => false, 
            }
        }

        let input = "1 2 3";
        let tokens = lexer::lex(&input).unwrap();
        let output = grouper([Pattern::Pred(odd), Pattern::Wild], "label", tokens.into_iter()).collect::<Vec<_>>();
        assert_eq!(output.len(), 2);
        assert!(matches!(output[0], Lexeme::Group(_, _, _)));
        assert!(matches!(output[1], Lexeme::Number(_, _)));

        if let Lexeme::Group(meta, label, ls) = &output[0] {
            assert_eq!(meta.start, 0);
            assert_eq!(meta.end, 2);
            assert_eq!(label, "label");
            assert_eq!(ls.len(), 2);
        }

        if let Lexeme::Number(_, n) = &output[1] { 
            assert_eq!(n, "3");
        }
    }
    
    #[test]
    fn should_group_float_like_structure() {
        fn any_num() -> Pattern {
            Pattern::Pred(|x| matches!(x, Lexeme::Number(_, _)))
        }

        let input = "1.2 0 12.34 5 3.4";
        let tokens = lexer::lex(&input).unwrap().into_iter();
        let output = grouper([any_num(), Pattern::Exact(punct('.')), any_num()], "float", tokens).collect::<Vec<_>>();
        assert_eq!(output.len(), 5);
        assert!(matches!(output[0], Lexeme::Group(_, _, _)));
        assert!(matches!(output[1], Lexeme::Number(_, _)));
        assert!(matches!(output[2], Lexeme::Group(_, _, _)));
        assert!(matches!(output[3], Lexeme::Number(_, _)));
        assert!(matches!(output[4], Lexeme::Group(_, _, _)));

        if let Lexeme::Group(meta, label, ls) = &output[0] {
            assert_eq!(meta.start, 0);
            assert_eq!(meta.end, 2);
            assert_eq!(label, "float");
            assert_eq!(ls.len(), 3);
            assert!(matches!(ls[0], Lexeme::Number(_, _)));
            assert!(matches!(ls[1], Lexeme::Punct(_, _)));
            assert!(matches!(ls[2], Lexeme::Number(_, _)));
        }

        if let Lexeme::Number(_, n) = &output[1] { 
            assert_eq!(n, "0");
        }

        if let Lexeme::Group(meta, label, ls) = &output[2] {
            assert_eq!(meta.start, 6);
            assert_eq!(meta.end, 10);
            assert_eq!(label, "float");
            assert_eq!(ls.len(), 3);
            assert!(matches!(ls[0], Lexeme::Number(_, _)));
            assert!(matches!(ls[1], Lexeme::Punct(_, _)));
            assert!(matches!(ls[2], Lexeme::Number(_, _)));
        }

        if let Lexeme::Number(_, n) = &output[3] { 
            assert_eq!(n, "5");
        }

        if let Lexeme::Group(meta, label, ls) = &output[4] {
            assert_eq!(meta.start, 14);
            assert_eq!(meta.end, 16);
            assert_eq!(label, "float");
            assert_eq!(ls.len(), 3);
            assert!(matches!(ls[0], Lexeme::Number(_, _)));
            assert!(matches!(ls[1], Lexeme::Punct(_, _)));
            assert!(matches!(ls[2], Lexeme::Number(_, _)));
        }
    }
}
