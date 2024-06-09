#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyAttributeError,
    NodeNameError,
    StateError,
}
