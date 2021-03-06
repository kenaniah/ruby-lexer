use crate::lexer::*;

/// `!` | `!=` | `!~` | `&&` | `||` | *operator_method_name* | `=` | *assignment_operator*
pub(crate) fn operator(i: Input) -> LexResult {
    recognize(alt((
        assignment_operator,
        operator_method_name,
        tag("="),
        tag("||"),
        tag("&&"),
        tag("!~"),
        tag("!="),
        tag("!"),
    )))(i)
}

/// `^` | `&` | `|` | `<=>` | `==` | `===` | `=~` | `>` | `>=` | `<` | `<=` | `<<` | `>>` | `+` | `-` | `*` | `/` | `%` | `**` | `~` | `+@` | `-@` | `[]` | `[]=`
pub(crate) fn operator_method_name(i: Input) -> LexResult {
    recognize(alt((
        alt((
            tag("<=>"),
            tag("==="),
            tag("[]="),
            tag("=="),
            tag("=~"),
            tag(">="),
            tag(">>"),
            tag("<="),
            tag("<<"),
            tag("**"),
            tag(">"),
            tag("<"),
        )),
        alt((
            tag("^"),
            tag("&"),
            tag("|"),
            tag("+"),
            tag("-"),
            tag("*"),
            tag("/"),
            tag("%"),
            tag("~"),
            tag("+@"),
            tag("-@"),
            tag("[]"),
        )),
    )))(i)
}

/// *assignment_operator_name* `=`
pub(crate) fn assignment_operator(i: Input) -> LexResult {
    recognize(tuple((assignment_operator_name, char('='))))(i)
}

/// `&&` | `||` | `^` | `&` | `|` | `<<` | `>>` | `+` | `-` | `%` | `/` | `**`
pub(crate) fn assignment_operator_name(i: Input) -> LexResult {
    alt((
        tag("&&"),
        tag("||"),
        tag("^"),
        tag("&"),
        tag("|"),
        tag("<<"),
        tag(">>"),
        tag("+"),
        tag("-"),
        tag("%"),
        tag("/"),
        tag("**"),
    ))(i)
}
