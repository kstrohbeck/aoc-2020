use num::{
    integer::{self, ExtendedGcd},
    Integer,
};

pub fn star_1(data: String) {
    let (timestamp, ids) = parse(&data);
    for t in timestamp.. {
        for id in ids.iter().filter_map(|x| *x) {
            if t % id == 0 {
                println!("{}", (t - timestamp) * id);
                return;
            }
        }
    }
}

pub fn star_2(data: String) {
    let (_, ids) = parse(&data);

    let ids = ids
        .iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(|x| (i as i64, x as i64)))
        .map(|(a, b)| ((b - a) % b, b))
        .collect::<Vec<_>>();

    println!("{:?}", ids);

    // Verify all values are coprime.
    if ids
        .iter()
        .zip(ids.iter())
        .filter(|(x, y)| x.1 != y.1)
        .any(|(x, y)| integer::gcd(x.1, y.1) != 1)
    {
        panic!("Values aren't coprime.");
    }

    let rem = chinese_remainder(&ids[..]).unwrap();

    println!("{}", rem);
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(mods: &[(i64, i64)]) -> Option<i64> {
    let prod = mods.iter().map(|(_, m)| m).product::<i64>();
    let sum = mods
        .iter()
        .map(|(r, m)| {
            let p = prod / m;
            r * mod_inv(p, *m).unwrap() * p
        })
        .sum::<i64>();
    Some(sum % prod)
}

fn parse(data: &str) -> (u32, Vec<Option<u32>>) {
    let mut lines = data.lines().map(str::trim).filter(|s| !s.is_empty());
    let timestamp = lines.next().unwrap().parse::<u32>().unwrap();
    let ids = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| {
            if s == "x" {
                None
            } else {
                Some(s.parse::<u32>().unwrap())
            }
        })
        .collect();
    (timestamp, ids)
}
