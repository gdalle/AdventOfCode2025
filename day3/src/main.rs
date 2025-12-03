use std::collections::HashMap;
use std::{cmp::max, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let banks = contents.split("\n");
    let mut max_joltage = 0;
    for bank in banks {
        let batteries = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        println!("batteries {batteries:?}");
        let j = bank_joltage(&batteries[..]);
        println!("{j}");
        max_joltage += j;
    }
    println!("Max joltage {max_joltage}");
}

fn bank_joltage(batteries: &[u32]) -> i64 {
    let mut subbank_joltage = HashMap::new();
    let n = batteries.len();
    for last_index in 0..n {
        subbank_joltage.insert((last_index, 0), 0);
    }
    for max_batteries in 0..n {
        subbank_joltage.insert((0, max_batteries), 0);
    }
    for last_index in 1..(n + 1) {
        for max_batteries in 1..=12 {
            // not using that battery
            let j1: i64 = *subbank_joltage
                .get(&(last_index - 1, max_batteries))
                .unwrap();
            // using that battery
            let j2: i64 = i64::from(batteries[last_index - 1])
                + 10 * *subbank_joltage
                    .get(&(last_index - 1, max_batteries - 1))
                    .unwrap();
            let j = max(j1, j2);
            subbank_joltage.insert((last_index, max_batteries), j);
        }
    }
    return *subbank_joltage.get(&(n, 12)).unwrap();
}
