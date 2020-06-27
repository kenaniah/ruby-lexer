use nom::combinator::map;
use nom::branch::alt;
use crate::{Input, Token, TokenResult};

mod single;
mod double;

/// *single_quoted_string* | *double_quoted_string* | *quoted_non_expanded_literal_string* | *quoted_expanded_literal_string* | *here_document* | *external_command_execution*
pub fn string_literal(i: Input) -> TokenResult {
    alt((
        map(single::single_quoted_string, |s| Token::SingleQuotedString(s)),
        map(double::double_quoted_string, |s| Token::DoubleQuotedString(s)),
        // quoted_non_expanded_literal_string,
        // quoted_expanded_literal_string,
        // here_document,
        // external_command_execution,
    ))(i)
}
