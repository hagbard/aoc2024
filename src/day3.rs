use lazy_regex::{Lazy, lazy_regex, Regex};

pub static MUL_RE: Lazy<Regex> = lazy_regex!(r"mul\((\d+),(\d+)\)");

pub fn run(input: &str) -> (u64, u64) {
    let part1 = count_mul(input);
    let part2 = input
        .split("do()")
        .map(|s| s.splitn(2, "don't()").next().unwrap())
        .map(|s| count_mul(s))
        .sum();
    (part1, part2)
}

fn count_mul(s: &str) -> u64 {
    MUL_RE.captures_iter(s).map(|c| c.extract())
        .map(|(_, [a, b])| atou64(a) * atou64(b)).sum::<u64>()
}

fn atou64(s: &str) -> u64 {
    s.parse::<u64>().unwrap()
}
