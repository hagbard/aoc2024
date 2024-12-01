use std::collections::HashMap;
use std::hash::Hash;

pub fn count_occurrences_by_ref<'a, I, T: 'a>(vals: I) -> HashMap<&'a T, usize>
    where
        I: Iterator<Item=&'a T>,
        T: Eq + PartialEq + Hash
{
    let mut m: HashMap<&'a T, usize> = HashMap::new();
    vals.for_each(|v| { m.entry(v).and_modify(|n| *n += 1).or_insert(1); });
    m
}

pub fn count_occurrences_by_value<I, T>(vals: I) -> HashMap<T, usize>
    where
        I: Iterator<Item=T>,
        T: Eq + PartialEq + Hash + Copy
{
    let mut m: HashMap<T, usize> = HashMap::new();
    vals.for_each(|v| { m.entry(v).and_modify(|n| *n += 1).or_insert(1); });
    m
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_by_ref() {
        let v: Vec<&str> = vec!["A", "B", "B", "C", "C", "C", "C", "D", "D"];
        let counts: HashMap<&&str, usize> = count_occurrences_by_ref(v.iter());
        assert_eq!(counts[&"A"], 1);
        assert_eq!(counts[&"B"], 2);
        assert_eq!(counts[&"C"], 4);
        assert_eq!(counts[&"D"], 2);
        assert_eq!(counts.get(&"X"), None);
    }

    #[test]
    fn test_by_value() {
        let v: Vec<&str> = vec!["A", "B", "B", "C", "C", "C", "C", "D", "D"];
        let counts: HashMap<&str, usize> = count_occurrences_by_value(v.into_iter());
        assert_eq!(counts["A"], 1);
        assert_eq!(counts["B"], 2);
        assert_eq!(counts["C"], 4);
        assert_eq!(counts["D"], 2);
        assert_eq!(counts.get("X"), None);
    }
}
