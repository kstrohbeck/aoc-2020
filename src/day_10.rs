pub fn star_1(data: String) {
    let nums = parse(&data);
    let (ones, threes) = nums
        .iter()
        .zip(nums.iter().skip(1))
        .map(|(a, b)| b - a)
        .fold((0, 0), |(ones, threes), x| match x {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        });
    let total = ones * threes;
    println!("{}", total);
}

pub fn star_2(data: String) {
    let nums = parse(&data);
    let mut choices_cache = vec![0u64; nums.len()];
    choices_cache[nums.len() - 1] = 1;
    for i in (0..(nums.len() - 1)).rev() {
        let num = nums[i];
        for j in (i + 1)..(i + 4) {
            if let Some(x) = nums.get(j) {
                if x - num <= 3 {
                    choices_cache[i] += choices_cache[j];
                }
            }
        }
    }
    let choices = choices_cache[0];
    println!("{:?}", choices);
}

fn parse(data: &str) -> Vec<u64> {
    let mut nums = data
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    nums.push(0);
    nums.sort();
    nums.push(nums[nums.len() - 1] + 3);
    nums
}
