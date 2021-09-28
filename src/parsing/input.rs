use std::str::CharIndices;

pub struct Input<'a> {
    cs : CharIndices<'a>,
    total_length : usize,
}

pub struct RestorePoint<'a> {
    cs : CharIndices<'a>,
    total_length : usize,
}

impl<'a> Input<'a> {

    pub fn new(s : &'a str) -> Input<'a> {
        Input { cs: s.char_indices()
              , total_length: s.len()
              }
    }

    pub fn restore_point(&self) -> RestorePoint<'a> {
        RestorePoint { cs: self.cs.clone()
                     , total_length: self.total_length
                     }
    }

    pub fn restore(&mut self, rp : RestorePoint<'a>) {
        self.cs = rp.cs;
    }

    pub fn get_char(&mut self) -> Result<(usize, char), usize> {
        match self.cs.next() {
            Some(c) => Ok(c),
            None => Err(self.total_length),
        }
    }

    pub fn peek(&mut self) -> Result<(usize, char), usize> {
        let rp = self.restore_point();

        match self.get_char() {
            it @ Ok(_) => { self.restore(rp); it },
            it @ Err(_) => it,
        }
    }

    pub fn exact<'b>(&mut self, s : &'b str) -> Result<(&'b str, usize, usize), usize> {

        let (start, _) = self.peek()?;

        let mut n = self.cs.clone();

        let mut end = 0;
        for c in s.chars() {
            match n.next() {
                Some((index, target)) if c == target => { end = index }, 
                Some((index, _)) => return Err(index),
                None => return Err(self.total_length),
            }
        }

        self.cs = n;
        Ok((s, start, end))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_char_should_return_char_and_index() {
        let mut input = Input::new("string");

        let c = input.get_char().expect("Should be able to get 's'");
        assert_eq!( (0, 's'), c );

        let c = input.get_char().expect("Should be able to get 't'");
        assert_eq!( (1, 't'), c );

        let c = input.get_char().expect("Should be able to get 'r'");
        assert_eq!( (2, 'r'), c );

        let c = input.get_char().expect("Should be able to get 'i'");
        assert_eq!( (3, 'i'), c );

        let c = input.get_char().expect("Should be able to get 'n'");
        assert_eq!( (4, 'n'), c );

        let c = input.get_char().expect("Should be able to get 'g'");
        assert_eq!( (5, 'g'), c );
    }

    #[test]
    fn get_char_returns_failure_index() {
        assert!( false );
    }

    #[test]
    fn exact_failure_should_not_change_index() {
        let mut input = Input::new("string");

        let result = input.exact("yy");

        assert!( matches!( result, Err(_) ) );

        let result = input.exact("string");

        assert!( matches!( result, Ok(_) ) );
    }

    #[test]
    fn exact_success_should_change_index() {
        let mut input = Input::new("string");

        let result = input.exact("st");

        assert!( matches!( result, Ok(_) ) );

        let result = input.exact("ring");

        assert!( matches!( result, Ok(_) ) );
    }

    #[test]
    fn exact_returns_index_on_failure() {
        assert!( false );
    }

    #[test]
    fn exact_returns_target_string() {
        assert!( false );
    }

    #[test]
    fn exact_returns_correct_start_index() {
        assert!( false );
    }

    #[test]
    fn exact_returns_correct_end_index() {
        assert!( false );
    }
    
    #[test]
    fn peek_error_returns_index() {
        assert!( false );
    }

    #[test]
    fn peek_success_returns_index_and_value() {
        assert!( false );
    }

    #[test]
    fn peek_success_does_not_increase_index() {
        assert!( false );
    }
}
