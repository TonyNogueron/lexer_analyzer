# Lexer Analyzer
Creates a lexical analyzer and syntax highlight for Implementation of Computational Methods class.

### How to use it
The code is made in rust and it takes a text file as an input. "expressions.txt" with simple code lines and tokenizes them as:

```rust
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
```
