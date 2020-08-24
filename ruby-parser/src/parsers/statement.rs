use crate::lexer::TokenResult;
use crate::parsers::expression::primary_expression;
use crate::Input;
use nom::branch::alt;

/// *expression_statement* | *alias_statement* | *undef_statement* | *if_modifier_statement* | *unless_modifier_statement* | *while_modifier_statement* | *until_modifier_statement* | *rescue_modifier_statement* | *assignment_statement*
pub(crate) fn statement(i: Input) -> TokenResult {
    alt((
        expression_statement,
        alias_statement,
        undef_statement,
        if_modifier_statement,
        unless_modifier_statement,
        while_modifier_statement,
        until_modifier_statement,
        rescue_modifier_statement,
        assignment_statement,
    ))(i)
}

pub(crate) fn expression_statement(i: Input) -> TokenResult {
    primary_expression(i)
}

pub(crate) fn alias_statement(i: Input) -> TokenResult {
    stub(i)
}

pub(crate) fn undef_statement(i: Input) -> TokenResult {
    stub(i)
}

pub(crate) fn if_modifier_statement(i: Input) -> TokenResult {
    stub(i)
}

pub(crate) fn unless_modifier_statement(i: Input) -> TokenResult {
    stub(i)
}

pub(crate) fn while_modifier_statement(i: Input) -> TokenResult {
    stub(i)
}

pub(crate) fn until_modifier_statement(i: Input) -> TokenResult {
    stub(i)
}

pub(crate) fn rescue_modifier_statement(i: Input) -> TokenResult {
    stub(i)
}

pub(crate) fn assignment_statement(i: Input) -> TokenResult {
    stub(i)
}

fn stub(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}