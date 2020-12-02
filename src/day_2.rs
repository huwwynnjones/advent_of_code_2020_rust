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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_password_policy() {
        let correct_policy = Policy::new(1, 3, 'a');
        let input = "1-3 a";
        assert_eq!(correct_policy, parse_password_policy(&input))
    }
}
