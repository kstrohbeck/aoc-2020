pub fn star_1(data: String) {
    let nums = parse(&data).collect::<Vec<_>>();
    let num = invalid_num(&nums, 26);
    println!("{}", num);
}

pub fn star_2(data: String) {
    let nums = parse(&data).collect::<Vec<_>>();
    let num = invalid_num(&nums, 26);
    let slice = nums
        .contiguous_slices()
        .filter(|s| s.len() < 2)
        .find(|s| s.iter().sum::<u64>() == num)
        .unwrap();
    let max = slice.iter().max().unwrap();
    let min = slice.iter().min().unwrap();
    let weakness = max + min;
    println!("{}", weakness);
}

fn invalid_num(nums: &[u64], window_size: usize) -> u64 {
    nums.windows(window_size)
        .filter_map(|n| {
            let prev = &n[..n.len() - 1];
            let num = &n[n.len() - 1];
            if prev.all_pairs().filter(|(a, b)| *a + *b == *num).count() >= 1 {
                None
            } else {
                Some(*num)
            }
        })
        .next()
        .unwrap()
}

fn parse<'a>(data: &'a str) -> impl Iterator<Item = u64> + 'a {
    data.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
}

struct AllPairs<'a, T> {
    first: usize,
    second: usize,
    slice: &'a [T],
}

impl<'a, T> AllPairs<'a, T> {
    fn new(slice: &'a [T]) -> Self {
        Self {
            first: 0,
            second: 1,
            slice,
        }
    }
}

impl<'a, T> Iterator for AllPairs<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.second >= self.slice.len() {
            self.first += 1;
            self.second = self.first + 1;
        }

        if self.second >= self.slice.len() {
            return None;
        }

        let pair = (&self.slice[self.first], &self.slice[self.second]);
        self.second += 1;
        Some(pair)
    }
}

struct ContiguousSlices<'a, T> {
    start: usize,
    end: usize,
    slice: &'a [T],
}

impl<'a, T> ContiguousSlices<'a, T> {
    fn new(slice: &'a [T]) -> Self {
        Self {
            start: 0,
            end: 1,
            slice,
        }
    }
}

impl<'a, T> Iterator for ContiguousSlices<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.end > self.slice.len() {
            self.start += 1;
            self.end = self.start + 1;
        }

        if self.end > self.slice.len() {
            return None;
        }

        let slice = &self.slice[self.start..self.end];
        self.end += 1;
        Some(slice)
    }
}

trait SliceEx {
    type Item;

    fn all_pairs<'a>(&'a self) -> AllPairs<'a, Self::Item>;

    fn contiguous_slices<'a>(&'a self) -> ContiguousSlices<'a, Self::Item>;
}

impl<T> SliceEx for [T] {
    type Item = T;

    fn all_pairs<'a>(&'a self) -> AllPairs<'a, Self::Item> {
        AllPairs::new(self)
    }

    fn contiguous_slices<'a>(&'a self) -> ContiguousSlices<'a, Self::Item> {
        ContiguousSlices::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::SliceEx;

    #[test]
    fn all_pairs_returns_all_pairs() {
        let list = vec![1, 2, 3, 4];
        let pairs = list.all_pairs().collect::<Vec<_>>();
        assert_eq!(
            pairs,
            vec![(&1, &2), (&1, &3), (&1, &4), (&2, &3), (&2, &4), (&3, &4)],
        );
    }

    #[test]
    fn contiguous_slices_returns_all_slices() {
        let list = vec![1, 2, 3, 4];
        let slices = list.contiguous_slices().collect::<Vec<_>>();
        assert_eq!(
            slices,
            vec![
                &[1][..],
                &[1, 2],
                &[1, 2, 3],
                &[1, 2, 3, 4],
                &[2],
                &[2, 3],
                &[2, 3, 4],
                &[3],
                &[3, 4],
                &[4],
            ]
        );
    }
}
