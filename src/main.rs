use std::fmt::Debug;
use std::fs::read_to_string;
use std::time::Instant;

mod day1;
mod multiset;

fn main() {
    let mut total_micros: u128 = 0;
    total_micros += time(1, &day1::run);
    println!("Total time: {} µs", total_micros);
}

fn time<I: Debug>(num: u32, fnc: &dyn Fn(&str) -> (I, I)) -> u128 {
    // panic on possible file-reading errors
    let input =
        read_to_string(&format!("day{}.txt", num)).unwrap().replace("\r\n", "\n");
    let start = Instant::now();
    let (p1, p2) = fnc(&input);
    let taken = start.elapsed();
    let taken_micros = taken.as_micros();
    println!("Day {} [{} µs] (1) = {:?}, (2) = {:?}", num, taken_micros, p1, p2);
    taken_micros
}
