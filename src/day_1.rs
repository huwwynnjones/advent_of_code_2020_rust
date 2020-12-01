use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
};

pub fn find_two_items_that_sum_2020(input: &[u32]) -> Option<(u32, u32)> {
    for i in input {
        for j in input {
            if i + j == 2020 {
                return Some((*i, *j));
            }
        }
    }
    None
}

pub fn find_three_items_that_sum_2020(input: &[u32]) -> Option<Vec<u32>> {
    for i in input {
        for j in input {
            for k in input {
                if i + j + k == 2020 {
                    let mut numbers = Vec::new();
                    numbers.push(*i);
                    numbers.push(*j);
                    numbers.push(*k);
                    return Some(numbers);
                }
            }
        }
    }
    None
}

pub fn load_input_file(file_name: &str) -> io::Result<Vec<u32>> {
    let input = File::open(file_name)?;
    let reader = BufReader::new(input);
    let mut numbers = Vec::new();

    for line in reader.lines() {
        let nmb = line
            .expect("Gap in input list")
            .parse::<u32>()
            .expect("Problems parsing");
        numbers.push(nmb)
    }
    Ok(numbers)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_items_that_sum_2020() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_two_items_that_sum_2020(&input), Some((1721, 299)))
    }

    #[test]
    fn test_find_items_that_sum_2020_alternate_order() {
        let input = [979, 675, 1721, 366, 1456, 299, 1456];
        assert_eq!(find_two_items_that_sum_2020(&input), Some((1721, 299)))
    }

    #[test]
    fn test_load_input_file() {
        let input = load_input_file("day_1_test.txt").expect("Unable to load the file");
        assert_eq!(input, [1472, 1757, 1404])
    }

    #[test]
    fn test_find_three_items_that_sum_2020() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(
            find_three_items_that_sum_2020(&input),
            Some(Vec::from([979, 366, 675]))
        )
    }
}
