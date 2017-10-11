use std::io;
use std::io::BufRead;
use std::str;
use std::iter;
use std::collections::HashMap;
use simplelang::*;

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
            let line = line.expect("unable to read input file");
            position = self.line(line.chars(), &mut tokens, position)?;
        }

        return Ok(tokens);
    }

    fn line(
        &self,
        line: str::Chars,
        result: &mut Vec<Token>,
        position: TextPosition,
    ) -> SyntaxResult<TextPosition> {

        let mut line = line.peekable();
        let mut position = position;

        while let Some(c) = line.next() {

            if is_whitespace(c) {
                position = position.next();
                continue;
            }

            let token = if is_identifier_start(c) {
                let id_token = self.identifier(c, &mut line, position)?;

                if let Some(kind) = self.keyword_mapping.get(&id_token.text[..]) {
                    Token {
                        kind: *kind,
                        ..id_token
                    }
                } else {
                    id_token
                }
            } else if is_digit(c) {
                self.integer(c, &mut line, position)?
            } else if is_quote(c) {
                self.string(c, &mut line, position)?
            } else {

                let next_char = line.peek().map(|c| *c);
                let mut op_token = |str: &str, kind| {

                    let endpos = match str.len() {
                        1 => position.next(),
                        2 => {
                            line.next(); // consume last char
                            position.next().next()
                        }
                        _ => panic!(),
                    };

                    Token {
                        start: position,
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
                    _ => return SyntaxError::at_pos(position, format!("Unexpected symbol '{}'", c)),
                }
            };

            position = token.end;
            result.push(token);
        }

        return Ok(position);
    }

    fn identifier<T>(
        &self,
        start: char,
        line: &mut iter::Peekable<T>,
        startpos: TextPosition,
    ) -> SyntaxResult<Token>
    where
        T: iter::Iterator<Item = char>,
    {
        let mut s: String = start.to_string();
        let endpos = read_while(line, &mut s, startpos.next(), is_identifier_body)?;
        Ok(Token {
            text: s,
            start: startpos,
            end: endpos,
            kind: TokenKind::Identifier,
        })
    }

    fn integer<T>(
        &self,
        start: char,
        line: &mut iter::Peekable<T>,
        startpos: TextPosition,
    ) -> SyntaxResult<Token>
    where
        T: iter::Iterator<Item = char>,
    {
        let mut s: String = start.to_string();
        let endpos = read_while(line, &mut s, startpos.next(), is_digit)?;

        Ok(Token {
            text: s,
            start: startpos,
            end: endpos,
            kind: TokenKind::Integer,
        })
    }

    fn string<T>(
        &self,
        start: char,
        line: &mut iter::Peekable<T>,
        startpos: TextPosition,
    ) -> SyntaxResult<Token>
    where
        T: iter::Iterator<Item = char>,
    {
        let is_string_body = |c| !is_quote(c);
        let mut s: String = start.to_string();
        let mut endpos = read_while(line, &mut s, startpos.next(), is_string_body)?;

        endpos.index += 1;

        if let Some(c) = line.next() {
            if is_quote(c) {
                s.push(c);
                Ok(Token {
                    text: s,
                    start: startpos,
                    end: endpos,
                    kind: TokenKind::String,
                })
            } else {
                panic!("unexpected program state")
            }
        } else {
            SyntaxError::at_range(startpos, endpos, format!("Unclosed string"))
        }
    }
}

fn read_while<T, P>(
    input: &mut iter::Peekable<T>,
    output: &mut String,
    pos: TextPosition,
    predicated: P,
) -> SyntaxResult<TextPosition>
where
    T: iter::Iterator<Item = char>,
    P: Fn(char) -> bool,
{
    let mut pos = pos;

    loop {

        if input.peek().map(|c| predicated(*c)) != Some(true) {
            break;
        }

        output.push(input.next().unwrap());

        pos = pos.next();
    }
    return Ok(pos);
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
