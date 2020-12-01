mod day_1;

use day_1::{find_items_that_sum_2020, load_input_file};

fn main() {
    println!("Day 1 part 1");
    let day_1_input = load_input_file("day_1.txt").expect("Missing input file");
    if let Some(numbers) = find_items_that_sum_2020(&day_1_input) {
        let answer = numbers.0 * numbers.1;
        println!("Answer is {}", answer)
    };
}