use crate::lexers::identifier::identifier;
use crate::lexers::keyword::keyword;
use crate::lexers::string::double::double_quoted_string;
use crate::lexers::string::single::single_quoted_string;
use crate::lexers::token::operator_method_name;
use crate::types::ParseResult;
use crate::{Input, Interpolatable, Token, TokenResult};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, recognize};
use nom::sequence::tuple;

/// *symbol_literal* | *dynamic_symbol*
pub fn symbol(i: Input) -> TokenResult {
    alt((symbol_literal, dynamic_symbol))(i)
}

/// `:` *symbol_name*
pub(crate) fn symbol_literal(i: Input) -> TokenResult {
    map(recognize(tuple((char(':'), symbol_name))), |s| {
        Token::Symbol((*s).to_owned())
    })(i)
}

/// `:` *single_quoted_string*  | `:` *double_quoted_string* | `%s` *literal_beginning_delimiter* *non_expanded_literal_string** *literal_ending_delimiter*
pub(crate) fn dynamic_symbol(i: Input) -> TokenResult {
    alt((
        map(tuple((char(':'), single_quoted_string)), |mut t| {
            t.1.insert(0, ':');
            Token::Symbol(t.1)
        }),
        map(tuple((char(':'), double_quoted_string)), |t| match t.1 {
            Interpolatable::String(mut s) => {
                s.insert(0, ':');
                Token::Symbol(s)
            }
            Interpolatable::Interpolated(mut vec) => {
                vec.insert(0, Token::Segment(":".to_owned()));
                Token::InterpolatedSymbol(vec)
            }
        }),
    ))(i)
}

/// *identifier* | *operator_method_name* | *keyword*
pub(crate) fn symbol_name(i: Input) -> ParseResult {
    alt((recognize(identifier), operator_method_name, keyword))(i)
}
