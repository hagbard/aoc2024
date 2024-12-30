use std::cmp::Ordering;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

struct PageOrder {
    successor_masks: [u128; 100],
}

fn mask_of(n: u32) -> u128 {
    1u128 << n
}

impl PageOrder {
    fn parse(lines: &str) -> PageOrder {
        let mut order: PageOrder = PageOrder { successor_masks: [0u128; 100] };
        for (lhs, rhs) in lines.lines().map(|line| line.split_once('|').unwrap()) {
            match (lhs.parse::<u32>(), rhs.parse::<u32>()) {
                (Ok(a), Ok(b)) if a < 100 && b < 100 && a != b => order.set_succ(a, b),
                _ => panic!("Cannot parse input!")
            }
        }
        order
    }

    fn is_sorted(&self, v: &Vec<u32>) -> bool {
        // Ending in the Done state means we encountered an allowed successor element which had
        // already been seen (i.e. a predeccessor), so we were NOT ordered properly.
        !v.iter().fold_while(0u128, |seen, &n| {
            // Test the set of seen indicies against the allowed successors for each element.
            if self.is_any_succ_of(n, seen) { Done(0) } else { Continue(seen | mask_of(n)) }
        }).is_done()
    }

    fn compare(&self, a: u32, b: u32) -> Ordering {
        match (self.is_succ_of(a, b), self.is_succ_of(b, a)) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) if a == b => Ordering::Equal,
            _ => panic!("Indirect ordering encountered (unexpected revision data)!")
        }
    }

    fn set_succ(&mut self, a: u32, b: u32) {
        if a == b { panic!("Direct cycle detected (bad ordering data)"); }
        self.successor_masks[a as usize] |= mask_of(b);
    }

    fn is_succ_of(&self, a: u32, b: u32) -> bool {
        self.is_any_succ_of(a, mask_of(b))
    }

    fn is_any_succ_of(&self, a: u32, mask: u128) -> bool {
        self.successor_masks[a as usize] & mask != 0
    }
}

// Note: The data to this puzzle is "kind" insofar as there is never any need to recursively resolve
// successors in order to determine ordering. It's "unkind" in the sense that the ordering contains
// cycles when considered as a whole (i.e. you cannot preprocess it to make a total ordering).
pub fn run(input: &str) -> (u32, u32) {
    let (ord, revs) = input.split_once("\n\n").unwrap();

    let order = PageOrder::parse(ord);
    let revisions: Vec<Vec<u32>> = revs.lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    let part1 = revisions.iter()
        .filter(|&v| order.is_sorted(v))
        .map(|v| v[(v.len() - 1) / 2])
        .sum();
    let part2 = revisions.iter()
        .filter(|&v| !order.is_sorted(v))
        .map(|v| v.iter().sorted_by(|&&a, &&b| order.compare(a, b)).collect_vec())
        .map(|v| v[(v.len() - 1) / 2])
        .sum();
    (part1, part2)
}
