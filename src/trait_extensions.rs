pub trait IdentifierChar {
    fn is_ascii_identifier_char(&self) -> bool;
}

impl IdentifierChar for char {
    fn is_ascii_identifier_char(&self) -> bool {
        self.is_ascii_alphabetic() || *self == '_'
    }
}
