
use super::parser::{Parser, bind, unit, map, exact, any, peek};
use super::ast::Ast;
    
use monad::compute;

pub fn parse() -> Vec<Ast> {

    vec![]
}

fn number_literal() -> Parser<Ast> {
    // TODO also need to handle floats
    // TODO also need to handle negative
    let p = any().when(|d| d.is_digit(10) ).one_or_more();
    
    map(p, |ds| Ast::Integer(
        ds.into_iter()
            .collect::<String>()
            .parse::<i64>()
            .expect("Parsed integer fails parse::<i64>()")))
} 

#[cfg(test)]
mod test {
    use super::*;
    use super::super::output::Output;
    use super::super::input::Input;
    use monad::compute;

    #[test]
    fn number_literal_should_parse_positive_value() {
        let p = number_literal();
        let mut input = Input::new("1234");

        let v = p.parse(&mut input);

        assert!(matches!( v, Output::Success(Ast::Integer(1234), _, _)));
    }
}