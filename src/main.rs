#![allow(dead_code)]

mod solutions;
mod utils;

macro_rules! aoc {
    ($($day: ident),* $(,)?) => {{
        let args = std::env::args().collect::<Vec<String>>();
        if args.len() != 3 {
            eprintln!(
                "Missing day and part numbers. Exemple to run day 1, part 2: {} 1 2",
                args[0]
            );
            return;
        }
        let day = &args[1];
        let part = &args[2];

        $(
        if day.as_str() == &(stringify!($day))[3..4] {
            let input_path = format!("./input/day{}.txt", &stringify!($day)[3..4]);
            match part.as_str() {
                "1" => {
                    println!(
                        "Day {} - Part 1 : {}",
                        day,
                        crate::solutions::$day::part1(&crate::utils::load_input(input_path))
                    );
                }
                "2" => {
                    println!(
                        "Day {} - Part 2 : {}",
                        day,
                        crate::solutions::$day::part2(&crate::utils::load_input(input_path))
                    );
                }
                x => eprintln!("Only part 1 and 2 are available, not {x}.")
            }
            return;
        }
        )*

        eprintln!("Day {day} is not available.")
    }};
}

fn main() {
    aoc!(day1, day2, day3, day4);
}
