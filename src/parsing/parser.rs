
use super::input::Input;
use super::output::Output;

pub enum Parser<T : 'static + Clone> {
    Parse(Box<dyn Fn(&mut Input) -> Output<T>>),
    Unit(T),
}

pub fn bind<A : 'static + Clone, B : 'static + Clone>( pa : Parser<A>, next : impl Fn(A) -> Parser<B> + 'static ) -> Parser<B> {
    Parser::Parse(Box::new(move |input| {
        let rp = input.restore_point();
        match pa.parse(input) {
            Output::Success(item, start, end) => next(item).parse(input), // TODO start/end needs to be fed into item ?
            Output::Failure(index) => { input.restore(rp); Output::Failure(index) },
            Output::Fatal(index) => Output::Fatal(index),
        }
    }))
}

pub fn unit<T : Clone>( t : T ) -> Parser<T> {
    Parser::Unit(t)
}

pub fn exact<'a>(s : &'a str) -> Parser<&'a str> {
    Parser::Parse(Box::new(move |input| {
        match input.exact(s) {
            Ok((start, end, value)) => Output::Success(value, start, end),
            Err(index) => Output::Failure(index),
        }
    }))
}

pub fn the(c : char) -> Parser<char> {
    Parser::Parse(Box::new(move |input| {
        let rp = input.restore_point();
        match input.get_char() {
            Ok((index, value)) if c == value => Output::Success(value, index, index),
            Ok((index, _)) => { input.restore(rp); Output::Failure(index) },
            Err(index) => Output::Failure(index),
        }
    }))
}

pub fn any() -> Parser<char> {
    Parser::Parse(Box::new(move |input| {
        match input.get_char() {
            Ok((index, value)) => Output::Success(value, index, index),
            Err(index) => Output::Failure(index),
        }
    }))
}

pub fn peek() -> Parser<char> {
    Parser::Parse(Box::new(move |input| {
        match input.peek() {
            Ok((index, value)) => Output::Success(value, index, index),
            Err(index) => Output::Failure(index),
        }
    }))
}

impl<T : 'static + Clone> Parser<T> {
    pub fn new(parser : impl Fn(&mut Input) -> Output<T> + 'static) -> Parser<T> {
        Parser::Parse(Box::new(parser))
    }

    pub fn fatal(self) -> Parser<T> {
        Parser::Parse(Box::new(move |input| {
            match self.parse(input) {
                it @ Output::Success(_, _, _) => it,
                Output::Failure(index) => Output::Fatal(index),
                Output::Fatal(index) => Output::Fatal(index),
            }
        }))
    }
    
    pub fn parse(&self, input : &mut Input) -> Output<T> {
        match self {
            Parser::Parse(p) => p(input),
            Parser::Unit(t) => Output::Success(t.clone(), 0, 0), // TODO start and end?
        }
    }

    pub fn map<B : 'static + Clone>( self, f : fn(T) -> B ) -> Parser<B> {
        Parser::Parse(Box::new(move |input| {
            let rp = input.restore_point();
            match self.parse(input) {
                Output::Success(item, start, end) => Output::Success(f(item), start, end), 
                Output::Failure(index) => { input.restore(rp); Output::Failure(index) },
                Output::Fatal(index) => Output::Fatal(index),
            }
        }))
    }

    pub fn when(self, pred : fn(&T) -> bool) -> Parser<T> {
        Parser::Parse(Box::new(move |input| {
            let rp = input.restore_point();
            match self.parse(input) {
                Output::Success(v, start, end) if pred(&v) => Output::Success(v, start, end),
                Output::Success(v, index, _) => { input.restore(rp); Output::Failure(index) },
                Output::Failure(index) => { input.restore(rp); Output::Failure(index) },
                Output::Fatal(index) => Output::Fatal(index),
            }
        }))
    }

    pub fn maybe(self) -> Parser<Option<T>> {
        Parser::Parse(Box::new(move |input| {
            let rp = input.restore_point();
            match self.parse(input) {
                Output::Success(v, start, end) => Output::Success(Some(v), start, end),
                Output::Failure(index) => { input.restore(rp); Output::Success(None, index, index) },
                Output::Fatal(index) => Output::Fatal(index),
            }
        }))
    }

    pub fn zero_or_more(self) -> Parser<Vec<T>> {
        Parser::Parse(Box::new(move |input| {

            let mut items : Vec<T> = vec![];

            loop {
                let rp = input.restore_point();

                match self.parse(input) {
                    Output::Success(v, start, end) => items.push(v), // TODO start/end ?
                    Output::Failure(_) => { input.restore(rp); break },
                    Output::Fatal(index) => return Output::Fatal(index),
                }
            }

            Output::Success(items, 0, 0) // TODO start/end ?
        }))
    }

    pub fn one_or_more(self) -> Parser<Vec<T>> {
        Parser::Parse(Box::new(move |input| {

            let mut items : Vec<T> = vec![];

            let rp = input.restore_point();

            match self.parse(input) {
                Output::Success(v, start, end) => items.push(v), // TODO start/end ?
                Output::Failure(index) => { input.restore(rp); return Output::Failure(index) },
                Output::Fatal(index) => return Output::Fatal(index),
            }

            loop {
                let rp = input.restore_point();

                match self.parse(input) {
                    Output::Success(v, start, end) => items.push(v), // TODO start/end ?
                    Output::Failure(_) => { input.restore(rp); break },
                    Output::Fatal(index) => return Output::Fatal(index),
                }
            }

            Output::Success(items, 0, 0) // TODO start/end ?
        }))
    }

    pub fn or(self, other : Parser<T>) -> Parser<T> {

        Parser::Parse(Box::new(move |input| {

            let rp = input.restore_point();

            match self.parse(input) {
                Output::Success(v, start, end) => Output::Success(v, start, end), 
                Output::Failure(_) => { input.restore(rp); other.parse(input) },
                Output::Fatal(index) => Output::Fatal(index),
            }
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use monad::compute;

}