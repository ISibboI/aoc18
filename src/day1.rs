use crate::io::load_input;
use std::collections::HashSet;

pub fn day1_1() {
    let input = load_input("day1_1");
    let mut sum = 0;

    for line in input {
        let n: i32 = line.parse().unwrap();
        sum += n;
    }

    info!("Result of day1_1: {}", sum);
}

pub fn day1_2() {
    let input = load_input("day1_2");
    let mut sum = 0;
    let mut reached_sums = HashSet::new();
    reached_sums.insert(sum);

    for line in input {
        let n: i32 = line.parse().unwrap();
        sum += n;

        if reached_sums.contains(&sum) {
            break;
        } else {
            reached_sums.insert(sum);
        }
    }

    info!("Result of day1_2: {}", sum);
}