use simplelang::*;
use simplelang::ast::*;

#[test]
fn expr_integer() {
    assert_eq!(parse_expr("123"), ExpressionNode::Integer(123));
}

#[test]
fn expr_string() {
    assert_eq!(
        parse_expr("\"hello\""),
        ExpressionNode::String("hello".to_string())
    );
}

#[test]
fn expr_parenthesis() {
    assert_eq!(parse_expr("(456)"), ExpressionNode::Integer(456));
}

#[test]
fn expr_binary() {

    assert_eq!(
        parse_expr("12 + 34"),
        ExpressionNode::Binary(
            BinaryOperator::Add,
            Box::new(ExpressionNode::Integer(12)),
            Box::new(ExpressionNode::Integer(34)),
        )
    );
}

#[test]
fn expr_binary_precendence() {

    assert_eq!(
        parse_expr("12 + 34 * 56"),
        ExpressionNode::Binary(
            BinaryOperator::Add,
            Box::new(ExpressionNode::Integer(12)),
            Box::new(ExpressionNode::Binary(
                BinaryOperator::Mul,
                Box::new(ExpressionNode::Integer(34)),
                Box::new(ExpressionNode::Integer(56)),
            )),
        )
    );

    assert_eq!(
        parse_expr("12 * 34 + 56"),
        ExpressionNode::Binary(
            BinaryOperator::Add,
            Box::new(ExpressionNode::Binary(
                BinaryOperator::Mul,
                Box::new(ExpressionNode::Integer(12)),
                Box::new(ExpressionNode::Integer(34)),
            )),
            Box::new(ExpressionNode::Integer(56)),
        )
    );
}

#[test]
fn expr_binary_associativity() {

    assert_eq!(
        parse_expr("12 + 34 + 56"),
        ExpressionNode::Binary(
            BinaryOperator::Add,
            Box::new(ExpressionNode::Binary(
                BinaryOperator::Add,
                Box::new(ExpressionNode::Integer(12)),
                Box::new(ExpressionNode::Integer(34)),
            )),
            Box::new(ExpressionNode::Integer(56)),
        )
    );
}

#[test]
fn expr_unary_prefix() {
    assert_eq!(
        parse_expr("+-5"),
        ExpressionNode::Unary(
            UnaryOperator::Plus,
            Box::new(ExpressionNode::Unary(
                UnaryOperator::Negate,
                Box::new(ExpressionNode::Integer(5)),
            )),
        )
    );
}

#[test]
fn expr_unary_postfix() {
    assert_eq!(
        parse_expr("5--++"),
        ExpressionNode::Unary(
            UnaryOperator::PostInc,
            Box::new(ExpressionNode::Unary(
                UnaryOperator::PostDec,
                Box::new(ExpressionNode::Integer(5)),
            )),
        )
    );
}

fn parse_expr(input: &str) -> ExpressionNode {
    let tokens = lexer::lex(input.as_bytes()).expect(
        "Lex error",
    );
    let (tokens, expr) = parser::expression(&tokens).expect("Parse error");
    assert_eof(tokens);
    expr
}

fn assert_eof(tokens: &[Token]) {
    assert!(
        tokens.len() == 0,
        "Expected end of file got tokens {:?}",
        tokens.iter().map(|t| &t.text).collect::<Vec<&String>>()
    );
}
