use std::io;
use std::io::BufRead;
use std::str;
use std::iter;
use std::collections::HashMap;
use simplelang::*;
use simplelang::indexed_slice::*;

type LexSlice<'a> = IndexedSlice<'a, char>;



pub struct Lexer {
    keyword_mapping: HashMap<&'static str, TokenKind>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            keyword_mapping: map! {
				"fn" => TokenKind::FnKeyword
			},
        }
    }

    pub fn lex<TIn>(&self, input: TIn) -> SyntaxResult<Vec<Token>>
    where
        TIn: io::Read,
    {
        let input = io::BufReader::new(input);
        let mut position = TextPosition { index: 0 };
        let mut tokens = Vec::new();

        for line in input.lines() {
            let line: Vec<_> = line.expect("unable to read input file").chars().collect();
            let mut slice = IndexedSlice::from_chars(&line);
            self.line(&mut slice, &mut tokens)?;
        }

        return Ok(tokens);
    }

    fn line(&self, line: &mut LexSlice, result: &mut Vec<Token>) -> SyntaxResult<()> {

        while let Some(c) = line.first() {
            let c = *c;

            if is_whitespace(c) {
                line.pop_first();
                continue;
            }

            let token = if is_identifier_start(c) {
                let id_token = self.identifier(line)?;

                if let Some(kind) = self.keyword_mapping.get(&id_token.text[..]) {
                    Token {
                        kind: *kind,
                        ..id_token
                    }
                } else {
                    id_token
                }
            } else if is_digit(c) {
                self.integer(line)?
            } else if is_quote(c) {
                self.string(line)?
            } else {

                let startpos = TextPosition { index: line.position() };

                let next_char = line.try_get(1).map(|c| *c);
                let mut op_token = |str: &str, kind| {

                    line.pop_first();

                    if str.len() == 2 {
                        line.pop_first(); // consume last char
                    }

                    let endpos = TextPosition { index: line.position() };

                    Token {
                        start: startpos,
                        end: endpos,
                        text: str.to_string(),
                        kind: kind,
                    }
                };

                match (c, next_char) {
                    ('+', Some('+')) => op_token("++", TokenKind::PlusPlus),
                    ('+', Some('=')) => op_token("+=", TokenKind::PlusEqual),
                    ('-', Some('-')) => op_token("--", TokenKind::MinusMinus),
                    ('-', Some('=')) => op_token("-=", TokenKind::MinusEqual),
                    ('*', Some('=')) => op_token("*=", TokenKind::AsteriskEqual),
                    ('+', _) => op_token("+", TokenKind::Plus),
                    ('-', _) => op_token("-", TokenKind::Minus),
                    ('*', _) => op_token("*", TokenKind::Asterisk),
                    ('(', _) => op_token("(", TokenKind::LParen),
                    (')', _) => op_token("(", TokenKind::RParen),
                    _ => {
                        return SyntaxError::at_pos(
                            startpos.index,
                            format!("Unexpected symbol '{}'", c),
                        )
                    }
                }
            };

            result.push(token);
        }

        return Ok(());
    }

    fn identifier(&self, line: &mut LexSlice) -> SyntaxResult<Token> {

        let startpos = TextPosition { index: line.position() };

        let mut s = String::new();
        s.push(consume_char(line, is_identifier_start)?);
        consume_while(line, is_identifier_body, &mut s);

        let endpos = TextPosition { index: line.position() };

        Ok(Token {
            text: s,
            start: startpos,
            end: endpos,
            kind: TokenKind::Identifier,
        })
    }

    fn integer(&self, line: &mut LexSlice) -> SyntaxResult<Token> {

        let startpos = TextPosition { index: line.position() };

        let mut s = String::new();
        consume_while(line, is_digit, &mut s);

        let endpos = TextPosition { index: line.position() };
        dump!(endpos, line.to_string());

        Ok(Token {
            text: s,
            start: startpos,
            end: endpos,
            kind: TokenKind::Integer,
        })
    }

    fn string(&self, line: &mut LexSlice) -> SyntaxResult<Token> {

        let startpos = TextPosition { index: line.position() };

        let mut s = String::new();
        s.push(consume_char(line, is_quote)?);
        consume_while(line, is_string_body, &mut s);
        s.push(consume_char(line, is_quote)?);

        let endpos = TextPosition { index: line.position() };

        Ok(Token {
            text: s,
            start: startpos,
            end: endpos,
            kind: TokenKind::String,
        })
    }
}

fn consume_char(slice: &mut LexSlice, predicate: fn(char) -> bool) -> SyntaxResult<char> {

    match slice.pop_first() {
        None => SyntaxError::at_pos(slice.position(), "Unexpected EOF".to_owned()),
        Some(c) if predicate(*c) => Ok(*c),
        _ => SyntaxError::at_pos(slice.position(), "Invalid symbol".to_owned()),
    }
}

fn consume_while(slice: &mut LexSlice, predicate: fn(char) -> bool, s: &mut String) {
    while let Some(c) = slice.first() {
        if predicate(*c) {
            s.push(*slice.pop_first().unwrap());
        } else {
            break;
        }
    }
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\n' || c == '\t'
}

fn is_digit(c: char) -> bool {
    ('0' <= c && c <= '9')
}

fn is_identifier_start(c: char) -> bool {
    ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || (c == '_')
}

fn is_identifier_body(c: char) -> bool {
    is_identifier_start(c) || is_digit(c)
}

fn is_quote(c: char) -> bool {
    return c == '"';
}

fn is_string_body(c: char) -> bool {
    !is_quote(c)
}
