use std::collections::HashSet;

use grid::{char_grid_from_line, Grid};
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_4/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_4/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let grid = char_grid_from_line(input);
    println!("part 1: {}", part_1_inner(grid))
}

fn part_2(input: AocBufReader) {
    let grid = char_grid_from_line(input);
    println!("part 1: {}", part_2_inner(grid))
}

fn part_1_inner(grid: Grid<char>) -> usize {
    count_xmas(grid.rows())
        + count_xmas(grid.cols())
        + count_xmas(grid.se_diagonals())
        + count_xmas(grid.ne_diagonals())
}

fn count_xmas(iter: impl Iterator<Item = Vec<char>>) -> usize {
    iter.map(|x| {
        let frontwards = x.iter().collect::<String>();
        let backwards = x.iter().rev().collect::<String>();
        frontwards.matches("XMAS").count() + backwards.matches("XMAS").count()
    })
    .sum()
}

fn part_2_inner(grid: Grid<char>) -> usize {
    let mut se_row_cols: Vec<(usize, usize)> =
        (0..grid.n_rows).rev().map(|row_idx| (row_idx, 0)).collect();
    se_row_cols.extend((1..grid.n_cols).map(|col_idx| (0, col_idx)));
    let se_centers: HashSet<(usize, usize)> = se_row_cols
        .into_iter()
        .flat_map(|(row_idx, col_idx)| {
            let diag = grid
                .se_diagonal(row_idx, col_idx)
                .into_iter()
                .collect::<String>();
            let mut hits = diag
                .match_indices("MAS")
                .map(move |(idx, _)| (row_idx + idx + 1, col_idx + idx + 1))
                .collect::<Vec<(usize, usize)>>();
            hits.extend(
                diag.match_indices("SAM")
                    .map(move |(idx, _)| (row_idx + idx + 1, col_idx + idx + 1)),
            );

            hits
        })
        .collect();

    let mut ne_row_cols: Vec<(usize, usize)> =
        (0..grid.n_rows).map(|row_idx| (row_idx, 0)).collect();
    ne_row_cols.extend((1..grid.n_cols).map(|col_idx| (grid.n_rows - 1, col_idx)));
    let ne_centers: HashSet<(usize, usize)> = ne_row_cols
        .into_iter()
        .flat_map(|(row_idx, col_idx)| {
            let diag = grid
                .ne_diagonal(row_idx, col_idx)
                .into_iter()
                .collect::<String>();
            let mut hits = diag
                .match_indices("MAS")
                .map(move |(idx, _)| (row_idx - idx - 1, col_idx + idx + 1))
                .collect::<Vec<(usize, usize)>>();
            hits.extend(
                diag.match_indices("SAM")
                    .map(move |(idx, _)| (row_idx - idx - 1, col_idx + idx + 1)),
            );

            hits
        })
        .collect();

    se_centers
        .intersection(&ne_centers)
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let grid = char_grid_from_line(
            vec![
                "MMMSXXMASM",
                "MSAMXMSMSA",
                "AMXSXMAAMM",
                "MSAMASMSMX",
                "XMASAMXAMM",
                "XXAMMXXAMA",
                "SMSMSASXSS",
                "SAXAMASAAA",
                "MAMMMXMMMM",
                "MXMXAXMASX",
            ]
            .into_iter()
            .map(|x| x.to_string()),
        );
        assert_eq!(part_1_inner(grid), 18);
    }

    #[test]
    fn test_part_2() {
        let grid = char_grid_from_line(
            vec![
                "MMMSXXMASM",
                "MSAMXMSMSA",
                "AMXSXMAAMM",
                "MSAMASMSMX",
                "XMASAMXAMM",
                "XXAMMXXAMA",
                "SMSMSASXSS",
                "SAXAMASAAA",
                "MAMMMXMMMM",
                "MXMXAXMASX",
            ]
            .into_iter()
            .map(|x| x.to_string()),
        );
        assert_eq!(part_2_inner(grid), 9);
    }
}
