pub fn star_1(data: String) {
    let groups = parse_groups(&data);
    let total: u32 = groups
        .into_iter()
        .map(|v| v.into_iter().fold(0, |acc, x| acc | x))
        .map(|c| c.count_ones())
        .sum();
    println!("{:?}", total);
}

pub fn star_2(data: String) {
    let groups = parse_groups(&data);
    let total: u32 = groups
        .into_iter()
        .map(|v| v.into_iter().fold(0xffffffff, |acc, x| acc & x))
        .map(|c| c.count_ones())
        .sum();
    println!("{:?}", total);
}

fn parse_groups(data: &str) -> Vec<Vec<u32>> {
    Groups::new(data.lines().map(str::trim))
        .map(|v| v.into_iter().map(parse_answers).collect())
        .collect()
}

fn parse_answers(s: &str) -> u32 {
    s.chars()
        .map(|c| c as u32 - 'a' as u32)
        .fold(0, |acc, x| acc | (1 << x))
}

struct Groups<I> {
    iter: I,
}

impl<I> Groups<I> {
    fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<'a, I> Iterator for Groups<I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut vec = Vec::new();
        while let Some(s) = self.iter.next() {
            if s == "" {
                break;
            }
            vec.push(s);
        }
        if !vec.is_empty() {
            Some(vec)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_answers;

    #[test]
    fn answers_parse() {
        assert_eq!(parse_answers("heqznia"), 0b10000000010010000110010001);
    }
}
