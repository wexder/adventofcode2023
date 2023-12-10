mod advent23;

use crate::advent23::{day1, day2, day3, day4, day5};

fn main() {
    println!("Running day 1 part 1!");
    let res = day1::run("inputs/day1");
    println!("Result of day 1 part 1: {}", res);
    println!("");

    println!("Running day 1 part 2!");
    let res = day1::run_part2("inputs/day1");
    println!("Result of day 1 part 2: {}", res);
    println!("");

    println!("Running day 2 part 1!");
    let res = day2::run("inputs/day2");
    println!("Result of day 1 part 1: {}", res);
    println!("");

    println!("Running day 2 part 2!");
    let res = day2::run_part2("inputs/day2");
    println!("Result of day 2 part 2: {}", res);
    println!("");

    println!("Running day 3 part 1!");
    let res = day3::run("inputs/day3");
    println!("Result of day 3 part 1: {}", res);
    println!("");

    println!("Running day 3 part 2!");
    let res = day3::run_part2("inputs/day3");
    println!("Result of day 3 part 2: {}", res);
    println!("");

    println!("Running day 4 part 1!");
    let res = day4::run("inputs/day4");
    println!("Result of day 4 part 1: {}", res);
    println!("");

    println!("Running day 4 part 2!");
    let res = day4::run_part2("inputs/day4");
    println!("Result of day 4 part 2: {}", res);
    println!("");

    println!("Running day 5 part 1!");
    let res = day5::run("inputs/day5");
    println!("Result of day 5 part 1: {}", res);
    println!("");

    println!("Running day 5 part 2!");
    let res = day5::run_part2("inputs/day5");
    println!("Result of day 5 part 2: {}", res);
    println!("");
}
