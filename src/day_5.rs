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

fn find_row(seats: &[u32], input: &[char]) -> Vec<u32> {
    if input.is_empty() {
        seats.to_vec()
    } else {
        find_row(
            &split_seats(&seats, *input.first().expect("No character")),
            &input[1..input.len()],
        )
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
    fn test_split_seats_front() {
        let correct_list: Vec<u32> = (0..64).collect();
        assert_eq!(split_seats(&seat_rows(), 'F'), correct_list)
    }

    #[test]
    fn test_split_seats_back() {
        let correct_list: Vec<u32> = (64..128).collect();
        assert_eq!(split_seats(&seat_rows(), 'B'), correct_list)
    }

    #[test]
    fn test_find_row() {
        let indicators: Vec<char> = "FBFBBFF".chars().collect();
        assert_eq!(find_row(&seat_rows(), &indicators), Vec::from([44]))
    }
}
