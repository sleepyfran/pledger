use nom::bytes::complete::tag;
use nom::character::complete::{anychar, line_ending};
use nom::combinator::{map};
use nom::error::context;
use nom::multi::many_till;
use nom::sequence::{preceded};
use nom::IResult;

/// Attempts to parse a comment and returns nothing since we don't care about the comment content.
pub fn parse(input: &str) -> IResult<&str, ()> {
    context(
        "comment",
        map(preceded(tag("//"), many_till(anychar, line_ending)), |_| ()),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use nom::{
        error::Error,
        error::ErrorKind::{Eof, Tag},
        Err,
    };

    #[test]
    fn parses_valid_comment() {
        assert_eq!(parse("// A valid comment\n"), Ok(("", ())))
    }

    #[test]
    fn parses_valid_comment_that_contains_no_spaces() {
        assert_eq!(parse("//A valid comment\n"), Ok(("", ())));
    }

    #[test]
    fn errors_when_comment_does_not_end_in_line_ending() {
        assert_eq!(
            parse("// A comment that does not end in a line ending"),
            Err(Err::Error(Error {
                input: "",
                code: Eof,
            }))
        );
    }

    #[test]
    fn errors_when_comment_does_not_start_with_double_slash() {
        vec![" // Not a valid comment", "a// Not a valid comment", "/Not a valid comment"]
            .into_iter()
            .for_each(|input| {
                assert_eq!(
                    parse(input),
                    Err(Err::Error(Error {
                        input: input,
                        code: Tag
                    }))
                )
            })
    }
}
