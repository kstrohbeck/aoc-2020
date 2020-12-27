use crate::utils::u64_;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map, map_res, value},
    multi::many1,
    IResult,
};
use std::{collections::HashMap, fmt};

pub fn star_1(data: String) {
    let instructions = parse(&data);
    let mut state = State::default();
    for ins in instructions {
        state.step(ins);
    }
    println!("{}", state.total());
}

pub fn star_2(data: String) {
    let instructions = parse(&data);
    let mut state = State::default();
    for ins in instructions {
        state.step_v2(ins);
    }
    println!("{}", state.total());
}

fn parse(data: &str) -> Vec<Instruction> {
    data.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| instruction(s).unwrap().1)
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    mask: Mask,
    memory: HashMap<u64, u64>,
}

impl State {
    fn step(&mut self, ins: Instruction) {
        match ins {
            Instruction::SetMask(mask) => self.mask = mask,
            Instruction::SetMem(mem) => {
                let value = self.mask.apply_to(mem.value);
                *self.memory.entry(mem.address).or_insert(0) = value;
            }
        }
    }

    fn step_v2(&mut self, ins: Instruction) {
        match ins {
            Instruction::SetMask(mask) => self.mask = mask,
            Instruction::SetMem(mem) => {
                let addresses = self.mask.floating_masks().map(|m| m.apply_to(mem.address));
                for address in addresses {
                    *self.memory.entry(address).or_insert(0) = mem.value;
                }
            }
        }
    }

    fn total(&self) -> u64 {
        self.memory.values().sum()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            mask: Mask::default(),
            memory: HashMap::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    SetMask(Mask),
    SetMem(Mem),
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(set_mask, Instruction::SetMask),
        map(set_mem, Instruction::SetMem),
    ))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Mask {
    substrate: u64,
    mask: u64,
}

impl Mask {
    fn apply_to(self, value: u64) -> u64 {
        (self.substrate & self.mask) | (value & !self.mask)
    }

    fn floating_masks(self) -> impl Iterator<Item = Self> {
        let digits = (0..36)
            .filter(|x| (1 << x) & self.mask == 0)
            .collect::<Vec<_>>();

        let mask = self.substrate | !self.mask;

        (0..=digits.len())
            .map(move |c| digits.clone().into_iter().combinations(c))
            .flatten()
            .map(number_with_digits)
            .map(move |floating| Self {
                substrate: self.substrate | floating,
                mask,
            })
    }
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            substrate: 0,
            mask: 0,
        }
    }
}

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (0..36).rev() {
            if self.mask & (1 << i) == 0 {
                write!(f, "X")?;
            } else if self.substrate & (1 << i) == 0 {
                write!(f, "0")?;
            } else {
                write!(f, "1")?;
            }
        }
        Ok(())
    }
}

fn set_mask(input: &str) -> IResult<&str, Mask> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("mask")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = space0(input)?;
    mask(input)
}

fn mask(input: &str) -> IResult<&str, Mask> {
    let (input, bits) = many1(mask_bit)(input)?;

    let mut substrate = 0;
    let mut mask = 0;
    for bit in bits {
        substrate = substrate << 1;
        mask = mask << 1;
        if let Some(bit) = bit {
            substrate = (substrate & !1) | if bit { 1 } else { 0 };
            mask = mask | 1;
        }
    }
    Ok((input, Mask { substrate, mask }))
}

fn mask_bit(input: &str) -> IResult<&str, Option<bool>> {
    alt((
        value(None, tag("X")),
        value(Some(false), tag("0")),
        value(Some(true), tag("1")),
    ))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Mem {
    address: u64,
    value: u64,
}

fn set_mem(input: &str) -> IResult<&str, Mem> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("mem")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, _) = space0(input)?;
    let (input, address) = u64_(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("]")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = space0(input)?;
    let (input, value) = u64_(input)?;
    Ok((input, Mem { address, value }))
}

fn number_with_digits<I>(digits: I) -> u64
where
    I: IntoIterator<Item = u64>,
{
    digits.into_iter().fold(0, |acc, d| acc | (1 << d))
}
