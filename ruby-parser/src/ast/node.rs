use super::*;
use crate::lexer::*;

#[derive(Debug, PartialEq)]
pub enum Node {
    Conditional(Conditional),
    LogicalAnd(LogicalAnd),
    LogicalOr(LogicalOr),
    LogicalNot(LogicalNot),
    Literal(Literal),
    Identifier(Identifier),
    Interpolated(Interpolated),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Block(Vec<Self>),
    BlockArg(Box<Self>),
    Segment(Segment),
    Comment(String),
    Ranged(Ranged),
    Defined(Box<Self>),
    Splat(Box<Self>),
    Array(Vec<Self>),
    Hash(Vec<Self>),
    Nil,
    Self_,
    EndOfProgram,
    Placeholder,
}

#[allow(dead_code)]
impl Node {
    /// Creates a token that represents an empty block
    pub(crate) fn empty() -> Self {
        Self::Block(vec![])
    }
    /// Creates a token that represents a boolean value
    pub(crate) fn boolean(val: bool) -> Self {
        Self::Literal(Literal::Boolean(val))
    }
    /// Creates a token that represents an integer value
    pub(crate) fn int(val: isize) -> Self {
        Self::Literal(Literal::Integer(val))
    }
    /// Creates a token that represents a float value
    pub(crate) fn float(val: f64) -> Self {
        Self::Literal(Literal::Float(val))
    }
    /// Creates a token that represents a literal string
    pub(crate) fn literal_string(val: &str) -> Self {
        Self::Literal(Literal::String(val.to_owned()))
    }
    /// Creates a token that represents a float value
    pub(crate) fn literal_symbol(val: &str) -> Self {
        Self::Literal(Literal::Symbol(val.to_owned()))
    }
    /// Creates a token that represents an identifier
    pub(crate) fn ident(name: &str, kind: IdentifierKind) -> Self {
        Self::Identifier(Identifier {
            name: name.to_owned(),
            kind,
        })
    }
    /// Creates a token that represents a unary operation
    pub(crate) fn unary_op(op: UnaryOpKind, rhs: Node) -> Self {
        Self::UnaryOp(UnaryOp {
            op,
            rhs: Box::new(rhs),
        })
    }
    /// Creates a token that represents a binary operation
    pub(crate) fn binary_op(lhs: Node, op: BinaryOpKind, rhs: Node) -> Self {
        Self::BinaryOp(BinaryOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
    /// Creates a token that represents a logical AND
    pub(crate) fn logical_and(first: Node, second: Node) -> Self {
        Self::LogicalAnd(LogicalAnd {
            first: Box::new(first),
            second: Box::new(second),
        })
    }
    /// Creates a token that represents a logical OR
    pub(crate) fn logical_or(first: Node, second: Node) -> Self {
        Self::LogicalOr(LogicalOr {
            first: Box::new(first),
            second: Box::new(second),
        })
    }
    /// Creates a token that represents a logical NOT
    pub(crate) fn logical_not(expr: Node) -> Self {
        Self::LogicalNot(LogicalNot {
            expr: Box::new(expr),
        })
    }
    /// Creates a token that reprents a defined? statement
    pub(crate) fn defined(node: Node) -> Self {
        Self::Defined(Box::new(node))
    }
    /// Creates a token that reprents a splat argument
    pub(crate) fn splat(node: Node) -> Self {
        Self::Splat(Box::new(node))
    }
    /// Creates a token that reprents a splat argument
    pub(crate) fn block_arg(node: Node) -> Self {
        Self::BlockArg(Box::new(node))
    }
    /// Creates a token that reprents an array constructor
    pub(crate) fn array(node: Vec<Node>) -> Self {
        Self::Array(node)
    }
    /// Creates a token that reprents a hash constructor
    pub(crate) fn hash(node: Vec<Node>) -> Self {
        Self::Hash(node)
    }
    /// Creates a token that reprents a range
    pub(crate) fn range(from: Node, to: Node, exclusive: bool) -> Self {
        Self::Ranged(Ranged {
            from: Box::new(from),
            to: Box::new(to),
            exclusive,
        })
    }
    /// Creates a token that represents a conditional statement
    pub(crate) fn conditional(
        kind: ConditionalKind,
        cond: Node,
        then: Node,
        otherwise: Node,
    ) -> Node {
        Self::Conditional(Conditional {
            kind,
            cond: Box::new(cond),
            then: Box::new(then),
            otherwise: Box::new(otherwise),
        })
    }
    /// Allows placeholding nodes to be updated when working around left-recursion via LL(2)
    pub(crate) fn update_placeholder(value: Node, ast: Option<Node>) -> Node {
        if let Some(mut parent_node) = ast {
            use std::borrow::BorrowMut;
            {
                let mut n = &mut parent_node;
                loop {
                    match n {
                        Node::Conditional(sub) => {
                            n = match sub.kind {
                                ConditionalKind::ModifyingIf | ConditionalKind::ModifyingUnless => {
                                    sub.then.borrow_mut()
                                }
                                _ => sub.cond.borrow_mut(),
                            }
                        }
                        Node::BinaryOp(sub) => n = sub.lhs.borrow_mut(),
                        Node::LogicalOr(sub) => n = sub.first.borrow_mut(),
                        Node::LogicalAnd(sub) => n = sub.first.borrow_mut(),
                        Node::LogicalNot(sub) => n = sub.expr.borrow_mut(),
                        _ => break,
                    }
                }
                *n = value;
            }
            parent_node
        } else {
            value
        }
    }
}
