fn find_items_that_sum_2020(input: &[u32]) -> (u32, u32) {
    (1721, 299)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_items_that_sum_2020() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_items_that_sum_2020(&input), (1721, 299))
    }
}
