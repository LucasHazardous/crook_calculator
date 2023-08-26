#[derive(Debug, PartialEq)]
pub enum Element {
    Number(f64),
    Operation(char),
    OpenBracket,
    CloseBracket
}
