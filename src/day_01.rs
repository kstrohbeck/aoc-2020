
pub fn star_1(data: String) {
    let values = parse(&data);
    let report = expense_report(&values[..]).unwrap();
    println!("{}", report);
}

pub fn star_2(data: String) {
    let values =  parse(&data);
    let report = triple_expense_report(&values[..]).unwrap();
    println!("{}", report);
}

fn parse(data: &str) -> Vec<u32> {
    data.lines()
        .map(str::trim)
        .filter_map(|line| line.parse::<u32>().ok())
        .collect()
}

fn expense_report(values: &[u32]) -> Option<u32> {
    AllPairs::new(values).find(|(a, b)| *a + *b == 2020).map(|(a, b)| a * b)
}

fn triple_expense_report(values: &[u32]) -> Option<u32> {
    AllTriples::new(values).find(|(a, b, c)| *a + *b + *c == 2020).map(|(a, b, c)| a * b * c)
}

struct AllPairs<'a, T> {
    values: &'a [T],
    cur_left: usize,
    cur_right: usize,
}

impl<'a, T> AllPairs<'a, T> {
    fn new(values: &'a [T]) -> Self {
        Self {
            values,
            cur_left: 0,
            cur_right: 1,
        }
    }
}

impl<'a, T> Iterator for AllPairs<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_right >= self.values.len() {
            self.cur_left += 1;
            self.cur_right = self.cur_left + 1;
        }

        if self.cur_left >= self.values.len() || self.cur_right >= self.values.len() {
            return None;
        }

        let pair = (&self.values[self.cur_left], &self.values[self.cur_right]);

        self.cur_right += 1;

        Some(pair)
    }
}

struct AllTriples<'a, T> {
    values: &'a [T],
    cur_left: usize,
    cur_middle: usize,
    cur_right: usize,
}

impl<'a, T> AllTriples<'a, T> {
    fn new(values: &'a [T]) -> Self {
        Self {
            values,
            cur_left: 0,
            cur_middle: 1,
            cur_right: 2,
        }
    }
}

impl<'a, T> Iterator for AllTriples<'a, T> {
    type Item = (&'a T, &'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.values.len();

        if self.cur_right >= len {
            self.cur_middle += 1;
            self.cur_right = self.cur_middle + 1;
        }

        if self.cur_middle >= len || self.cur_right >= len {
            self.cur_left += 1;
            self.cur_middle = self.cur_left + 1;
            self.cur_right = self.cur_middle + 1;
        }

        if self.cur_left >= len || self.cur_middle >= len || self.cur_right >= len {
            return None;
        }

        let triple = (&self.values[self.cur_left], &self.values[self.cur_middle], &self.values[self.cur_right]);
        self.cur_right += 1;
        Some(triple)
    }
}

#[cfg(test)]
mod tests {
    use super::{AllPairs, AllTriples};

    #[test]
    fn empty_slice_produces_no_pairs() {
        let values = Vec::<u32>::new();
        let pairs = AllPairs::new(&values[..]).collect::<Vec<_>>();
        assert_eq!(pairs, vec![]);
    }

    #[test]
    fn singleton_slice_produces_no_pairs() {
        let values = vec![1];
        let pairs = AllPairs::new(&values[..]).collect::<Vec<_>>();
        assert_eq!(pairs, vec![]);
    }

    #[test]
    fn paired_slice_produces_single_pair() {
        let values = vec![1, 2];
        let pairs = AllPairs::new(&values[..]).collect::<Vec<_>>();
        assert_eq!(pairs, vec![(&1, &2)]);
    }

    #[test]
    fn three_value_slice_produces_all_three_pairs() {
        let values = vec![1, 2, 3];
        let pairs = AllPairs::new(&values[..]).collect::<Vec<_>>();
        assert_eq!(pairs, vec![(&1, &2), (&1, &3), (&2, &3)]);
    }

    #[test]
    fn four_value_slice_produces_all_six_pairs() {
        let values = vec![1, 2, 3, 4];
        let pairs = AllPairs::new(&values[..]).collect::<Vec<_>>();
        assert_eq!(pairs, vec![(&1, &2), (&1, &3), (&1, &4), (&2, &3), (&2, &4), (&3, &4)]);
    }

    #[test]
    fn empty_slice_produces_no_triples() {
        let values = Vec::<u32>::new();
        let triples = AllTriples::new(&values[..]).collect::<Vec<_>>();
        assert_eq!(triples, vec![]);
    }

    #[test]
    fn singleton_slice_produces_no_triples() {
        let values = vec![1];
        let triples = AllTriples::new(&values[..]).collect::<Vec<_>>();
        assert_eq!(triples, vec![]);
    }

    #[test]
    fn paired_slice_produces_no_triples() {
        let values = vec![1, 2];
        let triples = AllTriples::new(&values[..]).collect::<Vec<_>>();
        assert_eq!(triples, vec![]);
    }

    #[test]
    fn tripled_slice_produces_single_triple() {
        let values = vec![1, 2, 3];
        let triples = AllTriples::new(&values[..]).collect::<Vec<_>>();
        assert_eq!(triples, vec![(&1, &2, &3)]);
    }

    #[test]
    fn four_value_slice_produces_all_four_triples() {
        let values = vec![1, 2, 3, 4];
        let triples = AllTriples::new(&values[..]).collect::<Vec<_>>();
        assert_eq!(triples, vec![(&1, &2, &3), (&1, &2, &4), (&1, &3, &4), (&2, &3, &4)]);
    }
}
