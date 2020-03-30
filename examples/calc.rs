use generic_lexer::{Lexer, BufferedInput, MatchError};

#[derive(Debug)]
enum TokenKind {
    Int, Float,
    Name,
    Plus, Minus, Star, Slash, Semicolon, Equals,
}

fn lex_int(input: &mut BufferedInput) -> TokenKind {
    input.accept_while(char::is_ascii_digit);
    if let Some(_) = input.accept_if(|c| *c == '.') {
        return lex_float(input);
    }
    TokenKind::Int
}

fn lex_float(input: &mut BufferedInput) -> TokenKind {
    input.accept_while(char::is_ascii_digit);
    TokenKind::Float
}

fn lex_name(input: &mut BufferedInput) -> TokenKind {
    input.accept_while(|c| *c == '_' || c.is_ascii_alphabetic());
    TokenKind::Name
}

fn lex(first_char: char, input: &mut BufferedInput) -> Result<TokenKind, MatchError> {
    let kind = match first_char {
        '+' => TokenKind::Plus,
        '-' => TokenKind::Minus,
        '*' => TokenKind::Star,
        '/' => TokenKind::Slash,
        ';' => TokenKind::Semicolon,
        '=' => TokenKind::Equals,

        c if c.is_ascii_digit() => lex_int(input),
        c if c.is_ascii_alphabetic() => lex_name(input),

        c => return Err(MatchError::Unexpected(c))
    };

    Ok(kind)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "a = 420 + 69 * 3.14;";
    let lexer = Lexer::new(&input, &lex, true);
    let tokens = lexer.collect::<Result<Vec<_>, _>>()?;
    println!("{:#?}", tokens);
    Ok(())
}
