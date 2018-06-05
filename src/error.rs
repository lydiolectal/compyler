#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedStartOfToken(char),
}
