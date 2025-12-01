use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let lines = contents.split("\n");
    let mut dial: i32 = 50;
    let mut n0 = 0;
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let direction = line.chars().nth(0).unwrap();
        let rotations = line[1..].parse::<i32>().unwrap();
        for _ in 0..rotations {
            if direction == 'L' {
                dial -= 1;
            } else if direction == 'R' {
                dial += 1;
            };
            if dial.rem_euclid(100) == 0 {
                n0 += 1
            }
        }
    }
    println!("Number of times dial passed 0: {n0}");
}
