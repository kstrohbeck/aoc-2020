use crate::utils::u64_;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, digit1, none_of, space0},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use std::collections::HashSet;

pub fn star_1(data: String) {
    let data = parse(&data);
    let invalid_field_sum = data
        .other_tickets
        .iter()
        .map(|t| t.invalid_fields(&data.rules).into_iter().sum::<u64>())
        .sum::<u64>();
    println!("{}", invalid_field_sum);
}

pub fn star_2(data: String) {
    let data = parse(&data);
    let valid_tickets = data
        .other_tickets
        .iter()
        .filter(|t| t.invalid_fields(&data.rules).len() == 0)
        .collect::<Vec<_>>();

    let mut rules_by_field = Vec::with_capacity(data.rules.len());

    for i in 0..data.rules.len() {
        let mut possible_rules = HashSet::new();
        let fields = valid_tickets.iter().map(|t| t.0[i]).collect::<Vec<_>>();

        for (j, rule) in data.rules.iter().enumerate() {
            if fields.iter().all(|f| rule.is_valid(*f)) {
                possible_rules.insert(j);
            }
        }

        rules_by_field.push(RulesForField::new(possible_rules));
    }

    loop {
        let poss = rules_by_field
            .iter()
            .filter_map(|rf| rf.only_possibility())
            .next();
        if let Some(poss) = poss {
            for rf in &mut rules_by_field {
                rf.simplify();
                rf.remove_rule(poss);
            }
        } else {
            break;
        }
    }

    let rules = rules_by_field
        .iter()
        .map(|rf| match rf {
            RulesForField::Certain(rule) => &data.rules[*rule],
            RulesForField::Unknown(_) => panic!("Not handling this right now"),
        })
        .collect::<Vec<_>>();

    let prod = rules
        .iter()
        .zip(data.my_ticket.0.iter())
        .filter(|(r, _)| r.name.starts_with("departure"))
        .map(|(_, f)| f)
        .product::<u64>();

    println!("{:?}", prod);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RulesForField {
    Certain(usize),
    Unknown(HashSet<usize>),
}

impl RulesForField {
    fn new(rules: HashSet<usize>) -> Self {
        Self::Unknown(rules)
    }

    fn as_unknown(&self) -> Option<&HashSet<usize>> {
        match self {
            Self::Certain(_) => None,
            Self::Unknown(rules) => Some(rules),
        }
    }

    fn as_unknown_mut(&mut self) -> Option<&mut HashSet<usize>> {
        match self {
            Self::Certain(_) => None,
            Self::Unknown(rules) => Some(rules),
        }
    }

    fn only_possibility(&self) -> Option<usize> {
        let rules = self.as_unknown()?;
        if rules.len() == 1 {
            rules.iter().next().copied()
        } else {
            None
        }
    }

    fn remove_rule(&mut self, rule: usize) {
        if let Self::Unknown(rules) = self {
            rules.remove(&rule);
        }
    }

    fn simplify(&mut self) {
        *self = match std::mem::replace(self, Self::Unknown(HashSet::new())) {
            Self::Certain(rule) => Self::Certain(rule),
            Self::Unknown(mut rules) => {
                if rules.len() == 1 {
                    Self::Certain(rules.drain().next().unwrap())
                } else {
                    Self::Unknown(rules)
                }
            }
        }
    }
}

fn parse(data: &str) -> Data {
    let mut sections = data.split("\n\n");
    let rule_section = sections.next().unwrap();
    let rules = rule_section
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| rule(s).unwrap().1)
        .collect();
    let my_ticket_section = sections.next().unwrap();
    let my_ticket = ticket(
        my_ticket_section
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .nth(1)
            .unwrap(),
    );
    let other_ticket_section = sections.next().unwrap();
    let other_tickets = other_ticket_section
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .skip(1)
        .map(|s| ticket(s))
        .collect();
    Data {
        rules,
        my_ticket,
        other_tickets,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Data {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    name: String,
    ranges: Vec<(u64, u64)>,
}

impl Rule {
    fn is_valid(&self, field: u64) -> bool {
        self.ranges
            .iter()
            .find(|(s, e)| *s <= field && field <= *e)
            .is_some()
    }
}

fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, name) = take_until(":")(input)?;
    let name = name.to_owned();
    let (input, _) = tag(":")(input)?;
    let (input, _) = space0(input)?;
    let (input, ranges) = separated_list1(range_sep, range)(input)?;
    Ok((input, Rule { name, ranges }))
}

fn range(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, start) = u64_(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, _) = space0(input)?;
    let (input, end) = u64_(input)?;
    Ok((input, (start, end)))
}

fn range_sep(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("or")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, ()))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Ticket(Vec<u64>);

impl Ticket {
    fn invalid_fields(&self, rules: &[Rule]) -> Vec<u64> {
        let mut invalid = Vec::new();

        'outer: for field in &self.0 {
            for rule in rules {
                if rule.is_valid(*field) {
                    continue 'outer;
                }
            }
            invalid.push(*field);
        }

        invalid
    }
}

fn ticket(input: &str) -> Ticket {
    Ticket(
        input
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::{rule, Rule};

    #[test]
    fn rule_with_single_range_parses() {
        assert_eq!(
            rule("foo: 1-12..."),
            Ok((
                "...",
                Rule {
                    name: String::from("foo"),
                    ranges: vec![(1, 12)]
                }
            ))
        );
    }

    #[test]
    fn rule_with_two_ranges_parses() {
        assert_eq!(
            rule("foo: 1-12 or 13-16..."),
            Ok((
                "...",
                Rule {
                    name: String::from("foo"),
                    ranges: vec![(1, 12), (13, 16)]
                }
            ))
        );
    }

    #[test]
    fn rule_with_three_ranges_parses() {
        assert_eq!(
            rule("foo: 1-12 or 13-16 or 20-100..."),
            Ok((
                "...",
                Rule {
                    name: String::from("foo"),
                    ranges: vec![(1, 12), (13, 16), (20, 100)]
                }
            ))
        );
    }
}
