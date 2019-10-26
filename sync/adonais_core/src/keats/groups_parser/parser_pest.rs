use pest::Parser;

#[derive(Parser)]
#[grammar = "keats/groups_parser/groups.pest"]
struct GroupsParser;

use super::{Part, Range};

pub fn tree(input: &str) -> Vec<Part> {
    let pair = GroupsParser::parse(Rule::list, input)
        .unwrap()
        .next()
        .unwrap();

    // if we didn't match the entire pattern in one list
    if (pair.as_span().end() - pair.as_span().start()) != input.len() {
        return vec![Part::Range(Range {
            start: 200,
            end: 299,
        })];
    }

    let parts = pair
        .into_inner()
        .map(|pair| match pair.as_rule() {
            Rule::range => {
                let mut inner_rules = pair.into_inner();
                let start = inner_rules.next().unwrap().as_str().parse().unwrap();
                let end = inner_rules.next().unwrap().as_str().parse().unwrap();
                Part::Range(Range { start, end })
            }
            Rule::single => Part::Single(pair.as_str().parse().unwrap()),
            Rule::list | Rule::WHITESPACE => unreachable!(),
        })
        .collect();

    parts
}
