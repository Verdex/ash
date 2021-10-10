
#[derive(Debug)]
pub enum Output<T> {
    Success(T, usize, usize),
    Failure(usize),
    Fatal(usize),
}