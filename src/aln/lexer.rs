use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    At,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(String),
    BoolLiteral(bool),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub offset: usize,
}

#[derive(Debug, Error)]
pub enum LexError {
    #[error("Unexpected character '{ch}' at {offset}")]
    UnexpectedChar { ch: char, offset: usize },
    #[error("Unterminated string starting at {offset}")]
    UnterminatedString { offset: usize },
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((idx, ch)) = chars.next() {
        match ch {
            '@' => tokens.push(Token { kind: TokenKind::At, offset: idx }),
            '{' => tokens.push(Token { kind: TokenKind::LBrace, offset: idx }),
            '}' => tokens.push(Token { kind: TokenKind::RBrace, offset: idx }),
            '[' => tokens.push(Token { kind: TokenKind::LBracket, offset: idx }),
            ']' => tokens.push(Token { kind: TokenKind::RBracket, offset: idx }),
            ':' => tokens.push(Token { kind: TokenKind::Colon, offset: idx }),
            ',' => tokens.push(Token { kind: TokenKind::Comma, offset: idx }),
            '"' | '\'' => {
                let start_quote = ch;
                let start = idx;
                let mut value = String::new();
                loop {
                    match chars.next() {
                        Some((_, c)) if c == start_quote => break,
                        Some((_, c)) => value.push(c),
                        None => {
                            return Err(LexError::UnterminatedString { offset: start });
                        }
                    }
                }
                tokens.push(Token {
                    kind: TokenKind::StringLiteral(value),
                    offset: start,
                });
            }
            ch if ch.is_whitespace() => {}
            ch if ch.is_ascii_alphabetic() || ch == '_' => {
                let mut ident = String::new();
                ident.push(ch);
                while let Some((_, c)) = chars.peek().copied() {
                    if c.is_ascii_alphanumeric() || c == '_' || c == '.' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let kind = match ident.as_str() {
                    "true" => TokenKind::BoolLiteral(true),
                    "false" => TokenKind::BoolLiteral(false),
                    _ => TokenKind::Identifier(ident),
                };
                tokens.push(Token { kind, offset: idx });
            }
            ch if ch.is_ascii_digit() => {
                let mut num = String::new();
                num.push(ch);
                while let Some((_, c)) = chars.peek().copied() {
                    if c.is_ascii_digit() || c == '.' {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    kind: TokenKind::NumberLiteral(num),
                    offset: idx,
                });
            }
            other => {
                return Err(LexError::UnexpectedChar {
                    ch: other,
                    offset: idx,
                });
            }
        }
    }

    Ok(tokens)
}
