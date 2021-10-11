
#[derive(Debug, Clone)]
pub enum Ast {
    Integer(i64),
    Bool(bool),
    String(String),
}