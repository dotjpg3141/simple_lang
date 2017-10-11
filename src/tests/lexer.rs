use simplelang::*;
use simplelang::lexer::Lexer;

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
    assert_token("+", TokenKind::Plus);
    assert_token("++", TokenKind::PlusPlus);
    assert_token("+=", TokenKind::PlusEqual);
}

#[test]
fn whitespace() {
    assert_tokens(" \r \t \n ", Vec::<Token>::new());
    assert_tokens(" 123 ", vec![new_token(1, "123", TokenKind::Integer)]);
}

#[test]
fn multiple_tokens() {

    assert_tokens(
        "+++",
        vec![
            new_token(0, "++", TokenKind::PlusPlus),
            new_token(2, "+", TokenKind::Plus),
        ],
    );

}

fn assert_tokens(input: &str, expected: Vec<Token>) {
    let lexer = Lexer::new();
    let actual = lexer.lex(input.as_bytes()).unwrap();
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
