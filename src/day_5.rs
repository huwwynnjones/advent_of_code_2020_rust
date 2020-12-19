fn seat_rows() -> Vec<u32> {
    (0..128).collect()
}

fn lower_half(seats: &[u32]) -> Vec<u32> {
    (0..64).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lower_half() {
        let correct_list = (0..64).collect::<Vec<u32>>();
        assert_eq!(lower_half(&seat_rows()), correct_list)
    }
}
