
use super::parser::{Parser, bind, unit, exact, any, peek};
use super::ast::Ast;
    
use monad::compute;


macro_rules! trim { 
    ($p : expr) => {
        compute!{ bind, unit => 
            _junk_1 <- junk().zero_or_more();
            t <- $p; 
            _junk_2 <- junk().zero_or_more();
            unit t.clone()
        }
    };
}

pub fn parse() -> Vec<Ast> {

    vec![]
}

fn number_literal() -> Parser<Ast> {
    // TODO also need to handle floats
    // TODO also need to handle negative
    // TODO also need to handle sci notation
    let p = any().when(|d| d.is_digit(10) ).one_or_more();
    
    p.map(|ds| Ast::Integer(
        ds.into_iter()
            .collect::<String>()
            .parse::<i64>()
            .expect("Parsed integer fails parse::<i64>()")))
} 

fn bool_literal() -> Parser<Ast> {
    let not_sym_char = || peek().when(|c| !c.is_digit(10) && *c != '_' && !c.is_alphabetic());
       
    let p = compute!{ bind, unit => 
        v <- exact("true").or(exact("false"));
        _i <- not_sym_char();
        unit v
    };
    
    p.map(|b| Ast::Bool(b.parse::<bool>().expect("Parsed bool fails parse::<bool>()")))
}

fn key(s : &'static str) -> Parser<()> {
    let not_sym_char = || peek().when(|c| !c.is_digit(10) && *c != '_' && !c.is_alphabetic());

    trim!( compute!{bind, unit => 
        _keyword <- exact(s);
        _i <- not_sym_char();
        unit () 
    } )
}

fn junk() -> Parser<()> {
    any().when(|c| c.is_whitespace()).map(|_x| ())
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::output::Output;
    use super::super::input::Input;
    use monad::compute;

    #[test]
    fn key_combinator_should_fail_illegitimate_target() {
        let p = key("blah");
        let mut input = Input::new("  blahx  ");

        let v = p.parse(&mut input);

        assert!(matches!( v, Output::Failure(_)));
    }

    #[test]
    fn key_combinator_should_parse_legit_target() {
        let p = key("blah");
        let mut input = Input::new("  blah  ");

        let v = p.parse(&mut input);

        assert!(matches!( v, Output::Success(_, _, _)));
    }

    #[test]
    fn number_literal_should_parse_positive_value() {
        let p = number_literal();
        let mut input = Input::new("1234");

        let v = p.parse(&mut input);

        assert!(matches!( v, Output::Success(Ast::Integer(1234), _, _)));
    }

    #[test]
    fn bool_literal_should_parse_true() {
        let p = bool_literal();
        let mut input = Input::new("true ");

        let v = p.parse(&mut input);

        match v {
            Output::Success(Ast::Bool(b), _, _) => assert_eq!( b, true ),
            it @ _ => panic!( "unexpected output: {:?}", it ),
        }
    }

    #[test]
    fn bool_literal_should_parse_false() {
        let p = bool_literal();
        let mut input = Input::new("false ");

        let v = p.parse(&mut input);

        match v {
            Output::Success(Ast::Bool(b), _, _) => assert_eq!( b, false ),
            it @ _ => panic!( "unexpected output: {:?}", it ),
        }
    }

    #[test]
    fn bool_literal_should_not_parse_symbol_starting_with_bool() {
        let p = bool_literal();
        let mut input = Input::new("falsey");

        let v = p.parse(&mut input);

        assert!(matches!(v, Output::Failure(_)));
    }
}