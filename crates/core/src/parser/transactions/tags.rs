use nom::{
    bytes::complete::{tag, take_while1},
    combinator::map,
    error::context,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use crate::parser::ast::Tag;

/// Parses a list of comma separated tags that begin with a ;. Tags can only include alphanumeric
/// characters and hyphens to separate words.
pub fn parse(input: &str) -> IResult<&str, Vec<Tag>> {
    context(
        "status",
        map(
            preceded(
                tag(";"),
                separated_list1(
                    tag(","),
                    take_while1(|c: char| c.is_alphanumeric() || c == '-'),
                ),
            ),
            |tags| tags.into_iter().map(String::from).collect(),
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use nom::{error::Error, error::ErrorKind::Tag, Err};

    use super::parse;

    #[test]
    fn parses_single_tag() {
        assert_eq!(parse(";single"), Ok(("", vec!["single".to_owned()])))
    }

    #[test]
    fn parses_multiple_tags() {
        assert_eq!(
            parse(";single,multiple"),
            Ok(("", vec!["single".to_owned(), "multiple".to_owned()]))
        )
    }

    #[test]
    fn parses_single_tag_with_hyphen() {
        assert_eq!(
            parse(";single-tag"),
            Ok(("", vec!["single-tag".to_owned()]))
        )
    }

    #[test]
    fn parses_multiple_tags_with_hyphen() {
        assert_eq!(
            parse(";single-tag,multiple-tag"),
            Ok(("", vec!["single-tag".to_owned(), "multiple-tag".to_owned()]))
        )
    }

    #[test]
    fn parses_single_tag_with_numbers_and_hyphen() {
        assert_eq!(
            parse(";single-tag-2021"),
            Ok(("", vec!["single-tag-2021".to_owned()]))
        )
    }

    #[test]
    fn parses_multiple_tags_with_numbers_and_hyphen() {
        assert_eq!(
            parse(";single-tag-2021,multiple-tag-2021"),
            Ok((
                "",
                vec!["single-tag-2021".to_owned(), "multiple-tag-2021".to_owned()]
            ))
        )
    }

    #[test]
    fn errors_when_input_does_not_begin_with_semicolon() {
        assert_eq!(
            parse("single"),
            Err(Err::Error(Error {
                input: "single",
                code: Tag
            }))
        );
    }

    #[test]
    fn ignores_rest_of_input_if_contains_spaces() {
        assert_eq!(
            parse(";single, with spaces"),
            Ok((", with spaces", vec!["single".to_owned()]))
        );
    }
}
