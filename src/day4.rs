use itertools::Itertools;
use strum::IntoEnumIterator;
use crate::grid::{parse_lines, Grid, GridLike};
use crate::point::{Dir, Point, GridIter, PointLike};
use crate::point::Dir::{LeftDown, LeftUp, RightDown, RightUp};

pub fn run(input: &str) -> (usize, usize) {
    let grid = parse_lines(input);
    let part1 = grid.indexed_iter()
        .filter(|(_, &c)| c == 'X')
        .map(|(p, _)| count_xmas(&grid, &Point::of(&p)))
        .sum();
    let part2 = grid.indexed_iter()
        .filter(|(p, &c)| c == 'A' && is_x_mas(&grid, &Point::of(p)))
        .count();
    (part1, part2)
}

fn count_xmas(grid: &Grid, p: &Point) -> usize {
    Dir::iter()
        .filter(|&d| is_mas(
            // Step 1 forward to read after 'X'.
            GridIter::new(p.step(d, 1), d).map_while(|p| grid.at(&p)).next_tuple()))
        .count()
}

fn is_x_mas(grid: &Grid, p: &Point) -> bool {
    // grid[p] == 'A', an X is formed if 2 diagonals read "M A S".
    [RightUp, LeftUp, LeftDown, RightDown].into_iter()
        .filter(|&d| is_mas(
            // Step 1 backward to read before 'A'.
            GridIter::new(p.step(d, -1), d).map_while(|p| grid.at(&p)).next_tuple()))
        .count() == 2
}

fn is_mas(p: Option<(char, char, char)>) -> bool {
    match p {
        Some((m, a, s)) if m == 'M' && a == 'A' && s == 'S' => true,
        _ => false
    }
}
