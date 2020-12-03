use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
};

#[derive(Eq, PartialEq, Debug)]
pub struct Policy {
    min: u32,
    max: u32,
    target_char: char,
}

impl Policy {
    pub fn new(min: u32, max: u32, target_char: char) -> Policy {
        Policy {
            min,
            max,
            target_char,
        }
    }
}

fn parse_password_policy(input: &str) -> Policy {
    let split_input = input.split_whitespace().collect::<Vec<&str>>();
    let number_input = split_input.get(0).expect("No number input found");
    let target_char = *split_input
        .get(1)
        .expect("No char input found")
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .first()
        .expect("Empty vec");
    let min_max_split = number_input.split_terminator('-').collect::<Vec<&str>>();
    let min = min_max_split
        .get(0)
        .expect("No min found")
        .parse::<u32>()
        .expect("Could not parse min");
    let max = min_max_split
        .get(1)
        .expect("No max found")
        .parse::<u32>()
        .expect("Could not parse max");
    Policy::new(min, max, target_char)
}

fn min_max_strategy(policy: &Policy, password: &str) -> bool {
    let count = password
        .chars()
        .filter(|c| *c == policy.target_char)
        .count() as u32;
    count >= policy.min && count <= policy.max
}

fn split_input_string(input: &str) -> (Policy, String) {
    let split_input = input.split_terminator(':').collect::<Vec<&str>>();
    let policy = split_input.get(0).expect("No policy string");
    let password = split_input.get(1).expect("No password string").trim();

    (parse_password_policy(&policy), password.to_string())
}

fn interpret_input_line(input: &str, strategy: fn(&Policy, &str) -> bool) -> bool {
    let (policy, password) = split_input_string(input);
    strategy(&policy, &password)
}

pub enum PolicyStrategy {
    MinMax,
}

pub fn number_of_valid_passwords(input: &[String], policy_strategy: PolicyStrategy) -> u32 {
    let strategy = match policy_strategy {
        PolicyStrategy::MinMax => min_max_strategy,
    };
    input
        .iter()
        .filter(|line| interpret_input_line(line, strategy))
        .count() as u32
}

pub fn load_input_file(file_name: &str) -> io::Result<Vec<String>> {
    let input = File::open(file_name)?;
    let reader = BufReader::new(input);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.expect("Should be a line to read"))
    }
    Ok(lines)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_password_policy() {
        let correct_policy = Policy::new(1, 3, 'a');
        let input = "1-3 a";
        assert_eq!(correct_policy, parse_password_policy(input));
    }

    #[test]
    fn test_parse_password_policy_two_digit_numbers() {
        let correct_policy = Policy::new(10, 12, 'a');
        let input = "10-12 a";
        assert_eq!(correct_policy, parse_password_policy(input));
    }

    #[test]
    fn test_min_max_strategy() {
        let policy = Policy::new(1, 3, 'a');
        let password = "abcde";
        assert_eq!(true, min_max_strategy(&policy, password));
        let policy = Policy::new(1, 3, 'b');
        let password = "cdefg";
        assert_eq!(false, min_max_strategy(&policy, password))
    }

    #[test]
    fn test_split_input_string() {
        let input = "1-3 a: abcde";
        let policy = Policy::new(1, 3, 'a');
        let password = "abcde";
        assert_eq!((policy, password.into()), split_input_string(input))
    }

    #[test]
    fn test_intepret_input_line() {
        let input = "1-3 a: abcde";
        assert_eq!(
            true,
            interpret_input_line(input, |policy, password| min_max_strategy(policy, password))
        );
    }

    #[test]
    fn test_number_of_valid_passwords() {
        let input = Vec::from([
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ]);
        assert_eq!(2, number_of_valid_passwords(&input, PolicyStrategy::MinMax))
    }

    #[test]
    fn test_load_input_file() {
        let input = load_input_file("day_2_test.txt").expect("Unable to load the file");
        let correct_list = Vec::from([
            "4-7 z: zzzfzlzzz".to_string(),
            "3-4 l: blllk".to_string(),
            "8-11 j: jjjjjjjgjjjj".to_string(),
        ]);
        assert_eq!(correct_list, input)
    }
}
