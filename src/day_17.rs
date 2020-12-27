use itertools::iproduct;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub fn star_1(data: String) {
    let mut active = parse(&data).map(|(x, y)| (x, y, 0)).collect::<HashSet<_>>();
    for _ in 0..6 {
        active = step(&active, neighbors);
    }
    println!("{}", active.len());
}

pub fn star_2(data: String) {
    let mut active = parse(&data)
        .map(|(x, y)| (x, y, 0, 0))
        .collect::<HashSet<_>>();
    for _ in 0..6 {
        active = step(&active, neighbors4);
    }
    println!("{}", active.len());
}

pub fn step<T, F, I>(active: &HashSet<T>, neighbors: F) -> HashSet<T>
where
    T: Copy + Hash + Eq,
    F: Fn(T) -> I,
    I: Iterator<Item = T>,
{
    let mut counts = HashMap::new();
    for coord in active.iter().copied().flat_map(neighbors) {
        *counts.entry(coord).or_insert(0) += 1;
    }
    counts
        .iter()
        .filter(|(coord, count)| **count == 3 || (active.contains(coord) && **count == 2))
        .map(|(c, _)| *c)
        .collect()
}

fn parse<'a>(data: &'a str) -> impl Iterator<Item = (i64, i64)> + 'a {
    data.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
}

fn neighbors((x, y, z): (i64, i64, i64)) -> impl Iterator<Item = (i64, i64, i64)> {
    iproduct!(-1..=1, -1..=1, -1..=1)
        .filter(|o| *o != (0, 0, 0))
        .map(move |(a, b, c)| (x + a, y + b, z + c))
}

fn neighbors4((x, y, z, w): (i64, i64, i64, i64)) -> impl Iterator<Item = (i64, i64, i64, i64)> {
    iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
        .filter(|o| *o != (0, 0, 0, 0))
        .map(move |(a, b, c, d)| (x + a, y + b, z + c, w + d))
}
