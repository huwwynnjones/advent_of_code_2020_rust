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
    let min = input
        .get(0..1)
        .expect("No characters")
        .parse::<u32>()
        .expect("Could not parse");
    let max = input
        .get(2..3)
        .expect("No characters")
        .parse::<u32>()
        .expect("Could not parse");
    let target_char = input
        .get(4..5)
        .expect("No characters")
        .chars()
        .collect::<Vec<char>>()
        .first()
        .expect("Empty vec")
        .clone();
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
    (Policy::new(1, 3, 'a'), "abcde".into())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_password_policy() {
        let correct_policy = Policy::new(1, 3, 'a');
        let input = "1-3 a";
        assert_eq!(correct_policy, parse_password_policy(&input))
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
        assert_eq!((policy, password.into()), split_input_string(&input))
    }
}
