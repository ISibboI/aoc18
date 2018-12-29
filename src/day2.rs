use crate::io::load_input;
use multiset::HashMultiSet;

pub fn day2_1() {
    let input = load_input("day2");

    let mut doubles = 0;
    let mut triples = 0;

    for line in input {
        let mut counts = HashMultiSet::new();

        for c in line.chars() {
            counts.insert(c);
        }

        let mut has_doubles = false;
        let mut has_triples = false;
        for &entry in counts.distinct_elements() {
            let count = counts.count_of(entry);
            if count == 2 {
                has_doubles = true;
            }
            if count == 3 {
                has_triples = true;
            }
        }

        if has_doubles {
            doubles += 1;
        }
        if has_triples {
            triples += 1;
        }
    }

    let result = doubles * triples;
    info!("Result of day2_1: {}", result);
}

pub fn day2_2() {
    let input = load_input("day2");

    for (index, l1) in input.iter().enumerate() {
        for l2 in input.iter().skip(index + 1) {
            if l1.len() != l2.len() {
                continue;
            }

            let differences = l1.chars().zip(l2.chars()).filter(|(c1, c2)| c1 != c2).count();
            if differences == 1 {
                let result: String = l1.chars().zip(l2.chars()).filter(|(c1, c2)| c1 == c2).map(|(c, _)| c).collect();
                info!("Result of day2_2: {}", result);
                return;
            }
        }
    }
}