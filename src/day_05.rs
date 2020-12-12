use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, value},
    multi::fold_many_m_n,
    sequence::pair,
    Finish, IResult,
};
use std::str::FromStr;

pub fn star_1(data: String) {
    let seats = seats(&data);
    let max_id = seats.map(|s| s.id()).max().unwrap_or(0);
    println!("{}", max_id);
}

pub fn star_2(data: String) {
    let seats = seats(&data);
    let mut ids = seats.map(|s| s.id()).collect::<Vec<_>>();
    ids.sort();
    let missing_id = ids
        .iter()
        .zip(ids.iter().skip(1))
        .find_map(|(a, b)| if a + 1 != *b { Some(a + 1) } else { None })
        .unwrap_or(0);
    println!("{}", missing_id);
}

fn seats<'a>(data: &'a str) -> impl Iterator<Item = Seat> + 'a {
    data.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<Seat>().ok())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Seat {
    row: u8,
    col: u8,
}

impl Seat {
    fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }

    fn id(&self) -> u32 {
        self.row as u32 * 8 + self.col as u32
    }
}

impl FromStr for Seat {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        all_consuming(seat)(input)
            .finish()
            .map_err(|_| ())
            .map(|(_, x)| x)
    }
}

fn seat(input: &str) -> IResult<&str, Seat> {
    map(pair(row, col), |(row, col)| Seat::new(row, col))(input)
}

fn row(input: &str) -> IResult<&str, u8> {
    fn row_bit(input: &str) -> IResult<&str, u8> {
        alt((value(0, tag("F")), value(1, tag("B"))))(input)
    }

    fold_many_m_n(7, 7, row_bit, 0u8, |acc, x| (acc << 1) + x)(input)
}

fn col(input: &str) -> IResult<&str, u8> {
    fn col_bit(input: &str) -> IResult<&str, u8> {
        alt((value(0, tag("L")), value(1, tag("R"))))(input)
    }

    fold_many_m_n(3, 3, col_bit, 0u8, |acc, x| (acc << 1) + x)(input)
}

#[cfg(test)]
mod tests {
    use super::{col, row, seat, Seat};

    #[test]
    fn from_str_parses() {
        assert_eq!("BFFFBBFRLL".parse::<Seat>(), Ok(Seat { row: 70, col: 4 }));
    }

    #[test]
    fn from_str_requires_all_input_to_be_consumed() {
        assert_eq!("BFFFBBFRLL...".parse::<Seat>(), Err(()));
    }

    #[test]
    fn seat_parses() {
        assert_eq!(seat("BFFFBBFRLL..."), Ok(("...", Seat { row: 70, col: 4 })));
    }

    #[test]
    fn row_parses() {
        assert_eq!(row("BFFFBBF..."), Ok(("...", 70)));
    }

    #[test]
    fn col_parses() {
        assert_eq!(col("RLL..."), Ok(("...", 4)));
    }
}
