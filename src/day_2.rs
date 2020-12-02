#[derive(Eq, PartialEq, Debug)]
struct Policy {
    min: u32,
    max: u32,
    target_char: char,
}

impl Policy {
    fn new(min: u32, max: u32, target_char: char) -> Policy {
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
    let target_char = split_input
        .get(1)
        .expect("No char input found")
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .first()
        .expect("Empty vec")
        .clone();
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

fn password_matches_policy(policy: &Policy, password: &str) -> bool {
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

fn interpret_input_line(input: &str) -> bool {
    let (policy, password) = split_input_string(input);
    password_matches_policy(&policy, &password)
}

fn number_of_valid_passwords(input: &[&str]) -> u32 {
    input
        .iter()
        .filter(|line| interpret_input_line(line))
        .count() as u32
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
    fn test_password_matches_policy() {
        let policy = Policy::new(1, 3, 'a');
        let password = "abcde";
        assert_eq!(true, password_matches_policy(&policy, password));
        let policy = Policy::new(1, 3, 'b');
        let password = "cdefg";
        assert_eq!(false, password_matches_policy(&policy, password))
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
        assert_eq!(true, interpret_input_line(input));
    }

    #[test]
    fn test_number_of_valid_passwords() {
        let input = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        assert_eq!(2, number_of_valid_passwords(&input))
    }
}
