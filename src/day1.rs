use itertools::Itertools;

pub fn run(input: &str) -> (usize, usize) {
    let lines: Vec<&str> = input.lines().collect();

    let mut lhs: Vec<usize> = vec!();
    let mut rhs: Vec<usize> = vec!();
    lines.iter()
        .for_each(|line| {
            let (a, b) = split_lr(line);
            lhs.push(a);
            rhs.push(b);
        });
    lhs.sort_unstable();
    rhs.sort_unstable();

    // Sum of absolute difference of all pairs of values in sorted lists.
    let part1 = lhs.iter().zip(&rhs).map(|(&a, &b)| a.abs_diff(b)).sum();

    // Sum over all values of: Value * <Occurrences in first list> * <Occurrences in 2nd list>
    let lc = lhs.iter().counts();
    let rc = rhs.iter().counts();
    let part2 = lc.into_iter().map(|(k, n)| k * n * rc.get(k).unwrap_or(&0)).sum();
    (part1, part2)
}

// Handy util to split once on multiple whitespace and parse into 2-tuple.
fn split_lr(s: &str) -> (usize, usize) {
    s.split_ascii_whitespace().map(|v| v.parse().unwrap()).next_tuple().unwrap()
}
