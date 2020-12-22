fn seat_rows() -> Vec<u32> {
    (0..128).collect()
}

fn lower_half(seats: &[u32]) -> Vec<u32> {
    seats[0..mid_point(&seats)].to_vec()
}

fn upper_half(seats: &[u32]) -> Vec<u32> {
    seats[mid_point(&seats)..seats.len()].to_vec()
}

fn mid_point(seats: &[u32]) -> usize {
    assert!(seats.len() % 2 == 0);
    seats.len() / 2
}

fn split_rows(seats: &[u32], indicator: char) -> Vec<u32> {
    split(seats, indicator, 'F', 'B')
}

fn split(seats: &[u32], indicator: char, lower_indicator: char, upper_indicator: char) -> Vec<u32> {
    if indicator == lower_indicator {
        lower_half(seats)
    } else if indicator == upper_indicator {
        upper_half(seats)
    } else {
        panic!("Indicator should match")
    }
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

    #[test]
    fn test_split_rows() {
        let correct_list: Vec<u32> = (0..64).collect();
        assert_eq!(split_rows(&seat_rows(), 'F'), correct_list)
    }
}
