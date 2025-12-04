use ndarray::Array2;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let rows = contents.split("\n").collect::<Vec<&str>>();
    let m = rows.len();
    let n = rows[0].len();
    let mut room = Array2::<u64>::default((m, n));
    for i in 0..m {
        let rows_chars = rows[i].chars().collect::<Vec<char>>();
        for j in 0..n {
            let cell = rows_chars[j];
            match cell {
                '@' => room[[i, j]] = 1,
                _ => room[[i, j]] = 0,
            };
        }
    }
    let mut total_accessible = 0;
    loop {
        let mut room_clone = room.clone();
        let mut accessible: u64 = 0;
        for i in 0..m {
            for j in 0..n {
                if is_removable(&room_clone, i, j) {
                    accessible += 1;
                }
            }
        }
        if accessible == 0 {
            break;
        } else {
            println!("accessible {accessible}");
            for i in 0..m {
                for j in 0..n {
                    if is_removable(&room_clone, i, j) {
                        room[[i, j]] = 0
                    }
                }
            }
            total_accessible += accessible;
        }
    }
    println!("total accessible {total_accessible}");
}

fn is_removable(room: &Array2<u64>, i: usize, j: usize) -> bool {
    let mut neighbors = 0;
    for k in 0..=2 {
        for l in 0..=2 {
            if i + k >= 1 && j + l >= 1 && (k, l) != (1, 1) {
                let u: usize = (i + k - 1).try_into().unwrap();
                let v: usize = (j + l - 1).try_into().unwrap();
                match room.get((u, v)) {
                    Some(n) => neighbors += n,
                    None => (),
                }
            }
        }
    }
    return room[[i, j]] == 1 && neighbors < 4;
}
