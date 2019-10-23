use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::multispace0,
    combinator::{map, map_res},
    multi::separated_nonempty_list,
    sequence::{delimited, tuple},
    IResult,
};

use super::{Part, Range};

fn to_u32(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse()
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn dash_delimiter(input: &str) -> IResult<&str, &str> {
    delimited(multispace0, tag("-"), multispace0)(input)
}

fn comma_or_space_delimiter(input: &str) -> IResult<&str, &str> {
    alt((delimited(multispace0, tag(","), multispace0), tag(" ")))(input)
}

fn single(input: &str) -> IResult<&str, u32> {
    map_res(take_while(is_digit), to_u32)(input)
}

fn range(input: &str) -> IResult<&str, Range> {
    let (input, (start, _, end)) = tuple((single, dash_delimiter, single))(input)?;
    Ok((input, Range { start, end }))
}

fn part(input: &str) -> IResult<&str, Part> {
    alt((
        map(range, |r| Part::Range(r)),
        map(single, |s| Part::Single(s)),
    ))(input)
}

fn list(input: &str) -> IResult<&str, Vec<Part>> {
    separated_nonempty_list(comma_or_space_delimiter, part)(input)
}

pub fn tree(input: &str) -> Vec<Part> {
    let (leftover, mut parts) = list(input.trim()).unwrap_or((
        "",
        vec![Part::Range(Range {
            start: 200,
            end: 299,
        })],
    ));
    if leftover.len() > 0 {
        parts = vec![Part::Range(Range {
            start: 200,
            end: 299,
        })];
    };
    parts
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    fn assert_eq_and_complete<T: Debug + PartialEq, E: Debug + PartialEq>(
        actual: Result<(&str, T), E>,
        expected: T,
    ) {
        assert_eq!(actual, Ok(("", expected)))
    }

    #[test]
    fn parse_single() {
        assert_eq_and_complete(single("123"), 123);
    }

    #[test]
    fn parse_range() {
        assert_eq_and_complete(
            range("123 - 125"),
            Range {
                start: 123,
                end: 125,
            },
        );
    }

    #[test]
    fn parse_part() {
        assert_eq_and_complete(
            part("123 - 125"),
            Part::Range(Range {
                start: 123,
                end: 125,
            }),
        );
        assert_eq_and_complete(part("121"), Part::Single(121));
    }

    #[test]
    fn parse_groups() {
        // delimiter includes comma
        assert_eq_and_complete(
            list("121,123 - 125   , 121"),
            vec![
                Part::Single(121),
                Part::Range(Range {
                    start: 123,
                    end: 125,
                }),
                Part::Single(121),
            ],
        );

        // delimiter does not include comma
        assert_eq_and_complete(
            list("121 123 - 125 121"),
            vec![
                Part::Single(121),
                Part::Range(Range {
                    start: 123,
                    end: 125,
                }),
                Part::Single(121),
            ],
        );
    }
}
