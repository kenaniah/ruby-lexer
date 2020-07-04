use crate::lexers::program::compound_statement;
use crate::lexers::token::literal;
use crate::{Input, Token, TokenResult};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::tuple;

mod variable;

/// *class_definition* | *singleton_class_definition* | *module_definition* | *method_definition* | *singleton_method_definition* | *yield_with_optional_argument* | *if_expression* | *unless_expression* | *case_expression* | *while_expression* | *until_expression* | *for_expression* | *return_without_argument* | *break_without_argument* | *next_without_argument* | *redo_expression* | *retry_expression* | *begin_expression* | *grouping_expression* | *variable_reference* | *scoped_constant_reference* | *array_constructor* | *hash_constructor* | *literal* | *defined_with_parenthesis* | *primary_method_invocation*
/// NOTE: This was referred to as *primary-expression* in the ISO spec
pub fn expression(i: Input) -> TokenResult {
    alt((
        //class_definition,
        //singleton_class_definition,
        //module_definition,
        //method_definition,
        //singleton_method_definition,
        //yield_with_optional_argument,
        //if_expression,
        //unless_expression,
        //case_expression,
        //while_expression,
        //until_expression,
        //for_expression,
        //return_without_argument,
        //break_without_argument,
        //next_without_argument,
        //redo_expression,
        //retry_expression,
        //begin_expression,
        grouping_expression,
        variable::variable_reference,
        //scoped_constant_reference,
        //array_constructor,
        //hash_constructor,
        literal,
        //defined_with_parenthesis,
        //primary_method_invocation,
    ))(i)
}

pub(crate) fn grouping_expression(i: Input) -> TokenResult {
    map(
        tuple((char('('), alt((compound_statement, expression)), char(')'))),
        |t| t.1,
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression() {
        use_parser!(expression, Input, Token);
        // Parse errors
        assert_err!("");
        assert_err!("nil ");
        assert_err!("bar\n");
        assert_err!("('");
        assert_err!("((foo)");
        // Success cases
        assert_ok!("nil", Token::Nil);
        assert_ok!("42", Token::Integer(42));
        assert_ok!("24.2", Token::Float(24.2));
        assert_ok!("meh", Token::LocalVariableIdentifier("meh".to_owned()));
        assert_ok!("-23e4", Token::Float(-230000.0));
        assert_ok!(
            "'hello world'",
            Token::SingleQuotedString("hello world".to_owned())
        );
        assert_ok!("()", Token::Expression(vec![]));
        assert_ok!(
            "((false))",
            Token::Expression(vec![Token::Expression(vec![Token::False])])
        );
        assert_ok!(
            "(2;; 5;)",
            Token::Expression(vec![Token::Integer(2), Token::Integer(5)])
        );
    }
}
