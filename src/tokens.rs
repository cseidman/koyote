
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Token {
    pub tokenType: TokenType,
    pub line: usize,
    pub name: String,
    pub label: String,
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    T_LEFT_PAREN , T_RIGHT_PAREN, T_LEFT_BRACE, T_RIGHT_BRACE, T_LEFT_BRACKET,
    T_RIGHT_BRACKET,T_COMMA,T_DOT,T_MINUS,T_PLUS,
    T_SEMICOLON,T_SLASH,T_STAR,T_COLON,

    // One or two character tokens.
    T_BANG, T_BANG_EQUAL,T_EQUAL, T_EQUAL_EQUAL,T_GREATER,
    T_GREATER_EQUAL,T_LESS, T_LESS_EQUAL, T_DOUBLE_COLON,

    // Literals.
    T_IDENTIFIER, T_STRING,T_FLOAT, T_INTEGER,T_BOOL,

    // Keywords.
    T_AND, T_CLASS, T_ELSE, T_FALSE, T_FOR,
    T_FUN, T_IF, T_NIL, T_OR,T_PRINT, T_RETURN,
    T_SUPER, T_THIS,T_TRUE, T_VAR, T_WHILE,

    // System level keywords
    T_ERROR, T_EOF, T_START, T_CR
}

use self::TokenType::* ;

pub static TokenList: [TokenType;48] =
[T_LEFT_PAREN, T_RIGHT_PAREN, T_LEFT_BRACE,
T_RIGHT_BRACE, T_LEFT_BRACKET, T_RIGHT_BRACKET,
T_COMMA, T_DOT, T_MINUS,
T_PLUS, T_SEMICOLON, T_SLASH,
T_STAR, T_COLON, T_BANG,
T_BANG_EQUAL, T_EQUAL, T_EQUAL_EQUAL,
T_GREATER, T_GREATER_EQUAL, T_LESS,
T_LESS_EQUAL, T_DOUBLE_COLON, T_IDENTIFIER, T_STRING,
T_FLOAT, T_INTEGER, T_BOOL,
T_AND, T_CLASS, T_ELSE,
T_FALSE, T_FOR, T_FUN,
T_IF, T_NIL, T_OR,
T_PRINT, T_RETURN, T_SUPER,
T_THIS, T_TRUE, T_VAR,
T_WHILE, T_ERROR, T_EOF, T_START, T_CR] ;

