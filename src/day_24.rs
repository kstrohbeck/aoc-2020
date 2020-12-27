use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::many1, IResult};
use std::collections::HashMap;

pub fn star_1(data: String) {
    let seqs = parse(&data);
    let mut flipped = HashMap::new();
    for coord in seqs.iter().map(|s| to_coord(s)) {
        *flipped.entry(coord).or_insert(0) += 1;
    }
    let count = flipped.iter().filter(|(_, v)| **v % 2 == 1).count();
    println!("{}", count);
}

pub fn star_2(data: String) {
    let seqs = parse(&data);
    let mut flipped = HashMap::new();
    for coord in seqs.iter().map(|s| to_coord(s)) {
        *flipped.entry(coord).or_insert(0) += 1;
    }
    let mut black = flipped
        .iter()
        .filter(|(_, v)| **v % 2 == 1)
        .map(|(k, _)| *k)
        .collect::<Vec<_>>();
    for _ in 0..100 {
        let mut counts = HashMap::<Coord, (bool, usize)>::new();
        let adjs = black.iter().flat_map(|a| {
            Direction::ALL
                .iter()
                .map(|d| d.to_coord_offset())
                .map(move |b| a + b)
        });
        for adj in adjs {
            let entry = counts.entry(adj).or_insert((false, 0));
            entry.1 += 1;
        }
        for b in black {
            let entry = counts.entry(b).or_insert((false, 0));
            entry.0 = true;
        }
        black = counts
            .iter()
            .filter(|(_, (is_black, count))| {
                (*is_black && *count > 0 && *count < 3) || (!*is_black && *count == 2)
            })
            .map(|(c, _)| *c)
            .collect();
    }
    println!("{}", black.len());
}

fn parse(data: &str) -> Vec<Vec<Direction>> {
    data.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| dir_sequence(s).unwrap().1)
        .collect()
}

fn to_coord(seq: &[Direction]) -> Coord {
    seq.iter()
        .map(|d| d.to_coord_offset())
        // TODO: Sum?
        .fold(Coord { x: 0, y: 0 }, |a, b| a + b)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add<&Coord> for Coord {
    type Output = Self;

    fn add(self, other: &Coord) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add for &Coord {
    type Output = Coord;

    fn add(self, other: Self) -> Self::Output {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add<Coord> for &Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Self::Output {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

impl Direction {
    const ALL: [Self; 6] = [
        Self::East,
        Self::Southeast,
        Self::Southwest,
        Self::West,
        Self::Northwest,
        Self::Northeast,
    ];

    fn to_coord_offset(self) -> Coord {
        let (x, y) = match self {
            Self::East => (1, 0),
            Self::Southeast => (0, -1),
            Self::Southwest => (-1, -1),
            Self::West => (-1, 0),
            Self::Northwest => (0, 1),
            Self::Northeast => (1, 1),
        };
        Coord { x, y }
    }
}

fn dir_sequence(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(direction)(input)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::East, tag("e")),
        value(Direction::Southeast, tag("se")),
        value(Direction::Southwest, tag("sw")),
        value(Direction::West, tag("w")),
        value(Direction::Northwest, tag("nw")),
        value(Direction::Northeast, tag("ne")),
    ))(input)
}
