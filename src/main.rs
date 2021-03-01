mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_8;
mod day_9;

use day_1::{find_three_items_that_sum_2020, find_two_items_that_sum_2020};
use day_2::{number_of_valid_passwords, PolicyStrategy};
use day_3::{count_trees, tree_product};
use day_4::{count_valid_passports, CountType};
use day_9::{all_numbers_valid, encryption_weakness};

fn main() {
    println!("Day 1 part 1");
    let day_1_input = day_1::load_input_file("day_1.txt").expect("Missing input file");
    if let Some(numbers) = find_two_items_that_sum_2020(&day_1_input) {
        let answer = numbers.0 * numbers.1;
        println!("Answer is {}", answer)
    };
    println!("Day 1 part 2");
    if let Some(numbers) = find_three_items_that_sum_2020(&day_1_input) {
        let answer = numbers[0] * numbers[1] * numbers[2];
        println!("Answer is {}", answer)
    };

    println!("Day 2 part 1");
    let day_2_input = day_2::load_input_file("day_2.txt").expect("Missing input file");
    let answer = number_of_valid_passwords(&day_2_input, PolicyStrategy::MinMax);
    println!("Answer is {}", answer);
    println!("Day 2 part 2");
    let answer = number_of_valid_passwords(&day_2_input, PolicyStrategy::Position);
    println!("Answer is {}", answer);

    println!("Day 3 part 1");
    let day_3_input = day_3::load_input_file("day_3.txt").expect("Missing input file");
    let answer = count_trees(&day_3_input, (3, 1));
    println!("Answer is {}", answer);
    println!("Day 3 part 2");
    let slopes = Vec::from([(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);
    let answer = tree_product(&day_3_input, &slopes);
    println!("Answer is {}", answer);

    println!("Day 4 part 1");
    let day_4_input = day_4::load_input_file("day_4.txt").expect("Missing input file");
    let answer = count_valid_passports(&day_4_input, CountType::KeysOnly);
    println!("Answer is {}", answer);
    println!("Day 4 part 2");
    let answer = count_valid_passports(&day_4_input, CountType::KeysAndValues);
    println!("Answer is {}", answer);

    println!("Day 9 part 1");
    let day_9_input = day_9::load_input_file("day_9.txt").expect("Missing input file");
    let answer_1 = all_numbers_valid(&day_9_input, 25);
    println!("Answer is {}", answer_1.1.unwrap());
    println!("Day 9 part 2");
    let answer_2 = encryption_weakness(&day_9_input, answer_1.1.unwrap());
    println!("Answer is {}", answer_2.unwrap());
}
