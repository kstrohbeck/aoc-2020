use std::{env, fs::File, io::Read};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let day = args.next().unwrap().parse::<u32>().unwrap();
    let star = args.next().unwrap().parse::<u32>().unwrap();
    let filename = args.next().unwrap();

    let mut data = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut data).unwrap();

    let func = match (day, star) {
        (1, 1) => day_01::star_1,
        (1, 2) => day_01::star_2,
        (2, 1) => day_02::star_1,
        (2, 2) => day_02::star_2,
        (3, 1) => day_03::star_1,
        (3, 2) => day_03::star_2,
        (4, 1) => day_04::star_1,
        (4, 2) => day_04::star_2,
        (5, 1) => day_05::star_1,
        (5, 2) => day_05::star_2,
        (6, 1) => day_06::star_1,
        (6, 2) => day_06::star_2,
        (7, 1) => day_07::star_1,
        (7, 2) => day_07::star_2,
        (8, 1) => day_08::star_1,
        (8, 2) => day_08::star_2,
        _ => {
            println!("Invalid day and/or star.");
            return;
        }
    };

    func(data);
}
