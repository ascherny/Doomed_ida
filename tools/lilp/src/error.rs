#[derive(Debug)]
pub enum ParseError {
    OutOfBounds,
    InvalidMagic,
    Unsupported
}
