
use crate::data::Bracket;

#[derive(Debug)]
pub enum Pattern {
    Wild,
    Exact(Bracket),
    Pred(fn(&Bracket) -> bool),
}

fn pattern_match(pattern : &[Pattern], data : &[Bracket]) -> bool {
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

pub struct Rule<T> {
    pattern : Vec<Pattern>, 
    f : fn(Vec<Bracket>) -> T,
}

impl<T> Rule<T> {
    pub fn new(pattern : Vec<Pattern>, f : fn(Vec<Bracket>) -> T) -> Rule<T> {
        Rule { pattern, f }
    }
}

impl<T> From<(Vec<Pattern>, fn(Vec<Bracket>) -> T)> for Rule<T> {
    fn from(value : (Vec<Pattern>, fn(Vec<Bracket>) -> T)) ->  Self {
        Rule::new(value.0, value.1)
    }
}

#[derive(Debug)]
pub enum BracketProcessError {
    CurrentBufferExceedsAllPatterns(Vec<Bracket>),
    CurrentBufferCannotBeMatchedAgainstAnyPattern(Vec<Bracket>),
}

impl std::fmt::Display for BracketProcessError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BracketProcessError::CurrentBufferExceedsAllPatterns(xs) => 
                write!(f, "The current buffer length exceeds all available pattern lengths: {:?}", xs),
            BracketProcessError::CurrentBufferCannotBeMatchedAgainstAnyPattern(xs) => 
                write!(f, "The current buffer does not match any pattern: {:?}", xs),
        }
    }
}

impl std::error::Error for BracketProcessError { }

pub fn process<T, I : Iterator<Item = Bracket>>(rules : &[Rule<T>], mut input : I) -> Result<Vec<T>, BracketProcessError> {
    let max = rules.iter().map(|r| r.pattern.len()).max().unwrap_or(0);
    let mut match_buffer : Vec<Bracket> = vec![];
    let mut ret : Vec<T> = vec![];

    loop {
        if match_buffer.len() > max {
            return Err(BracketProcessError::CurrentBufferExceedsAllPatterns(match_buffer));
        }
        for rule in rules {
            if rule.pattern.len() == match_buffer.len() && pattern_match(&rule.pattern, &match_buffer) {
                ret.push((rule.f)(std::mem::replace(&mut match_buffer, vec![])));
            }
        }    
        match input.next() {
            None if match_buffer.len() > 0 => { 
                return Err(BracketProcessError::CurrentBufferCannotBeMatchedAgainstAnyPattern(match_buffer)); 
            },
            None => { break; }
            Some(x) => { match_buffer.push(x); },
        }
    }

    Ok(ret)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parsing::lexer;
    use crate::parsing::bracketer;
    use crate::data::*;

    #[test]
    fn should_process_brackets() {
        fn is_paren(x : &Bracket) -> bool { matches!(x, Bracket::Paren(_, _)) }
        fn is_curl(x : &Bracket) -> bool { matches!(x, Bracket::Curl(_, _)) }

        let input = "if (stuff) { a = 1; b = 2; } else { c = y(1, 2, 3); }";
        let tokens = lexer::lex(&input).unwrap().into_iter();
        let brackets = bracketer::bracket(tokens).unwrap().into_iter();
        let if_sym = Bracket::Lex(Lexeme::Symbol(LMeta::new(), "if".to_string()));
        let else_sym = Bracket::Lex(Lexeme::Symbol(LMeta::new(), "else".to_string()));

        let if_rule = Rule::new(vec![Pattern::Exact(if_sym), Pattern::Pred(is_paren), Pattern::Pred(is_curl)], 
                               |_| 0);
        let else_rule = Rule::new(vec![Pattern::Exact(else_sym), Pattern::Pred(is_curl)],
                                 |_| 1);
        let rules = vec![if_rule, else_rule];

        let output = process(&rules, brackets).unwrap();

        assert_eq!(output.len(), 2);
        assert_eq!(output[0], 0);
        assert_eq!(output[1], 1);
    }
}