use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space0, space1},
    combinator::{map_res, opt, value},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use std::collections::{HashMap, HashSet};

pub fn star_1(data: String) {
    let rules = parse(&data);
    let mut inv_graph = HashMap::new();
    for rule in rules {
        for contained in rule.contains {
            inv_graph
                .entry(contained.bag)
                .or_insert_with(HashSet::new)
                .insert(rule.container);
        }
    }
    let mut set = HashSet::new();
    for bag in &inv_graph[&Bag {
        adjective: "shiny",
        color: "gold",
    }] {
        set.insert(*bag);
    }

    loop {
        let mut new_set = set.clone();
        for bag in &set {
            if let Some(bags) = inv_graph.get(bag) {
                new_set = new_set.union(bags).copied().collect::<HashSet<Bag>>();
            }
        }
        if new_set == set {
            break;
        }
        set = new_set;
    }

    println!("{}", set.len());
}

pub fn star_2(data: String) {
    let rules = parse(&data);
    let graph = rules
        .map(|rule| (rule.container, rule.contains))
        .collect::<HashMap<_, _>>();
    let mut counts = HashMap::new();

    fn calculate_count<'a>(
        bag: Bag<'a>,
        graph: &HashMap<Bag<'a>, Vec<Contained<'a>>>,
        counts: &mut HashMap<Bag<'a>, u32>,
    ) -> u32 {
        if let Some(count) = counts.get(&bag) {
            return *count;
        }

        let contained = &graph[&bag];
        let count = contained
            .iter()
            .map(|c| c.amount * (1 + calculate_count(c.bag, graph, counts)))
            .sum();
        counts.insert(bag, count);
        count
    }

    let count = calculate_count(
        Bag {
            adjective: "shiny",
            color: "gold",
        },
        &graph,
        &mut counts,
    );

    println!("{}", count);
}

fn parse(data: &str) -> impl Iterator<Item = Rule> {
    data.lines()
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .map(|s| rule(s).unwrap().1)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule<'a> {
    container: Bag<'a>,
    contains: Vec<Contained<'a>>,
}

fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, _) = space0(input)?;
    let (input, container) = bag(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("contain")(input)?;
    let (input, _) = space1(input)?;
    let (input, contains) = contains(input)?;
    let (input, _) = tag(".")(input)?;
    Ok((
        input,
        Rule {
            container,
            contains,
        },
    ))
}

fn contains(input: &str) -> IResult<&str, Vec<Contained>> {
    alt((
        value(Vec::new(), preceded(space0, tag("no other bags"))),
        separated_list1(tag(","), contained),
    ))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Contained<'a> {
    bag: Bag<'a>,
    amount: u32,
}

fn contained(input: &str) -> IResult<&str, Contained> {
    let (input, _) = space0(input)?;
    let (input, amount) = u32_(input)?;
    let (input, _) = space1(input)?;
    let (input, bag) = bag(input)?;
    Ok((input, Contained { bag, amount }))
}

fn u32_(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Bag<'a> {
    adjective: &'a str,
    color: &'a str,
}

fn bag(input: &str) -> IResult<&str, Bag> {
    let (input, _) = space0(input)?;
    let (input, adjective) = alpha1(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = alpha1(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("bag")(input)?;
    let (input, _) = opt(tag("s"))(input)?;
    Ok((input, Bag { adjective, color }))
}

#[cfg(test)]
mod tests {
    use super::{bag, contained, rule, Bag, Contained, Rule};

    #[test]
    fn rule_parses_empty_contains() {
        assert_eq!(
            rule("faded blue bags contain no other bags."),
            Ok((
                "",
                Rule {
                    container: Bag {
                        adjective: "faded",
                        color: "blue"
                    },
                    contains: Vec::new()
                }
            ))
        )
    }

    #[test]
    fn rule_parses_single_contains() {
        assert_eq!(
            rule("bright white bags contain 1 shiny gold bag."),
            Ok((
                "",
                Rule {
                    container: Bag {
                        adjective: "bright",
                        color: "white",
                    },
                    contains: vec![Contained {
                        bag: Bag {
                            adjective: "shiny",
                            color: "gold",
                        },
                        amount: 1,
                    }],
                }
            ))
        );
    }

    #[test]
    fn rule_parses_multi_contains() {
        assert_eq!(
            rule("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
            Ok((
                "",
                Rule {
                    container: Bag {
                        adjective: "muted",
                        color: "yellow",
                    },
                    contains: vec![
                        Contained {
                            bag: Bag {
                                adjective: "shiny",
                                color: "gold",
                            },
                            amount: 2,
                        },
                        Contained {
                            bag: Bag {
                                adjective: "faded",
                                color: "blue",
                            },
                            amount: 9,
                        },
                    ]
                }
            ))
        )
    }

    #[test]
    fn contained_parses_multiple_digits() {
        assert_eq!(
            contained("12 shiny gold bags..."),
            Ok((
                ("..."),
                Contained {
                    bag: Bag {
                        adjective: "shiny",
                        color: "gold"
                    },
                    amount: 12
                }
            ))
        );
    }

    #[test]
    fn bag_parses_singular() {
        assert_eq!(
            bag("shiny gold bag..."),
            Ok((
                "...",
                Bag {
                    adjective: "shiny",
                    color: "gold"
                }
            )),
        );
    }

    #[test]
    fn bag_parses_plural() {
        assert_eq!(
            bag("shiny gold bags..."),
            Ok((
                "...",
                Bag {
                    adjective: "shiny",
                    color: "gold"
                }
            )),
        );
    }
}
