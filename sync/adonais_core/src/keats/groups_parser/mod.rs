#[cfg(feature = "parser_nom")]
mod parser_nom;
#[cfg(feature = "parser_pest")]
mod parser_pest;

#[cfg(feature = "parser_nom")]
use parser_nom::tree;
#[cfg(feature = "parser_pest")]
use parser_pest::tree;

#[derive(Debug, PartialEq)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, PartialEq)]
pub enum Part {
    Single(u32),
    Range(Range),
}

/// Parse a collection of ints of the form `1-3, 5`.
/// If invalid syntax, default to 200 - 299.
pub fn parse_group_range(range: &str) -> Vec<u32> {
    if range.len() == 0 {
        return (200..300).collect();
    }

    let mut items: Vec<u32> = vec![];
    for part in tree(range) {
        match part {
            Part::Range(Range { start, end }) => items.extend(start..(end + 1)),
            Part::Single(group) => items.push(group),
        }
    }
    items.sort();
    items.dedup();
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_group_range() {
        let all_groups: Vec<u32> = (200..300).collect();

        // Single item
        assert_eq!(parse_group_range("0"), vec![0]);
        // Range of items, inclusive
        assert_eq!(parse_group_range("0-2"), vec![0, 1, 2]);
        // Multiple spec, separted by comma
        assert_eq!(parse_group_range("0, 7"), vec![0, 7]);
        assert_eq!(parse_group_range("0, 7-10"), vec![0, 7, 8, 9, 10]);

        // When in doubt, default to everyone
        assert_eq!(parse_group_range("0, 297-spam"), all_groups);
        assert_eq!(parse_group_range("0, spam-201"), all_groups);
        assert_eq!(parse_group_range(""), all_groups);
        assert_eq!(parse_group_range("250, spam"), all_groups);

        // Weird and wonderful edge case
        assert_eq!(
            parse_group_range("121,123 - 125   , 121"),
            vec![121, 123, 124, 125],
        );
        // Trailing whitespace is bad, make sure to trim it
        assert_eq!(parse_group_range("261 "), vec![261]);
    }
}
