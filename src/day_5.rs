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

fn split_seats(seats: &[u32], indicator: char) -> Vec<u32> {
    match indicator {
        'F' => lower_half(seats),
        'B' => upper_half(seats),
        _ => panic!("Unknown indicator"),
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
    fn test_split_seats() {
        let correct_list: Vec<u32> = (0..64).collect();
        assert_eq!(split_seats(&seat_rows(), 'F'), correct_list)
    }
}
