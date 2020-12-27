use crate::utils::usize_;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char as char_, space0, space1},
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::collections::{HashMap, HashSet};

pub fn star_1(data: String) {
    let (rules, messages) = parse(&data);
    let count = messages
        .iter()
        .filter(|msg| rule_matches(&rules, 0, msg).contains(""))
        .count();
    println!("{}", count);
}

pub fn star_2(data: String) {
    let (mut rules, messages) = parse(&data);

    // Patch rules
    rules.insert(8, Rule::Nonterminal(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::Nonterminal(vec![vec![42, 31], vec![42, 11, 31]]));

    let count = messages
        .iter()
        .filter(|msg| rule_matches(&rules, 0, msg).contains(""))
        .count();
    println!("{}", count);
}

fn parse(data: &str) -> (HashMap<usize, Rule>, Vec<&str>) {
    let mut sections = data.split("\n\n");
    let rules_section = sections.next().unwrap();
    let rules = rules_section
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| rule(s).unwrap().1)
        .collect();
    let messages_section = sections.next().unwrap();
    let messages = messages_section
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    (rules, messages)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
    Nonterminal(Vec<Vec<usize>>),
    Terminal(char),
}

fn rule_matches<'a>(rules: &HashMap<usize, Rule>, idx: usize, input: &'a str) -> HashSet<&'a str> {
    match &rules[&idx] {
        Rule::Nonterminal(opts) => {
            let mut set = HashSet::new();
            for opt in opts {
                let results = opt.into_iter().fold(
                    std::iter::once(input).collect(),
                    |acc: HashSet<_>, &idx| {
                        acc.iter()
                            .flat_map(|input| rule_matches(rules, idx, input))
                            .collect()
                    },
                );
                set = &set | &results;
            }
            set
        }
        Rule::Terminal(c) => match str_split_first(input) {
            Some((first, rest)) if *c == first => std::iter::once(rest).collect(),
            _ => HashSet::new(),
        },
    }
}

fn str_split_first(input: &str) -> Option<(char, &str)> {
    let first = input.chars().next()?;
    let rest = unsafe { input.get_unchecked(first.len_utf8()..) };
    Some((first, rest))
}

fn using_rule<'a>(
    rule: &'a Rule,
    rules: &'a HashMap<usize, Rule>,
) -> impl for<'b> Fn(&'b str) -> IResult<&'b str, ()> + 'a {
    move |input: &str| match rule {
        Rule::Nonterminal(opts) => {
            let mut last_err = None;
            'o: for opt in opts {
                let mut input = input;
                for sub_idx in opt {
                    match using_rule(&rules[sub_idx], rules)(input) {
                        Ok((inp, _)) => input = inp,
                        Err(e) => last_err = Some(e),
                    };
                }
                return Ok((input, ()));
            }
            Err(last_err.unwrap())
        }
        Rule::Terminal(c) => value((), char_(*c))(input),
    }
}

fn rule(input: &str) -> IResult<&str, (usize, Rule)> {
    let (input, index) = usize_(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, rule) = alt((nonterminal, terminal))(input)?;
    Ok((input, (index, rule)))
}

fn nonterminal(input: &str) -> IResult<&str, Rule> {
    let (input, _) = space0(input)?;
    map(
        separated_list1(
            tuple((space0, tag("|"), space0)),
            separated_list1(space1, usize_),
        ),
        Rule::Nonterminal,
    )(input)
}

fn terminal(input: &str) -> IResult<&str, Rule> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("\"")(input)?;
    let (input, c) = anychar(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, Rule::Terminal(c)))
}

#[cfg(test)]
mod tests {
    use super::{rule, Rule};

    #[test]
    fn rule_parses_nonterminal() {
        assert_eq!(
            rule("1: 2 3 | 3 2 | 4..."),
            Ok((
                "...",
                (1, Rule::Nonterminal(vec![vec![2, 3], vec![3, 2], vec![4]]))
            ))
        );
    }
}
