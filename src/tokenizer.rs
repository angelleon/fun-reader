use crate::func::*;

#[derive(Copy, Clone)]
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
    /// DD*(.DD*)?(E-?DD*)?
    Const,
    Error,
    Empty
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
        const F: i32 = -1;
        const E: i32 = -2;
        let alphabet = ['-', '+', '=', '(', ')', '_', '\'', '^', 'e', 'E'];
        let transition_matrix = [
        //   D  L  .  -  +  =  (  )  _  '  ^  e  E  LAM
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 00
            [1, 0, 2, F, F, F, F, F, F, F, F, F, F,   F], // 01
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 02
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 03
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 04
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 05
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 06
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 07
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 08
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 09
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 10
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 11
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 12
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 13
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 14
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   0], // 15
        ];
        let tkn_clasif = [
            TokenKind::Empty, // 00
            TokenKind::Const, // 01 
        ];
        let mut position = 0usize;
        loop {
            if position == self.raw_func.len() {
                break;
            }
            position += 1;
        }
        container.push(Token::new(TokenKind::E, String::from("e")));
        container
    }
}

pub enum LexicalUnit {
    /// Name Lpar Var Rpar => "f(x)"
    Func,
}