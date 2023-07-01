use crate::token_type::TokenType;

pub struct Token<T> {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<T>,
    pub line: usize,
}
