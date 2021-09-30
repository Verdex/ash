

// TODO monad will provide an AND behavior
// TODO Or will provide an OR behavior
// TODO in either case a Failure response just means that you need to stop
// TODO but a Fatal(usize) means that you need to stop everything and pass up the failure
use super::input::Input;
use super::output::Output;

pub enum Parser<T> {
    Parse(fn(&mut Input) -> Output<T>),
    Where(Box<Parser<T>>, fn(&T) -> bool),
    Exactly(&'static str),
    Maybe(Box<Parser<T>>),
    ZeroOrMore(Box<Parser<T>>),
    OneOrMore(Box<Parser<T>>),
    Unit(T),
}

