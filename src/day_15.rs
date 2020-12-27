use std::collections::HashMap;

pub fn star_1(data: String) {
    for nums in parse(&data) {
        println!("{}", Nums::new(nums).nth(2020 - 1).unwrap());
    }
}

pub fn star_2(data: String) {
    for nums in parse(&data) {
        println!("{}", Nums::new(nums).nth(30000000 - 1).unwrap());
    }
}

fn parse(data: &str) -> Vec<Vec<u64>> {
    data.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.split(',').map(|s| s.parse::<u64>().unwrap()).collect())
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Nums {
    timestamp: u64,
    map: HashMap<u64, u64>,
    inner: Inner,
}

impl Nums {
    fn new(nums: Vec<u64>) -> Self {
        Self {
            timestamp: 0,
            map: HashMap::new(),
            inner: Inner::List(nums),
        }
    }

    fn say(&mut self, num: u64) -> u64 {
        let e = self.map.entry(num).or_insert(self.timestamp);
        let diff = self.timestamp - *e;
        *e = self.timestamp;
        self.timestamp += 1;
        diff
    }
}

impl Iterator for Nums {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let num = self.inner.value_at_timestamp(self.timestamp);
        let diff = self.say(num);

        let inner = std::mem::replace(&mut self.inner, Inner::Last(diff));

        if let Inner::List(ns) = inner {
            if (self.timestamp as usize) < ns.len() {
                self.inner = Inner::List(ns);
            }
        }

        Some(num)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Inner {
    List(Vec<u64>),
    Last(u64),
}

impl Inner {
    fn try_as_slice(&self) -> Option<&[u64]> {
        match self {
            Self::List(ns) => Some(ns),
            Self::Last(_) => None,
        }
    }

    fn value_at_timestamp(&self, timestamp: u64) -> u64 {
        match self {
            Self::List(ns) => ns[timestamp as usize],
            Self::Last(n) => *n,
        }
    }
}
