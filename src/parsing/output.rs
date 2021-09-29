
pub enum Output<T> {
    Success(T, usize, usize),
    Failure,
    Fatal(usize),
}