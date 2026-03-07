use std::collections::HashMap;
use std::sync::OnceLock;

use super::Lexer;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number,
    Identifier,
    Equals,
    Let,
    Const,
    OpenParen,
    CloseParen,
    SemiColon,
    // BinaryOperator,
    AdittiveOperator,
    MultiplicitaveOperator,
    EOF,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) value: String,
    pub(crate) token_type: TokenType,
}

impl Token {
    fn new(value: String, token_type: TokenType) -> Token {
        Token { token_type, value }
    }
}

static KEYWORDS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();
fn keywords() -> &'static HashMap<&'static str, TokenType> {
    KEYWORDS.get_or_init(|| {
        let mut map: HashMap<&str, TokenType> = HashMap::new();
        map.insert("let", TokenType::Let);
        map.insert("const", TokenType::Const);
        map
    })
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer: Lexer = Lexer::new(source_code);
    while !lexer.is_eof() {
        if let Some(c) = lexer.next() {
            if c == ' ' || c == '\n' || c == '\t' || c == '\r' {
                continue;
            } else if c == '(' {
                tokens.push(Token::new(String::from(c), TokenType::OpenParen));
            } else if c == ')' {
                tokens.push(Token::new(String::from(c), TokenType::CloseParen));
            } else if c == ';' {
                tokens.push(Token::new(String::from(c), TokenType::SemiColon));
            } else if c == '+' || c == '-' {
                tokens.push(Token::new(String::from(c), TokenType::AdittiveOperator));
            } else if c == '*' || c == '/' || c == '%' {
                tokens.push(Token::new(
                    String::from(c),
                    TokenType::MultiplicitaveOperator,
                ));
            } else if c == '=' {
                tokens.push(Token::new(String::from(c), TokenType::Equals));
            } else {
                // Multi-char code
                if c.is_ascii_digit() {
                    let number: &str = lexer.lex_number();
                    tokens.push(Token::new(number.to_string(), TokenType::Number));
                } else if c.is_ascii_alphabetic() {
                    let ident: &str = lexer.lex_word();
                    if let Some(e) = keywords().get(ident) {
                        tokens.push(Token::new(ident.to_string(), e.clone()));
                    } else {
                        tokens.push(Token::new(ident.to_string(), TokenType::Identifier));
                    }
                } else {
                    panic!("Unreconized character found in the source: {:?}", c);
                }
            }
        }
    }
    tokens.push(Token::new(String::from("EndOfFile"), TokenType::EOF));
    tokens
}
