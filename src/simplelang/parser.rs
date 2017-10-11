use simplelang::*;
use simplelang::ast::*;
use std::collections::HashMap;

lazy_static! {
	static ref OPERATORS: HashMap<TokenKind, OperatorInfo> = {
		let mut map = HashMap::new();

        map.insert_key(OperatorInfo {
            token_kind: TokenKind::Asterisk,
            operator: BinaryOperator::Mul,
            precedence: 6,
        });

        map.insert_key(OperatorInfo {
            token_kind: TokenKind::Plus,
            operator: BinaryOperator::Add,
            precedence: 5,
        });

		map
	};
}

type ParseResult<'a, T> = SyntaxResult<(&'a [Token], T)>;

pub fn expression<'a>(tokens: &'a [Token]) -> ParseResult<'a, ExpressionNode> {
    let (tokens, lhs) = primary_expression(tokens)?;
    binary_rhs_expression(tokens, lhs, 0)
}

fn binary_rhs_expression<'a>(
    tokens: &'a [Token],
    lhs: ExpressionNode,
    min_precedence: u32,
) -> ParseResult<'a, ExpressionNode> {

    let mut tokens = tokens;
    let mut lhs = lhs;

    while let Some((ts, op_info)) = read_binary_operator(tokens, min_precedence) {
        let (ts, mut rhs) = primary_expression(ts)?;
        let next_precedence = op_info.precedence + 1;

        if read_binary_operator(ts, next_precedence).is_some() {
            assign!{ (tokens, rhs) = binary_rhs_expression(ts, rhs, next_precedence)? };
        } else {
            tokens = ts;
        }

        lhs = ExpressionNode::Binary(op_info.operator, Box::new(lhs), Box::new(rhs));
    }

    return Ok((tokens, lhs));
}

fn read_binary_operator<'a>(
    tokens: &'a [Token],
    min_precedence: u32,
) -> Option<(&'a [Token], &OperatorInfo)> {

    if let Some((t, ts)) = tokens.split_first() {
        if let Some(operator) = OPERATORS.get(&t.kind) {
            if operator.precedence >= min_precedence {
                return Some((ts, operator));
            }
        }
    }
    return None;
}

fn primary_expression<'a>(tokens: &'a [Token]) -> ParseResult<'a, ExpressionNode> {
    let (mut t, mut ts) = pop_first(tokens)?;

    let mut prefix_operators = Vec::new();
    while let Some(unary) = t.kind.as_prefix_operator() {
        prefix_operators.push(unary);
        assign!{ (t, ts) = pop_first(ts)? };
    }

    let mut exp = match t.kind {
        TokenKind::Integer => {
            let value = t.text.parse::<i32>().unwrap();
            ExpressionNode::Integer(value)
        }
        TokenKind::LParen => {
            assign!{ (ts, let exp) = expression(ts)? };

            match ts.first() {
                Some(&Token { kind: TokenKind::RParen, .. }) => {
                    ts = &ts[1..];
                    exp
                }
                Some(token) => return SyntaxError::from_token(token, "Missing ')'".to_string()),
                None => return SyntaxError::from_token(t, "Expected ')' at EOF".to_string()),
            }
        }
        TokenKind::String => {
            let value = t.text[1..t.text.len() - 1].to_string();
            ExpressionNode::String(value)
        }
        _ => return SyntaxError::from_token(&t, format!("Expected literal or '('")),
    };

    // unary postfix operators
    while let Some(unary) = ts.first().and_then(|t| t.kind.as_postfix_operator()) {
        exp = ExpressionNode::Unary(unary, Box::new(exp));
        ts = &ts[1..];
    }

    for unary in prefix_operators.iter().rev() {
        exp = ExpressionNode::Unary(*unary, Box::new(exp));
    }

    Ok((ts, exp))
}

fn pop_first(tokens: &[Token]) -> SyntaxResult<(&Token, &[Token])> {
    tokens.split_first().ok_or_else(|| {
        let pos = UNKNOWN_POSITION;
        SyntaxError {
            start: pos,
            end: pos,
            message: format!("Unexpected EOF"),
        }
    })
}

#[derive(Debug)]
struct OperatorInfo {
    token_kind: TokenKind,
    operator: BinaryOperator,
    precedence: u32,
}

impl PrimaryKey<TokenKind> for OperatorInfo {
    fn primary_key(&self) -> TokenKind {
        self.token_kind
    }
}
