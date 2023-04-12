#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenKind {
    Integer,
    Float,
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Power,
    LParen,
    RParen,
    Identifier,
    Comment,
    Spaces,
    SemiColon,
    LineBreak,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

pub fn tokenize_line(line: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut token = Token {
        kind: TokenKind::Spaces,
        value: String::new(),
    };

    for c in line.chars() {
        if token.kind == TokenKind::Comment {
            token.value.push(c);
            continue;
        } else {
            if c == ' ' {
                if token.kind != TokenKind::Spaces {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Spaces,
                        value: String::new(),
                    };
                }
            } else if c == ';' {
                if token.kind != TokenKind::Spaces {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Spaces,
                        value: String::new(),
                    };
                }
                token.kind = TokenKind::SemiColon;
                token.value.push(c);
            } else if c == '=' {
                if token.kind != TokenKind::Spaces {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Spaces,
                        value: String::new(),
                    };
                }
                token.kind = TokenKind::Assign;
                token.value.push(c);
            } else if c == '+' {
                if token.kind != TokenKind::Spaces {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Spaces,
                        value: String::new(),
                    };
                }
                token.kind = TokenKind::Plus;
                token.value.push(c);
            } else if c == '-' {
                if token.kind == TokenKind::Spaces {
                    token.kind = TokenKind::Minus;
                    token.value.push(c);
                } else if token.kind == TokenKind::Float && token.value.contains('e') || token.value.contains('E') {
                    token.value.push(c);
                } else {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Spaces,
                        value: String::new(),
                    };
                }
            } else if c == '*' {
                if token.kind != TokenKind::Spaces {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Spaces,
                        value: String::new(),
                    };
                }
                token.kind = TokenKind::Asterisk;
                token.value.push(c);
            } else if c == '/' {
                if token.kind == TokenKind::Spaces {
                    token.kind = TokenKind::Slash;
                    token.value.push(c);
                } else if token.kind == TokenKind::Slash {
                    token.kind = TokenKind::Comment;
                    token.value.push(c);
                } else {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Spaces,
                        value: String::new(),
                    };
                }
            } else if c == '%' {
                if token.kind != TokenKind::Spaces {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Spaces,
                        value: String::new(),
                    };
                }
                token.kind = TokenKind::Percent;
                token.value.push(c);
            } else if c == '^' {
                if token.kind != TokenKind::Spaces {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Spaces,
                        value: String::new(),
                    };
                }
                token.kind = TokenKind::Power;
                token.value.push(c);
            } else if c == '(' {
                if token.kind != TokenKind::Spaces {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Spaces,
                        value: String::new(),
                    };
                }
                token.kind = TokenKind::LParen;
                token.value.push(c);
            } else if c == ')' {
                if token.kind != TokenKind::Spaces {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Spaces,
                        value: String::new(),
                    };
                }
                token.kind = TokenKind::RParen;
                token.value.push(c);
            } else if c.is_digit(10) {
                if token.kind == TokenKind::Spaces {
                    token.kind = TokenKind::Integer;
                    token.value.push(c);
                } else if token.kind == TokenKind::Integer {
                    token.value.push(c);
                } else if token.kind == TokenKind::Float {
                    token.value.push(c);
                } else {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Integer,
                        value: String::new(),
                    };
                    token.value.push(c);
                }
            } else if c == '.' {
                if token.kind == TokenKind::Integer {
                    token.kind = TokenKind::Float;
                    token.value.push(c);
                } else {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Float,
                        value: String::new(),
                    };
                    token.value.push(c);
                }
            } else if c.is_alphabetic() || c == '_' {
                if token.kind == TokenKind::Spaces {
                    token.kind = TokenKind::Identifier;
                    token.value.push(c);
                } else if token.kind == TokenKind::Identifier {
                    token.value.push(c);
                } else if token.kind == TokenKind::Float {
                    if c == 'e' || c == 'E' {
                        token.value.push(c);
                    }
                } else {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Identifier,
                        value: String::new(),
                    };
                    token.value.push(c);
                }
            } else if c == '\n' || c == '\r' || c == '\t' {
                if token.kind != TokenKind::Spaces {
                    tokens.push(token);
                    token = Token {
                        kind: TokenKind::Spaces,
                        value: String::new(),
                    };
                }
                token.kind = TokenKind::LineBreak;
                token.value.push(c);
            } else {
                tokens.push(token);
                token = Token {
                    kind: TokenKind::Spaces,
                    value: String::new(),
                };
            }
        }
    }
    if token.kind != TokenKind::Spaces {
        tokens.push(token);
    }
    tokens
}