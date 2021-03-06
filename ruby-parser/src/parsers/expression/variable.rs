use crate::ast::Literal;
use crate::lexer::*;
use crate::parsers::expression::recursing_primary_expression;
use crate::parsers::token::identifier::*;

/// *pseudo_variable* | *variable*
pub(crate) fn variable_reference(i: Input) -> NodeResult {
    alt((pseudo_variable, map(variable, |v| Node::from(v))))(i)
}

/// *constant_identifier* | *global_variable_identifier* | *class_variable_identifier* | *instance_variable_identifier* | *local_variable_identifier*
pub(crate) fn variable(i: Input) -> IdentifierResult {
    alt((
        constant_identifier,
        global_variable_identifier,
        class_variable_identifier,
        instance_variable_identifier,
        local_variable_identifier,
    ))(i)
}

/// *nil_expression* | *true_expression* | *false_expression* | *self_expression*
pub(crate) fn pseudo_variable(i: Input) -> NodeResult {
    alt((
        nil_expression,
        true_expression,
        false_expression,
        self_expression,
    ))(i)
}

/// `::` *constant_identifier*
pub(crate) fn simple_scoped_constant_reference(i: Input) -> NodeResult {
    map(tuple((tag("::"), ws0, constant_identifier)), |_| {
        Node::Placeholder
    })(i)
}

/// `::` *constant_identifier*
pub(crate) fn _scoped_constant_reference(i: Input) -> NodeResult {
    map(
        tuple((
            tag("::"),
            ws0,
            constant_identifier,
            opt(recursing_primary_expression),
        )),
        |_| Node::Placeholder,
    )(i)
}

/// `nil`
pub(crate) fn nil_expression(i: Input) -> NodeResult {
    map(tuple((tag("nil"), not(peek(identifier_character)))), |_| {
        Node::Nil
    })(i)
}

/// `true`
pub(crate) fn true_expression(i: Input) -> NodeResult {
    map(
        tuple((tag("true"), not(peek(identifier_character)))),
        |_| Node::Literal(Literal::Boolean(true)),
    )(i)
}

/// `false`
pub(crate) fn false_expression(i: Input) -> NodeResult {
    map(
        tuple((tag("false"), not(peek(identifier_character)))),
        |_| Node::Literal(Literal::Boolean(false)),
    )(i)
}

/// `self`
pub(crate) fn self_expression(i: Input) -> NodeResult {
    map(
        tuple((tag("self"), not(peek(identifier_character)))),
        |_| Node::Self_,
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_reference() {
        use_parser!(variable_reference);
        // Parse errors
        assert_err!("");
        assert_err!("nil ");
        assert_err!("bar\n");
        // Success cases
        assert_ok!("nil", Node::Nil);
        assert_ok!("true", Node::boolean(true));
        assert_ok!("false", Node::boolean(false));
        assert_ok!("self", Node::Self_);
        assert_ok!("TRUE", Node::ident("TRUE", IdentifierKind::Constant));
        assert_ok!("False", Node::ident("False", IdentifierKind::Constant));
        assert_ok!("nil_", Node::ident("nil_", IdentifierKind::LocalVariable));
        assert_ok!(
            "$true",
            Node::ident("$true", IdentifierKind::GlobalVariable)
        );
    }
}
