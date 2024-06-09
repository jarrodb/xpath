#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyAttributeError,
    EmptyPathError,
    SyntaxError,
    StateError,
}
