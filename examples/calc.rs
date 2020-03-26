use generic_lexer::{Lexer, LexerInput};

#[derive(Debug)]
enum TokenKind {
    Int, Float,
    Name,
    Plus, Minus, Star, Slash, Semicolon, Equals,
}

fn lex_int(input: &mut LexerInput) -> TokenKind {
    input.next_while(u8::is_ascii_digit);
    if input.accept(b'.') {
        return lex_float(input);
    }
    TokenKind::Int
}

fn lex_float(input: &mut LexerInput) -> TokenKind {
    input.next_while(u8::is_ascii_digit);
    TokenKind::Float
}

fn lex_name(input: &mut LexerInput) -> TokenKind {
    input.next_while(|&b| b == b'_' || b.is_ascii_alphabetic());
    TokenKind::Name
}

fn lex(byte: u8, input: &mut LexerInput) -> Result<TokenKind, String> {
    let kind = match byte {
        b'+' => TokenKind::Plus,
        b'-' => TokenKind::Minus,
        b'*' => TokenKind::Star,
        b'/' => TokenKind::Slash,
        b';' => TokenKind::Semicolon,
        b'=' => TokenKind::Equals,

        b if b.is_ascii_digit() => lex_int(input),
        b if b.is_ascii_alphabetic() => lex_name(input),
        _ => return Err(format!("Unknown byte {}", char::from(byte)))
    };

    Ok(kind)
}

fn main() -> Result<(), String> {
    let input = "a = 420 + 69 * 3.14;";

    let mut lexer = Lexer::new(&input, lex, true);

    while let Some(token) = lexer.next() {
        let token = token?;
        println!("{:?}", token);
    }

    Ok(())
}