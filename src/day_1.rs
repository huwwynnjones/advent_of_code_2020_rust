fn find_items_that_sum_2020(input: &[u32]) -> Option<(u32, u32)> {
    for i in input {
        for j in input {
            if i + j == 2020 {
                return Some((*i, *j));
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_items_that_sum_2020() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_items_that_sum_2020(&input), Some((1721, 299)))
    }

    #[test]
    fn test_find_items_that_sum_2020_alternate_order() {
        let input = [979, 675, 1721, 366, 1456, 299, 1456];
        assert_eq!(find_items_that_sum_2020(&input), Some((1721, 299)))
    }
}
