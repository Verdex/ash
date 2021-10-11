
use super::parser::{Parser, bind, unit, exact, any, peek, the};
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

/*
  private Parser<Expr> StrParser() {
            static Parser<char> EscapeParser() 
                => (from slash in Expect("\\")
                   from other in Expect("t")
                                 .Or(Expect("n"))
                                 .Or(Expect("r"))
                                 .Or(Expect("\\"))
                                 .Or(Expect("\""))
                   select other).Select( c => c switch {
                       "t" => '\t',
                       "n" => '\n',
                       "r" => '\r',
                       "\\" => '\\',
                       "\"" => '"',
                       _ => throw new Exception("Impossible escape character encountered"),
                   });

            static Parser<char> NotQuote() 
                => from c in Any()
                   where c != '"'
                   select c;

            return (from q1 in DoubleQuote() 
                   from cs in EscapeParser().Or(NotQuote()).ZeroOrMore()
                   from q2 in DoubleQuote() 
                   select new Str(new string(cs.ToArray())) as Expr).Trim();
        }
*/

fn string_literal() -> Parser<Ast> {

    fn escape_parser() -> Parser<char> {
        compute!{bind, unit => 
            _slash <- the('\\');
            other <- the('n').or(the('r'))
                             .or(the('t'))
                             .or(the('\\'))
                             .or(the('"'));
            unit other
        }
    }

    unit(Ast::String("blarg".to_string()))
}

fn key(s : &'static str) -> Parser<&'static str> {
    let not_sym_char = || peek().when(|c| !c.is_digit(10) && *c != '_' && !c.is_alphabetic());

    trim!( compute!{bind, unit => 
        _keyword <- exact(s);
        _i <- not_sym_char();
        unit s
    } )
}

fn punct(s : &'static str) -> Parser<&'static str> {
    trim!( exact(s) )
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
    fn key_parser_should_fail_illegitimate_target() {
        let p = key("blah");
        let mut input = Input::new("  blahx  ");

        let v = p.parse(&mut input);

        assert!(matches!( v, Output::Failure(_)));
    }

    #[test]
    fn key_parser_should_parse_legit_target() {
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