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

fn move_tobogan(start_position: (usize, usize), slope: (usize, usize)) -> (usize, usize) {
    (start_position.0 + slope.0, start_position.1 + slope.1)
}

fn positions_used_to_reach_bottom(
    grid: &[Vec<Square>],
    slope: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut start_position = (0, 0);
    let grid_depth = grid.len() - 1;
    let mut positions = Vec::new();
    loop {
        let end_position = move_tobogan(start_position, slope);
        if end_position.1 == grid_depth {
            positions.push(end_position);
            return positions;
        } else {
            positions.push(end_position);
            start_position = end_position
        }
    }
}

fn count_trees(grid: &[Vec<Square>], slope: (usize, usize)) -> u32 {
    1
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

    #[test]
    fn test_move_tobogan() {
        let start_position = (0, 0);
        let slope = (3, 1);
        let final_position = (3, 1);
        assert_eq!(move_tobogan(start_position, slope), final_position)
    }

    #[test]
    fn test_positions_used_to_reach_bottom() {
        let input = Vec::from([
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
        ]);
        let grid = create_grid(&input);
        let slope = (3, 1);

        let correct_positions = Vec::from([(3, 1), (6, 2), (9, 3)]);
        assert_eq!(
            positions_used_to_reach_bottom(&grid, slope),
            correct_positions
        )
    }

    #[test]
    fn test_positions_used_to_reach_bottom_with_wrapping() {
        let input = Vec::from([
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            "..#.#...#.#".to_string(),
        ]);
        let grid = create_grid(&input);
        let slope = (3, 1);

        let correct_positions = Vec::from([(3, 1), (6, 2), (9, 3), (12, 4)]);
        assert_eq!(
            positions_used_to_reach_bottom(&grid, slope),
            correct_positions
        )
    }

    #[test]
    fn test_count_trees() {
        let input = Vec::from([
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
        ]);
        let grid = create_grid(&input);
        let slope = (3, 1);

        assert_eq!(count_trees(&grid, slope), 1)
    }
}
