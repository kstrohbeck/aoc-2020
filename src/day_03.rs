pub fn star_1(data: String) {
    let count = traverse_slope(&data, 3, 1);
    println!("{}", count);
}

pub fn star_2(data: String) {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let count = slopes
        .iter()
        .map(|(r, d)| traverse_slope(&data, *r, *d))
        .product::<usize>();
    
    println!("{}", count);
}

fn traverse_slope(data: &str, right: usize, down: usize) -> usize {
    data.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .step_by(down)
        .enumerate()
        .map(|(i, s)| ((right * i) % s.len(), s))
        .filter_map(|(i, s)| s.chars().nth(i))
        .filter(|c| *c == '#')
        .count()
}
