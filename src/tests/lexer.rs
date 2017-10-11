use simplelang::*;
use simplelang::lexer;

#[test]
fn empty() {
    assert_tokens("", Vec::<Token>::new());
}

#[test]
fn tokens() {
    assert_token("123", TokenKind::Integer);
    assert_token("\"Hello, World!\"", TokenKind::String);
    assert_token("hello", TokenKind::Identifier);
    assert_token("fn", TokenKind::FnKeyword);
    assert_token("fnabc", TokenKind::Identifier);
    assert_token("abcfn", TokenKind::Identifier);
}

#[test]
fn op_tokens() {
    assert_token("+", TokenKind::Plus);
    assert_token("++", TokenKind::PlusPlus);
    assert_token("+=", TokenKind::PlusEqual);
    assert_tokens(
        "+++",
        vec![
            new_token(0, "++", TokenKind::PlusPlus),
            new_token(2, "+", TokenKind::Plus),
        ],
    );
}

#[test]
fn whitespace() {
    assert_tokens(" \r \t \n ", Vec::<Token>::new());
    assert_tokens(" 123 ", vec![new_token(1, "123", TokenKind::Integer)]);
    assert_tokens(
        "12 34 ",
        vec![
            new_token(0, "12", TokenKind::Integer),
            new_token(3, "34", TokenKind::Integer),
        ],
    );
}

fn assert_tokens(input: &str, expected: Vec<Token>) {
    let actual = lexer::lex(input.as_bytes()).unwrap();
    assert_eq!(expected, actual);
}

fn assert_token(input: &str, kind: TokenKind) {
    let token = new_token(0, input, kind);
    assert_tokens(input, vec![token]);
}

fn new_token(start: usize, text: &str, kind: TokenKind) -> Token {
    Token {
        start: TextPosition { index: start },
        end: TextPosition { index: start + text.len() },
        text: text.to_owned(),
        kind: kind,
    }
}
