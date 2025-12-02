use std::{cmp, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let ranges = contents.split(",");
    let mut invalid_ids = 0;
    for range in ranges {
        let parts: Vec<&str> = range.split("-").collect();
        let start = parts[0].parse::<u64>().unwrap();
        let end = parts[1].parse::<u64>().unwrap();
        for n in start..(end + 1) {
            let nc = to_digits(n);
            let l = nc.len();
            let mut repeated = false;
            for d in 1..l {
                if l % d == 0 {
                    let nc1 = &nc[..d];
                    let mut repeated_divisor = true;
                    for k in 1..(l / d) {
                        let nc2 = &nc[(k * d)..((k + 1) * d)];
                        if !equal_vecs(nc1, nc2) {
                            repeated_divisor = false;
                        }
                    }
                    repeated = repeated || repeated_divisor;
                }
            }
            if repeated {
                println!("Invalid {n}");
                invalid_ids += n;
            }
        }
    }
    println!("Invalid IDs: {invalid_ids}");
}

fn to_digits(n: u64) -> Vec<char> {
    return n.to_string().chars().collect();
}

fn equal_vecs<T: cmp::PartialEq>(v1: &[T], v2: &[T]) -> bool {
    let l = v1.len();
    for i in 0..l {
        if v1[i] != v2[i] {
            return false;
        }
    }
    return true;
}
