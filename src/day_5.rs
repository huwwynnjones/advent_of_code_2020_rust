fn seat_rows() -> Vec<u32> {
    (0..128).collect()
}

fn lower_half(seats: &[u32]) -> Vec<u32> {
    assert!(seats.len() % 2 == 0);
    let mid_point = seats.len() / 2;
    seats[0..mid_point].to_vec()
}

fn upper_half(seats: &[u32]) -> Vec<u32> {
    assert!(seats.len() % 2 == 0);
    let mid_point = seats.len() / 2;
    seats[mid_point..seats.len()].to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lower_half() {
        let correct_list = (0..64).collect::<Vec<u32>>();
        assert_eq!(lower_half(&seat_rows()), correct_list)
    }

    #[test]
    fn test_upper_half() {
        let correct_list: Vec<u32> = (64..128).collect();
        assert_eq!(upper_half(&seat_rows()), correct_list)
    }
}
