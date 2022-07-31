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

// rustc +nightly -Zunpretty=expanded your.rs
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
            .collect()
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
    let (n, m) = read!(i64, i32);
    let mut a: Vec<i64> = read!(Vec<i64>);
    if a.len() == 1 {
        print!("{}", 2);
        return;
    }
    a.sort();
    let mut diff: Vec<i64> = vec![];
    for (i, x) in a.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if i == a.len() - 1 {
            // last
            diff.push(n - x + a[0] - 1);
        }
        let ui = i as usize;
        diff.push(x - a[ui - 1] - 1);
    }
    diff.sort_by(|a, b| b.cmp(a));
    let mut ans: i64 = 0;
    let mut round = 0;
    for (i, x) in diff.iter().enumerate() {
        let save: i64 = x - round * 2;
        if save <= 0 {
            break;
        }
        if save <= 2 {
            round += 1;
            ans += 1;
        } else {
            ans += max(1, (save - 1));
            round += 2;
        }
    }
    print!("{}", n - ans);
}

fn main() {
    let mut t = read!(i32);
    while t > 0 {
        t -= 1;
        solve();
        print!("\n");
    }
}
