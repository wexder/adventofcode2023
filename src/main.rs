mod advent23;

use crate::advent23::{day1, day2};

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
}
