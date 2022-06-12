use crate::func::*;

pub enum TokenKind {
    /// Literal "sin"
    Sin,
    /// Literal "cos"
    Cos,
    /// Literal "tan"
    Tan,
    /// Literal "asin"
    Asin,
    /// Literal "acos"
    Acos,
    /// Literal "atan"
    Atan,
    /// Literal "e"
    E,
    /// Literal "ln"
    Ln,
    /// Literal "log"
    Log,
    /// Literal "^"
    Pow,
    /// Literal "sqrt"
    Sqrt,
    /// Literal "qbrt"
    Qbrt,
    /// "nthrt" Num
    Nthrt,
    /// Literal "*"
    Times,
    /// Literal "/"
    Over,
    /// Literal "+"
    Plus,
    // TODO: avoid negative number collision
    /// Literal "-"
    Minus,
    /// "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z"
    Var,
    /// Literal _
    SubIx,
    /// Literal "("
    Lpar,
    /// Literal ")"
    Rpar,
    /// L((L*)|(N*))*
    Name,
    /// NN*(.NN*)?(E-?NN*)?
    Const,
}

pub struct Token {
    kind: TokenKind,
    content: String,
}

impl Token {
    pub fn new(kind: TokenKind, content: String) -> Self {
        Self { kind, content }
    }
}

pub struct Tokenizer {
    raw_func: String,
}

impl Tokenizer {
    pub fn new(raw_func: String) -> Self {
        Self { raw_func }
    }

    pub fn process(&self) -> Vec<Token> {
        let mut container: Vec<Token> = Vec::new();
        container.push(Token::new(TokenKind::E, String::from("e")));
        container
    }
}

pub enum LexicalUnit {
    /// Name Lpar Var Rpar => "f(x)"
    Func,
}