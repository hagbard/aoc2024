use std::collections::HashMap;
use itertools::Itertools;

pub fn run(input: &str) -> (usize, usize) {
    let lines: Vec<&str> = input.lines().collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for &line in &lines {
        let mut diffs = diffs(&line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect_vec());

        let correctable_values = normalize_diffs(&mut diffs);
        // Requires more than one correction. Don't even attempt it.
        if correctable_values > 1 { continue; }

        // Sequence contains at most a single "bad" value, either zero change or decrementing.
        let mut needed_correction = false;
        // The non-incrementing entry can be merged either left or right, or just removed.
        if correctable_values == 1 {
            correct_once(&mut diffs);
            needed_correction = true;
        }
        if is_safe(&diffs) {
            part2 += 1;
            if !needed_correction { part1 += 1 }
        }
    }
    (part1, part2)
}

fn diffs(values: &Vec<i32>) -> Vec<i32> {
    values.iter().tuple_windows().map(|(a, b)| b - a).collect_vec()
}

fn is_safe(diffs: &Vec<i32>) -> bool {
    diffs.iter().all(|n| (1..=3).contains(n))
}

fn normalize_diffs(diffs: &mut Vec<i32>) -> usize {
    let signums: HashMap<i32, usize> = diffs.iter().map(|n| n.signum()).counts();
    let inc = *signums.get(&1).unwrap_or(&0);
    let dec = *signums.get(&-1).unwrap_or(&0);
    let zero = *signums.get(&0).unwrap_or(&0);
    if dec > inc {
        diffs.iter_mut().for_each(|n| *n = -*n);
        inc + zero
    } else {
        dec + zero
    }
}

fn correct_once(diffs: &mut Vec<i32>) {
    let (i, &bad) = diffs.iter().find_position(|&&n| n <= 0).unwrap();
    if i == 0 {
        let rhs = diffs[i + 1];
        if rhs + bad > 0 { diffs[i + 1] += bad }
    } else if i == diffs.len() - 1 {
        let lhs = diffs[i - 1];
        if lhs + bad > 0 { diffs[i - 1] += bad }
    } else {
        let lhs = diffs[i - 1];
        let rhs = diffs[i + 1];
        if rhs > lhs {
            diffs[i + 1] += bad;
        } else {
            diffs[i - 1] += bad;
        }
    }
    diffs.remove(i);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_safe_diffs() {
        // 7 6 4 2 1: Safe without removing any level.
        let mut foo: Vec<i32> = diffs(&vec![7, 6, 4, 2, 1]);
        assert_eq!(normalize_diffs(&mut foo), 0);
        assert_eq!(foo, vec![1, 2, 2, 1]);
        assert_eq!(is_safe(&foo), true);

        // 1 3 6 7 9: Safe without removing any level.
        let mut bar: Vec<i32> = diffs(&vec![1, 3, 6, 7, 9]);
        assert_eq!(normalize_diffs(&mut bar), 0);
        assert_eq!(bar, vec![2, 3, 1, 2]);
        assert_eq!(is_safe(&bar), true);
    }

    #[test]
    fn test_unsafe_diffs() {
        // 1 2 7 8 9: Unsafe regardless of which level is removed.
        let mut foo: Vec<i32> = diffs(&vec![1, 2, 7, 8, 9]);
        assert_eq!(normalize_diffs(&mut foo), 0);
        assert_eq!(foo, vec![1, 5, 1, 1]);
        assert_eq!(is_safe(&foo), false);

        // 9 7 6 2 1: Unsafe regardless of which level is removed.
        let mut bar: Vec<i32> = diffs(&vec![9, 7, 6, 2, 1]);
        assert_eq!(normalize_diffs(&mut bar), 0);
        assert_eq!(bar, vec![2, 1, 4, 1]);
        assert_eq!(is_safe(&bar), false);
    }

    #[test]
    fn test_correctable_diffs() {
        // 1 3 2 4 5: Safe by removing the second level, 3.
        let mut foo: Vec<i32> = diffs(&vec![1, 3, 2, 4, 5]);
        assert_eq!(normalize_diffs(&mut foo), 1);
        assert_eq!(foo, vec![2, -1, 2, 1]);
        assert_eq!(is_safe(&foo), false);
        correct_once(&mut foo);
        assert_eq!(foo, vec![1, 2, 1]);
        assert_eq!(is_safe(&foo), true);

        // 8 6 4 4 1: Safe by removing the third level, 4.
        let mut bar: Vec<i32> = diffs(&vec![8, 6, 4, 4, 1]);
        assert_eq!(normalize_diffs(&mut bar), 1);
        assert_eq!(bar, vec![2, 2, 0, 3]);
        assert_eq!(is_safe(&bar), false);
        correct_once(&mut bar);
        assert_eq!(bar, vec![2, 2, 3]);
        assert_eq!(is_safe(&bar), true);
    }

    #[test]
    fn test_corrected() {
        let mut foo: Vec<i32> = vec![-2, 1, 1, 1];
        correct_once(&mut foo);
        assert_eq!(foo, vec![1, 1, 1]);
        assert_eq!(is_safe(&foo), true);

        let mut bar: Vec<i32> = vec![-4, 6, 1, 1];
        correct_once(&mut bar);
        assert_eq!(bar, vec![2, 1, 1]);
        assert_eq!(is_safe(&bar), true);

    //     assert_eq!(is_incrementing_safe(&vec![5, 1, 1, 1], true), true);
    //     assert_eq!(is_incrementing_safe(&vec![-2, 1, 2, 3], false), true);
    //     assert_eq!(is_incrementing_safe(&vec![3, -2, 2, 1], false), true);
    }
}

