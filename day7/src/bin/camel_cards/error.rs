#[derive(Debug)]
pub enum Error {
    HandNotParsableError,
    LabelCountError(String),
}
