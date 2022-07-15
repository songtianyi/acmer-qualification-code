#[allow(unused_imports)]
use std::cmp::max;
#[allow(unused_imports)]
use std::cmp::min;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;
use std::io;

// input macros
#[allow(unused_macros)]

macro_rules! read {
    // eg.
    // let s = read!();
    () => {{
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().to_string()
    }};

    // eg.
    // let v = read!(Vec<i32>)
    // let v = read!(Vec<char>)
    (Vec<$t:ty>) => ({
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<$t>().unwrap())
            .collect();
    });

    // eg.
    // let v = read!(i32);
    // let v = read!(i64);
    // let (i, j, k) = read!(i32, i32, i32);
    ($($t:ty),*) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()),*)
    }};

}

fn solve() {
    let (n, c, q) = read!(i32, i32, i32);

    let s = read!();
    // init implicit first round
    let mut round_left: Vec<i64> = vec![0i64; c as usize + 1];
    round_left[0] = 0;
    let mut round_right: Vec<i64> = vec![0i64; c as usize + 1];
    round_right[0] = n as i64; // not include
    let mut round_diff: Vec<i64> = vec![0i64; c as usize + 1];
    round_diff[0] = 0;

    for cr in 1..c + 1 {
        let (l, r) = read!(i64, i64);
        // for each round
        let cru = cr as usize;
        round_left[cru] = round_right[cru - 1];
        round_right[cru] = round_left[cru] + r - l + 1;
        round_diff[cru] = round_left[cru] - l + 1;
        //println!("{} {} {}", round_left[cru], round_right[cru], round_diff[cru]);
    }
    for i in 0..q {
        let mut pos = read!(i64) - 1;
        for cr in (1..c + 1).rev() {
            let cru = cr as usize;
            if pos < round_left[cru] {
                continue;
            } else {
                pos -= round_diff[cru];
            }
        }
        //println!("{}", pos);
        print!("{}\n", s.chars().nth((pos) as usize).unwrap());
    }
}

fn main() {
    let mut t = read!(i32);
    while t > 0 {
        t -= 1;
        solve();
        //print!("\n");
    }
}
