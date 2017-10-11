#[macro_use]
pub mod macros;
pub mod indexed_slice;
pub mod lexer;
pub mod parser;
pub mod ast;

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub text: String,
    pub start: TextPosition,
    pub end: TextPosition,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TextPosition {
    pub index: usize,
}

impl TextPosition {
    fn next(&self) -> Self {
        TextPosition { index: self.index + 1 }
    }
}

static UNKNOWN_POSITION: TextPosition = TextPosition { index: 0 };

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TokenKind {
    Identifier,
    Integer,
    String,

    // keywords
    FnKeyword,

    // symbols
    Plus,
    PlusPlus,
    PlusEqual,
    Minus,
    MinusMinus,
    MinusEqual,
    Asterisk,
    AsteriskEqual,
    LParen,
    RParen,
}

impl TokenKind {
    fn as_prefix_operator(self) -> Option<ast::UnaryOperator> {
        match self {
            TokenKind::Plus => Some(ast::UnaryOperator::Plus),
            TokenKind::Minus => Some(ast::UnaryOperator::Negate),
            TokenKind::PlusPlus => Some(ast::UnaryOperator::PreInc),
            TokenKind::MinusMinus => Some(ast::UnaryOperator::PreDec),
            _ => None,
        }
    }

    fn as_postfix_operator(self) -> Option<ast::UnaryOperator> {
        match self {
            TokenKind::PlusPlus => Some(ast::UnaryOperator::PostInc),
            TokenKind::MinusMinus => Some(ast::UnaryOperator::PostDec),
            _ => None,
        }
    }
}

pub type SyntaxResult<T> = Result<T, SyntaxError>;

#[derive(Debug)]
pub struct SyntaxError {
    start: TextPosition,
    end: TextPosition,
    message: String,
}

impl SyntaxError {
    fn at_pos<T>(position: TextPosition, message: String) -> SyntaxResult<T> {
        Err(SyntaxError {
            start: position,
            end: position,
            message: message,
        })
    }

    fn at_range<T>(start: TextPosition, end: TextPosition, message: String) -> SyntaxResult<T> {
        Err(SyntaxError {
            start: start,
            end: end,
            message: message,
        })
    }

    fn from_token<T>(token: &Token, message: String) -> SyntaxResult<T> {
        Err(SyntaxError {
            start: token.start,
            end: token.end,
            message,
        })
    }
}

trait PrimaryKey<T> {
    fn primary_key(&self) -> T;
}

trait HashMapPrimaryKeyExt<K, V> {
    fn insert_key(&mut self, v: V) -> Option<V>;
}

impl<K, V> HashMapPrimaryKeyExt<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
    V: PrimaryKey<K>,
{
    fn insert_key(&mut self, v: V) -> Option<V> {
        self.insert(v.primary_key(), v)
    }
}
