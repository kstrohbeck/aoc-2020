use std::collections::{HashSet, VecDeque};

pub fn star_1(data: String) {
    let (mut first, mut second) = parse(&data);
    let score = combat(first, second);
    println!("{}", score);
}

pub fn star_2(data: String) {
    let (mut first, mut second) = parse(&data);
    let (_, score) = recursive_combat(first, second);
    println!("{}", score);
}

fn combat(mut first: VecDeque<u64>, mut second: VecDeque<u64>) -> u64 {
    while !first.is_empty() && !second.is_empty() {
        step(&mut first, &mut second);
    }
    if !first.is_empty() {
        score(&first)
    } else {
        score(&second)
    }
}

fn step(first: &mut VecDeque<u64>, second: &mut VecDeque<u64>) {
    let first_top = first.pop_front().unwrap();
    let second_top = second.pop_front().unwrap();
    if first_top > second_top {
        first.push_back(first_top);
        first.push_back(second_top);
    } else {
        second.push_back(second_top);
        second.push_back(first_top);
    }
}

fn recursive_combat(mut first: VecDeque<u64>, mut second: VecDeque<u64>) -> (Winner, u64) {
    let mut history = HashSet::<(VecDeque<u64>, VecDeque<u64>)>::new();

    while !first.is_empty() && !second.is_empty() {
        let h = (first.clone(), second.clone());
        if history.contains(&h) {
            return (Winner::First, score(&first));
        }
        history.insert(h);

        step_rec(&mut first, &mut second);
    }
    if !first.is_empty() {
        (Winner::First, score(&first))
    } else {
        (Winner::Second, score(&second))
    }
}

fn step_rec(first: &mut VecDeque<u64>, second: &mut VecDeque<u64>) {
    let first_top = first.pop_front().unwrap();
    let second_top = second.pop_front().unwrap();

    if (first_top as usize) <= first.len() && (second_top as usize) <= second.len() {
        let mut first_sub = first.iter().take(first_top as usize).copied().collect();
        let mut second_sub = second.iter().take(second_top as usize).copied().collect();

        let (winner, _) = recursive_combat(first_sub, second_sub);

        if winner == Winner::First {
            first.push_back(first_top);
            first.push_back(second_top);
        } else {
            second.push_back(second_top);
            second.push_back(first_top);
        }
    } else {
        if first_top > second_top {
            first.push_back(first_top);
            first.push_back(second_top);
        } else {
            second.push_back(second_top);
            second.push_back(first_top);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Winner {
    First,
    Second,
}

pub fn score(deck: &VecDeque<u64>) -> u64 {
    deck.iter()
        .zip((1..(deck.len() + 1)).rev())
        .map(|(c, i)| c * i as u64)
        .sum()
}

fn parse(data: &str) -> (VecDeque<u64>, VecDeque<u64>) {
    let mut sections = data.split("\n\n");
    let first = parse_section(&mut sections);
    let second = parse_section(&mut sections);
    (first, second)
}

fn parse_section<'a, I>(iter: &mut I) -> VecDeque<u64>
where
    I: Iterator<Item = &'a str>,
{
    iter.next()
        .unwrap()
        .lines()
        .skip(1)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}
