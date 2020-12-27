use std::collections::HashMap;

pub fn star_1(data: String) {
    let mut cups = Cups::new(parse(&data));
    for _ in 0..100 {
        cups.step();
    }
    print_canonical(&cups);
}

pub fn star_2(data: String) {
    let mut cups = Cups::million(parse(&data));
    for _ in 0..10_000_000 {
        cups.step();
    }
    let prod = cups.canonical().take(2).product::<usize>();
    println!("{}", prod);
}

fn print_canonical(cups: &Cups) {
    for cup in cups.canonical() {
        print!("{}", cup);
    }
    println!("");
}

fn parse(data: &str) -> Vec<usize> {
    data.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cups {
    cur: usize,
    cup_next: Vec<usize>,
    lowest: usize,
    highest: usize,
}

impl Cups {
    fn new(cups: Vec<usize>) -> Self {
        let cups = cups.iter().map(|x| *x - 1).collect::<Vec<_>>();

        let cur = cups[0];
        let lowest = cups.iter().copied().min().unwrap();
        let highest = cups.iter().copied().max().unwrap();

        let mut cup_next = (lowest..=highest).collect::<Vec<usize>>();
        for (c, n) in cups.iter().zip(cups.iter().cycle().skip(1)) {
            cup_next[*c] = *n;
        }
        Self {
            cur,
            cup_next,
            lowest,
            highest,
        }
    }

    fn million(cups: Vec<usize>) -> Self {
        let cups = cups.iter().map(|x| *x - 1).collect::<Vec<_>>();

        let cur = cups[0];

        let mut cup_next = (1..1_000_000)
            .chain(std::iter::once(cur))
            .collect::<Vec<usize>>();

        for (c, n) in cups.iter().zip(cups.iter().skip(1)) {
            cup_next[*c] = *n;
        }
        cup_next[cups[cups.len() - 1]] = cups.len();

        Self {
            cur,
            cup_next,
            lowest: 0,
            highest: 999_999,
        }
    }

    fn step(&mut self) {
        let cur_next = self.cup_next[self.cur];
        let mut trg = self.sub(self.cur);
        let mut cups = CupIter {
            cups: self,
            cur: cur_next,
        };
        let a = cups.next().unwrap();
        let b = cups.next().unwrap();
        let last = cups.next().unwrap();
        while trg == a || trg == b || trg == last {
            trg = self.sub(trg);
        }

        let last_next = self.cup_next[last];
        let trg_next = self.cup_next[trg];

        self.cup_next[self.cur] = last_next;
        self.cup_next[last] = trg_next;
        self.cup_next[trg] = cur_next;

        self.cur = self.cup_next[self.cur];
    }

    fn sub(&self, val: usize) -> usize {
        if val == self.lowest {
            self.highest
        } else {
            val - 1
        }
    }

    fn canonical<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        CupIter {
            cups: self,
            cur: self.cur,
        }
        .map(|x| x + 1)
        .skip_while(|c| *c != 1)
        .skip(1)
        .take(self.cup_next.len() - 1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CupIter<'a> {
    cups: &'a Cups,
    cur: usize,
}

impl<'a> Iterator for CupIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur;
        self.cur = self.cups.cup_next[self.cur];
        Some(cur)
    }
}
