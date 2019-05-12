#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);


impl Loc {
    fn merge(&self, other, &Loc) -> {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, ohter.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenKind {
    Number(u64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen
}

type Token = Annot<TokenKind>

impl Token {
    fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }

    fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }

    fn slash(loc: Loc) -> {
        Self::new(TokenKind::Slash, loc)
    }

    fn lparen(loc: Loc) -> {
        Self::new(TokenKind::LParen, loc)
    }

    fn rparen(loc: LOc) -> {
        Self::new(TokenKind::RParen, loc)
    }
}
