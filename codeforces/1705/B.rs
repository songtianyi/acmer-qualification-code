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
    let n = read!(i32);
    let mut a: Vec<i64> = read!(Vec<i64>);
    a.pop();
    let mut ans: i64 = 0;

    let idx = a.iter().position(|x| *x != 0);
    if idx.is_none() {
        print!("0");
        return;
    }
    let start = idx.unwrap();
    for i in start..(n - 1) as usize {
        if a[i as usize] != 0 {
            ans += a[i as usize];
        } else {
            ans += 1;
        }
    }
    print!("{}", ans);
}

fn main() {
    let mut t = read!(i32);
    while t > 0 {
        t -= 1;
        solve();
        print!("\n");
    }
}
