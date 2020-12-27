pub fn star_1(data: String) {
    let (card_key, door_key) = parse(&data);
    let card_loop = brute_force_loop(card_key);
    let door_loop = brute_force_loop(door_key);
    let enc_key = calculate_enc_key(card_key, door_loop);
    println!("{}", enc_key);
}

pub fn star_2(data: String) {
    println!("There is no star 2.");
}

fn brute_force_loop(key: u64) -> u64 {
    let mut accum = 1;
    for num in 1.. {
        accum *= 7;
        accum %= 20201227;
        if accum == key {
            return num;
        }
    }
    unreachable!()
}

fn calculate_enc_key(card_key: u64, door_loop: u64) -> u64 {
    let mut accum = 1;
    for num in 0..door_loop {
        accum *= card_key;
        accum %= 20201227;
    }
    accum
}


fn parse(data: &str) -> (u64, u64) {
    let mut nums = data
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap());
    let card_pub = nums.next().unwrap();
    let door_pub = nums.next().unwrap();
    (card_pub, door_pub)
}
