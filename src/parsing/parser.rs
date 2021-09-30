
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
            Output::Failure => { input.restore(rp); Output::Failure },
            Output::Fatal(index) => Output::Fatal(index),
        }
    }))
}

pub fn unit<T : Clone>( t : T ) -> Parser<T> {
    Parser::Unit(t)
}

pub fn map<A : 'static + Clone, B : 'static + Clone>( p : Parser<A>, f : fn(A) -> B ) -> Parser<B> {
    Parser::Parse(Box::new(move |input| {
        let rp = input.restore_point();
        match p.parse(input) {
            Output::Success(item, start, end) => Output::Success(f(item), start, end), 
            Output::Failure => { input.restore(rp); Output::Failure },
            Output::Fatal(index) => Output::Fatal(index),
        }
    }))
}


impl<T : 'static + Clone> Parser<T> {
    pub fn parse(&self, input : &mut Input) -> Output<T> {
        match self {
            Parser::Parse(p) => p(input),
            Parser::Unit(t) => Output::Success(t.clone(), 0, 0), // TODO start and end?
            _ => panic!("blarg"),
        }
    }

    pub fn when(self, pred : fn(&T) -> bool) -> Parser<T> {
        Parser::Parse(Box::new(move |input| {
            let rp = input.restore_point();
            match self.parse(input) {
                Output::Success(v, start, end) if pred(&v) => Output::Success(v, start, end),
                Output::Success(v, _, _) => { input.restore(rp); Output::Failure },
                Output::Failure => { input.restore(rp); Output::Failure },
                Output::Fatal(index) => Output::Fatal(index),
            }
        }))
    }

    pub fn maybe(self) -> Parser<Option<T>> {
        Parser::Parse(Box::new(move |input| {
            let rp = input.restore_point();
            let index = match input.peek() {
                Ok((i, _)) => i,
                Err(i) => i,
            };
            match self.parse(input) {
                Output::Success(v, start, end) => Output::Success(Some(v), start, end),
                Output::Failure => { input.restore(rp); Output::Success(None, index, index) },
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
                    Output::Failure => { input.restore(rp); break },
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
                Output::Failure => { input.restore(rp); return Output::Failure },
                Output::Fatal(index) => return Output::Fatal(index),
            }

            loop {
                let rp = input.restore_point();

                match self.parse(input) {
                    Output::Success(v, start, end) => items.push(v), // TODO start/end ?
                    Output::Failure => { input.restore(rp); break },
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
                Output::Failure => { input.restore(rp); other.parse(input) },
                Output::Fatal(index) => Output::Fatal(index),
            }
        }))
    }
}

#[cfg(test)]
mod test {
}