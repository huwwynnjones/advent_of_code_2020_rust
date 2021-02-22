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
}
