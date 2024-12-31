use itertools::Itertools;
use strum::IntoEnumIterator;
use crate::grid::{parse_lines, Grid, GridIter};
use crate::point::{Dir, Point, PointLike};

pub fn run(input: &str) -> (usize, usize) {
    let grid = parse_lines(input);
    let part1 = grid.indexed_iter()
        .filter(|&(_, c)| c == 'X')
        .map(|(p, _)| count_xmas(&grid, &p))
        .sum();
    let part2 = grid.indexed_iter()
        .filter(|&(_, c)| c == 'A')
        .filter(|&(p, _)| is_x_mas(&grid, &p))
        .count();
    (part1, part2)
}

fn count_xmas(grid: &Grid, x_pos: &Point) -> usize {
    // All directions (including diagonals).
    Dir::iter()
        // Step forward 1 to read after 'X'.
        .filter(|&d| is_mas(&mut grid.walk(x_pos.step(d, 1), d)))
        .count()
}

fn is_x_mas(grid: &Grid, a_pos: &Point) -> bool {
    // grid[p] == 'A', an X is formed if 2 diagonals read "M A S".
    Dir::diagonals()
        // Step backward 1 to read before 'A'.
        .filter(|&d| is_mas(&mut grid.walk(a_pos.step(d, -1), d)))
        .count() == 2
}

// Alternatively, call next_tuple() on the callers and accept Option<(char, char, char)> here.
fn is_mas(it: &mut GridIter) -> bool {
    match it.next_tuple() {
        Some((m, a, s)) if m == 'M' && a == 'A' && s == 'S' => true,
        _ => false
    }
}
