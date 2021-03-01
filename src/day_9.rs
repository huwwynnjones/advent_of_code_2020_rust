use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
};

// Checks to see if all the numbers in the given sequence
// are valid. When the first invalid number is found, a list is returned with result.
// The first item in the list tells you if the list is valid, the second gives
// the invalid number if found. The first invalid number found will stop the search
// and return the number.
fn valid_next_number(sequence: &[i64], number: i64) -> bool {
    for i in sequence {
        for j in sequence {
            if (i + j == number) && (i != j) {
                return true;
            }
        }
    }
    false
}

fn preamble(sequence: &[i64], position: usize, preamble_length: usize) -> Option<&[i64]> {
    if position >= sequence.len() || position < preamble_length {
        None
    } else {
        Some(&sequence[(position - preamble_length)..position])
    }
}

pub fn all_numbers_valid(sequence: &[i64], preamble_length: usize) -> (bool, Option<i64>) {
    for number in sequence {
        if let Some(pos) = position(&sequence, *number) {
            if let Some(p) = preamble(sequence, pos, preamble_length) {
                if !(valid_next_number(p, *number)) {
                    return (false, Some(*number));
                }
            }
        }
    }
    (true, None)
}

fn position(sequence: &[i64], number: i64) -> Option<usize> {
    for (idx, n) in sequence.iter().enumerate() {
        if *n == number {
            return Some(idx);
        }
    }
    None
}

fn find_sequence_that_sums_target(sequence: &[i64], target: i64) -> Option<Vec<i64>> {
    let mut subsequence = Vec::new();
    let mut current_total: i64;
    for number in sequence {
        subsequence.push(*number);
        current_total = subsequence.iter().sum();
        if current_total == target && subsequence.len() > 2 {
            return Some(subsequence);
        } else if current_total > target {
            return find_sequence_that_sums_target(sequence.split_first().unwrap().1, target);
        }
    }
    None
}

pub fn encryption_weakness(sequence: &[i64], target: i64) -> Option<i64> {
    match find_sequence_that_sums_target(sequence, target) {
        Some(s) => match s.iter().min() {
            Some(x) => match s.iter().max() {
                Some(y) => Some(x + y),
                None => None,
            },
            None => None,
        },
        None => None,
    }
}

pub fn load_input_file(file_name: &str) -> io::Result<Vec<i64>> {
    let input = File::open(file_name)?;
    let reader = BufReader::new(input);
    let mut numbers = Vec::new();

    for line in reader.lines() {
        let nmb = line
            .expect("Gap in input list")
            .parse::<i64>()
            .expect("Problems parsing");
        numbers.push(nmb)
    }
    Ok(numbers)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_next_number() {
        let sequence = (1..25).map(|i| i as i64).collect::<Vec<i64>>();
        assert_eq!(valid_next_number(&sequence, 25), true);
        assert_eq!(valid_next_number(&sequence, 50), false)
    }

    #[test]
    fn test_preamble() {
        let sequence = (1..=5).map(|i| i as i64).collect::<Vec<i64>>();
        assert_eq!(preamble(&sequence, 5, 4), None);
        assert_eq!(preamble(&sequence, 4, 5), None);
        assert_eq!(preamble(&sequence, 2, 3), None);
        assert_eq!(preamble(&sequence, 3, 3), Some(&sequence[0..3]));
    }

    #[test]
    fn test_all_numbers_valid() {
        let sequence = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(all_numbers_valid(&sequence, 5), (false, Some(127)))
    }

    #[test]
    fn test_find_sequence_that_sums_target() {
        let sequence = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let correct_sequence = vec![15, 25, 47, 40];
        assert_eq!(
            find_sequence_that_sums_target(&sequence, 127),
            Some(correct_sequence)
        )
    }

    #[test]
    fn test_encryption_weakness() {
        let sequence = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(encryption_weakness(&sequence, 127), Some(62))
    }
}
