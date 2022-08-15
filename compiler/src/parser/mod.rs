pub mod keywords;

use crate::parser::keywords::KEYWORDS;

#[derive(Debug)]
pub enum TokenKind {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
    Keyword,
    Equal,
    Int,
    Char,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}

pub struct Parser {
    source: String,
    state: usize,
}

impl Parser {
    pub fn new(source: String, state: usize) -> Self {
        Self { source, state }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.source.len() > self.state {
            if let Some(c) = self.get_current_char() {
                match c {
                    '=' => tokens.push(Token::new(TokenKind::Equal, "=".to_owned())),
                    ';' => tokens.push(Token::new(TokenKind::Semicolon, ";".to_owned())),
                    '{' => tokens.push(Token::new(TokenKind::OpenBrace, "{".to_owned())),
                    '}' => tokens.push(Token::new(TokenKind::CloseBrace, "}".to_owned())),
                    '(' => tokens.push(Token::new(TokenKind::OpenParen, "(".to_owned())),
                    ')' => tokens.push(Token::new(TokenKind::CloseParen, ")".to_owned())),
                    _ if c.is_alphabetic() => {
                        let str_l = self.yield_alphabetical();
                        self.state += &str_l.len() - 1;

                        if KEYWORDS.contains(&str_l.as_str()) {
                            tokens.push(Token::new(TokenKind::Keyword, str_l))
                        }
                    }
                    _ if c.is_numeric() => {
                        let num_l = self.yield_numerical();
                        self.state += &num_l.len() - 1;
                        tokens.push(Token::new(TokenKind::Int, num_l));
                    }
                    _ => (),
                }
                self.state += 1;
            } else {
                break;
            }
        }
        return tokens;
    }

    /// Reads out state and returns the char at this particular index as an Option type.
    fn get_current_char(&mut self) -> Option<char> {
        self.source.chars().nth(self.state)
    }
    /// Takes in an iterator over the chars of a string slice from index
    /// and yields all elements next that are alphabetic chars.
    fn yield_alphabetical(&mut self) -> String {
        self.source
            .chars()
            .skip(self.state)
            .take_while(|c| c.is_alphabetic())
            .collect()
    }

    /// Takes in an iterator over the chars of a string slice from index
    /// and yields all elements next that are numerical chars.
    fn yield_numerical(&mut self) -> String {
        self.source
            .chars()
            .skip(self.state)
            .take_while(|c| c.is_numeric())
            .collect()
    }
}
