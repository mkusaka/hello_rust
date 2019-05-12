#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
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
    RParen,
}

type Token = Annot<TokenKind>;

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

    fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }

    fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }

    fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

type LexError = Annot<LexErrorKind>;

impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }

    fn eof(loc: Loc) -> Self {
        LexError::new(LexErrorKind::Eof, loc)
    }
}

fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let input = input.as_bytes();

    let mut position = 0;

    macro_rules! lex_a_token {
        ($lexer:exper) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok)
            position = p
        }};
    }
    while position < input.len() {
        match input[position] {
            b'0'...b'9' => lex_a_token!(lex_number(input, position)),
            b'+' => lex_a_token!(lex_plus(input, position)),
            b'-' => lex_a_token!(lex_minus(input, position)),
            b'*' => lex_a_token!(lex_asterisk(input, position)),
            b'/' => lex_a_token!(lex_slash(input, position)),
            b'(' => lex_a_token!(lex_lparen(input, position)),
            b')' => lex_a_token!(lex_rparen(input, position)),
            b' ' | b'\n' | b'\t' => {
                let ((), p) = skip_spaces(input, position)?;
                position = p;
            },
            b => return Err(LexError::invalid_char(b as char, Loc(position, position + 1))),
        }
    }
    Ok(tokens)
}

fn consume_byte(input: &[u8], position: usize, b: u8) -> Result<(u8, usize), LexError> {
    if has_finished(input, position) {
        return Err(LexError::eof(Loc(pos, pos)));
    }

    if input != b {
        return Err(LexError::invalid_char(
            input[pos] as char,
            Loc(pos, pos + 1),

        ));
    }

    Ok((b, pos+1))
}

fn has_finished(input: &[u8], position: usize) -> bool {
    input.len() <= position
}

fn lex_plus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    // Token::plus undef?
    consume_byte(input, start, b'+').map(|(_, end)| (Token::plus(Loc(start, end)), end))
}

fn lex_minus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    // Token::minus undef?
    consume_byte(input, start, b'-').map(|(_, end)| (Token::minus(Loc(start, end)), end))
}

fn lex_asterisk(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    // Token::asterisk undef?
    consume_byte(input, start, b'*').map(|(_, end)| (Token::asterisk(Loc(start, end)), end))
}

fn lex_slash(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    // Token::slash undef?
    consume_byte(input, start, b'/').map(|(_, end)| (Token::slash(Loc(start, end)), end))
}

fn lex_lparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    // Token::lparen undef?
    consume_byte(input, start, b'(').map(|(_, end)| (Token::lparen(Loc(start, end)), end))
}

fn lex_rparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    // Token::rparen undef?
    consume_byte(input, start, b')').map(|(_, end)| (Token::rparen(Loc(start, end)), end))
}

// it is natural that return Result format. maybe
fn lex_number(input: &[u8], mut position: usize) -> (Token, usize) {
    use std::str::from_utf8

    let start = position;

    while position < input.len() && b"1234567890".contains(&input[position]) {
        position += 1;
    }

    let n = from_utf8(&input[start..position])
                .unwrap()
                .parse()
                .unwrap();
    (Token::number(n, Loc(start, position)), position)
}

fn skip_spaces(input: &[u8], mut position: usize) -> ((), usize) {
    while position < input.len() && b"\n\t".contains(&input[position]) {
        position += 1;
    }

    ((), position)
}
