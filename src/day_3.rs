#[derive(Debug, Eq, PartialEq)]
enum Square {
    Open,
    Tree,
}

fn create_grid_line(input: &str) -> Vec<Square> {
    input
        .chars()
        .map(|c| if c == '.' { Square::Open } else { Square::Tree })
        .collect()
}

fn create_grid(input: &[String]) -> Vec<Vec<Square>> {
    input.iter().map(|line| create_grid_line(line)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_grid_line() {
        let input = "..##.......";
        let correct_grid_line = Vec::from([
            Square::Open,
            Square::Open,
            Square::Tree,
            Square::Tree,
            Square::Open,
            Square::Open,
            Square::Open,
            Square::Open,
            Square::Open,
            Square::Open,
            Square::Open,
        ]);
        assert_eq!(create_grid_line(input), correct_grid_line);
    }

    #[test]
    fn test_create_grid() {
        let input = Vec::from(["..##.......".to_string(), "#...#...#..".to_string()]);
        let correct_grid = Vec::from([
            [
                Square::Open,
                Square::Open,
                Square::Tree,
                Square::Tree,
                Square::Open,
                Square::Open,
                Square::Open,
                Square::Open,
                Square::Open,
                Square::Open,
                Square::Open,
            ],
            [
                Square::Tree,
                Square::Open,
                Square::Open,
                Square::Open,
                Square::Tree,
                Square::Open,
                Square::Open,
                Square::Open,
                Square::Tree,
                Square::Open,
                Square::Open,
            ],
        ]);
        assert_eq!(create_grid(&input), correct_grid)
    }
}
