use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
};

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

fn get_square(grid: &[Vec<Square>], position: (usize, usize)) -> Option<&Square> {
    let (x, y) = position;
    let grid_line = match grid.get(y) {
        Some(grid_line) => grid_line,
        None => return None,
    };
    grid_line.get(x % grid_line.len())
}

pub fn count_trees(input: &[String], slope: (usize, usize)) -> u32 {
    let grid = create_grid(input);
    let positions = positions_used_to_reach_bottom(&grid, slope);
    positions
        .iter()
        .filter(|p| get_square(&grid, **p) == Some(&Square::Tree))
        .count() as u32
}

pub fn load_input_file(file_name: &str) -> io::Result<Vec<String>> {
    let input = File::open(file_name)?;
    let reader = BufReader::new(input);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.expect("Should be a line to read"))
    }
    Ok(lines)
}

pub fn tree_product(input: &[String], slopes: &[(usize, usize)]) -> u32 {
    slopes.iter().map(|s| count_trees(input, *s)).product()
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
        let slope = (3, 1);

        assert_eq!(count_trees(&input, slope), 1)
    }

    fn test_count_trees_example() {
        let input = Vec::from([
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ]);
        let slope = (3, 1);

        assert_eq!(count_trees(&input, slope), 7)
    }

    #[test]
    fn test_count_trees_with_different_slopes() {
        let input = Vec::from([
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ]);

        assert_eq!(count_trees(&input, (1, 1)), 2);
        assert_eq!(count_trees(&input, (5, 1)), 3);
        assert_eq!(count_trees(&input, (7, 1)), 4);
        assert_eq!(count_trees(&input, (1, 2)), 2);
    }

    #[test]
    fn test_get_square() {
        let input = Vec::from([
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
        ]);
        let grid = create_grid(&input);
        assert_eq!(get_square(&grid, (3, 1)), Some(&Square::Open));
        assert_eq!(get_square(&grid, (1, 2)), Some(&Square::Tree))
    }

    #[test]
    fn test_get_square_with_wrapping() {
        let input = Vec::from([
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
        ]);
        let grid = create_grid(&input);
        assert_eq!(get_square(&grid, (12, 4)), Some(&Square::Tree));
        assert_eq!(get_square(&grid, (15, 5)), Some(&Square::Tree))
    }

    #[test]
    fn test_load_input_file() {
        let input = load_input_file("day_3_test.txt").expect("Unable to load the file");
        let correct_list = Vec::from([
            ".##.............##......#.....#".to_string(),
            ".#.#................#..........".to_string(),
            "...#..#.##..#.#......#.#.#.#..#".to_string(),
        ]);
        assert_eq!(correct_list, input)
    }

    #[test]
    fn test_tree_product() {
        let input = Vec::from([
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ]);
        let slopes = Vec::from([(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);
        assert_eq!(tree_product(&input, &slopes), 336)
    }
}
