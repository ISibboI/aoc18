use crate::io::load_input;

pub fn day1() {
    let input = load_input("day1");
    let mut sum = 0;

    for line in input {
        let n: i32 = line.parse().unwrap();
        sum += n;
    }

    info!("Result of day1: {}", sum);
}