use crate::ast::{BinaryOp, BinaryOpToken};
use crate::lexer::*;
use crate::parsers::expression::unary::{unary_expression, unary_minus_expression};
use crate::parsers::program::{no_lt, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::combinator::map;
use nom::sequence::tuple;

/// *relational_expression* | *relational_expression* [ no line terminator here ] ( `<=>` | `===` | `==` | `!=` | `=~` | `!~` ) *relational_expression*
pub(crate) fn equality_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                relational_expression,
                no_lt,
                alt((
                    tag("<=>"),
                    tag("==="),
                    tag("=="),
                    tag("!="),
                    tag("=~"),
                    tag("!~"),
                )),
                ws,
                relational_expression,
            )),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: match *t.2 {
                        "<=>" => BinaryOpToken::Compare,
                        "===" => BinaryOpToken::CaseEqual,
                        "==" => BinaryOpToken::Equal,
                        "!=" => BinaryOpToken::NotEqual,
                        "=~" => BinaryOpToken::RegexMatch,
                        "!~" => BinaryOpToken::NotRegexMatch,
                        _ => unreachable!(),
                    },
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        relational_expression,
    ))(i)
}

/// *bitwise_or_expression* | *relational_expression* [ no line terminator here ] ( `>=` | `>` | `<=` | `<` ) *bitwise_or_expression*
pub(crate) fn relational_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                relational_expression,
                no_lt,
                alt((tag(">="), tag(">"), tag("<="), tag("<"))),
                ws,
                bitwise_or_expression,
            )),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: match *t.2 {
                        ">=" => BinaryOpToken::GreaterEqual,
                        ">" => BinaryOpToken::GreaterThan,
                        "<=" => BinaryOpToken::LessEqual,
                        "<" => BinaryOpToken::LessThan,
                        _ => unreachable!(),
                    },
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        bitwise_or_expression,
    ))(i)
}

/// *bitwise_and_expression* | *bitwise_or_expression* [ no line terminator here ] ( `|` | `^` ) *bitwise_and_expression*
pub(crate) fn bitwise_or_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                bitwise_or_expression,
                no_lt,
                one_of("|^"),
                ws,
                bitwise_and_expression,
            )),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: match t.2 {
                        '|' => BinaryOpToken::BitOr,
                        '^' => BinaryOpToken::BitXor,
                        _ => unreachable!(),
                    },
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        bitwise_and_expression,
    ))(i)
}

/// *bitwise_shift_expression* | *bitwise_and_expression* [ no line terminator here ] `&` *bitwise_shift_expression*
pub(crate) fn bitwise_and_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                bitwise_and_expression,
                no_lt,
                char('&'),
                ws,
                bitwise_shift_expression,
            )),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: BinaryOpToken::BitAnd,
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        bitwise_shift_expression,
    ))(i)
}

/// *additive_expression* | *bitwise_shift_expression* [ no line terminator here ] ( `<<` | `>>` ) *additive_expression*
pub(crate) fn bitwise_shift_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                bitwise_shift_expression,
                no_lt,
                alt((tag("<<"), tag(">>"))),
                ws,
                additive_expression,
            )),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: match *t.2 {
                        "<<" => BinaryOpToken::ShiftLeft,
                        ">>" => BinaryOpToken::ShiftRight,
                        _ => unreachable!(),
                    },
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        additive_expression,
    ))(i)
}

/// *multiplicative_expression* | *additive_expression* [ no line terminator here ] ( `+` | `-` ) *multiplicative_expression*
pub(crate) fn additive_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                additive_expression,
                no_lt,
                one_of("+-"),
                ws,
                multiplicative_expression,
            )),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: match t.2 {
                        '+' => BinaryOpToken::Add,
                        '-' => BinaryOpToken::Subtract,
                        _ => unreachable!(),
                    },
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        multiplicative_expression,
    ))(i)
}

/// *unary_minus_expression* | *multiplicative_expression* [ no line terminator here ] ( `*` | `/` | `%` ) *unary_minus_expression*
pub(crate) fn multiplicative_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                multiplicative_expression,
                no_lt,
                one_of("*/%"),
                ws,
                unary_minus_expression,
            )),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: match t.2 {
                        '*' => BinaryOpToken::Multiply,
                        '/' => BinaryOpToken::Divide,
                        '%' => BinaryOpToken::Modulus,
                        _ => unreachable!(),
                    },
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        unary_minus_expression,
    ))(i)
}

/// *unary_expression* | *unary_expression* [ no line terminator here ] `**` *power_expression*
pub(crate) fn power_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((unary_expression, no_lt, tag("**"), ws, power_expression)),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: BinaryOpToken::Power,
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        unary_expression,
    ))(i)
}
