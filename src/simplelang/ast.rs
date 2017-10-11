#[derive(Debug, PartialEq)]
pub enum ExpressionNode {
    Integer(i32),
    String(String),
    Binary(BinaryOperator, Box<ExpressionNode>, Box<ExpressionNode>),
    Unary(UnaryOperator, Box<ExpressionNode>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator {
    Mul,
    Add,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Negate,
    PreInc,
    PostInc,
    PreDec,
    PostDec,
}
