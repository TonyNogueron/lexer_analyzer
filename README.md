# Lexer Analyzer
Creates a lexical analyzer and syntax highlight for Implementation of Computational Methods class.

### How to use it
The code is made in rust and it takes a text file as an input. "test.txt" with simple code lines and tokenizes them as:

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
Then the code produces html output that syntax highlights and shows the table of tokens and their values.

### txt file:
```txt
b=7

a = 32.4 *(-8.6 - b)/       6.1E-8

d = a ^ b // Esto es un comentario
```

### Output Example:
![image](https://user-images.githubusercontent.com/89329480/228707832-3ee5abd0-cd99-46db-bbad-304cb3667311.png)

