#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Ident,
    Keyword,
    Int,
    Operator,
    Char,
    String,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    literal: &'a str,
    kind: TokenKind,
}

impl<'a> Token<'a> {
    pub fn new(literal: &'a str, kind: TokenKind) -> Self {
        Self { literal, kind }
    }
}

pub struct Tokenizer {
    source: String,
    state: usize,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Self { source, state: 0 }
    }

    pub fn next(&mut self) -> Option<Token> {
        while self.state < self.source.len() {
            if let Some(c) = self.get_current_state() {
                match c {
                    '#' | ';' | '{' | '}' | '(' | ')' => {
                        self.state += 1;
                        return Some(Token::new(
                            &self.source[self.state - 1..self.state],
                            TokenKind::Char,
                        ));
                    },
                    _ if c.is_alphabetic() => {
                        let idx_start = self.state;
                        while let Some(ci) = self.get_current_state() {
                            if ci.is_alphanumeric() {
                                self.state += 1;
                            } else {
                                break;
                            }
                        }
                        return Some(Token::new(
                            &self.source[idx_start..self.state],
                            TokenKind::Ident,
                        ));
                    },
                    _ if c.is_numeric() => {
                        let idx_start = self.state;
                        while let Some(ci) = self.get_current_state() {
                            if ci.is_numeric() {
                                self.state += 1;
                            } else {
                                break;
                            }
                        }
                        return Some(Token::new(
                            &self.source[idx_start..self.state],
                            TokenKind::Int,
                        ));
                    },
                    _ => (),
                }
                self.state += 1;
            }
        }
        None
    }

    /// Reads out state and returns the char at the particular index as an Option type.
    fn get_current_state(&mut self) -> Option<char> {
        self.source.chars().nth(self.state)
    }
}

/// Takes in an iterator over the chars of a string slice from index
/// and yields all elements next that are alphabetic chars.
fn _yield_alphabetical(source: &mut String, state: usize) -> String {
    source
        .chars()
        .skip(state)
        .take_while(|c| c.is_alphabetic())
        .collect()
}

/// Takes in an iterator over the chars of a string slice from index
/// and yields all elements next that are numerical chars.
fn _yield_numerical(source: &mut String, state: usize) -> String {
    source
        .chars()
        .skip(state)
        .take_while(|c| c.is_numeric())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parser::token::Tokenizer;

    #[test]
    fn test_get_ident() {
        let s = "int main()".to_string();
        let mut tokenizer = Tokenizer::new(s);

        assert_eq!("int", tokenizer.next().unwrap().literal);
        assert_eq!("main", tokenizer.next().unwrap().literal);
        assert_eq!("(", tokenizer.next().unwrap().literal);
        assert_eq!(")", tokenizer.next().unwrap().literal);
    }
}