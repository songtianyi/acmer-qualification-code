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
    let (n, m) = read!(i32, i32);
    let a: Vec<i32> = read!(Vec<i32>);
    let mut ans: Vec<i32> = vec![0; (m+1) as usize];
    a.iter().for_each(|x| {
        let min = min(*x, m - *x + 1);
        let max = max(*x, m - *x + 1);
        if ans[min  as usize] == 0 {
            ans[min as usize] += 1;
        } else {
            ans[max as usize] += 1;
        }
    });
    &ans[1..].iter().for_each(|x| {
        if *x == 0 {
            print!("B");
        } else {
            print!("A");
        }
    });
}

fn main() {
    let mut t = read!(i32);
    while t > 0 {
        t -= 1;
        solve();
        print!("\n");
    }
}