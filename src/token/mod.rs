pub use self::token_type::TokenType;

mod token_type;

#[derive(Debug)]
pub struct Token {
    r#type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(r#type: TokenType, literal: String) -> Token {
        Token { r#type, literal }
    }

    pub fn from_char(r#type: TokenType, ch: char) -> Token {
        Token { r#type, literal: ch.into() }
    }
}
