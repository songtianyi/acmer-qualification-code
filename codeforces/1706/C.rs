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
    let a: Vec<i64> = read!(Vec<i64>);
    let mut ans: Vec<i64> = vec![0; (n + 1) as usize];

    for i in 1..n as usize - 1 {
        let max = max(a[i - 1], a[i + 1]);
        if max >= a[i] {
            ans[i] = (a[i] - max).abs() + 1;
        }
    }
    println!("{:?}", ans);

    let mut total1 = 0;
    let mut total2 = 0;
    let mut i = 1 as usize;
    while i < n as usize - 1 {
        if i % 2 == 1 {
            total1 += ans[i];
        } else {
            total2 += ans[i];
        }
        i += 1;
    }
    if n % 2 == 0 {
        let mut minv = min(total1, total2);
        // sep
        if n == 6 {
            let mut j = 1 as usize;
            let mut sep1 = 0;
            let mut sep2 = 0;
            while j < (n / 2 - 1) as usize {
                sep1 += ans[j];
                j += 2;
            }
            j += 1;
            while j < (n - 1) as usize {
                sep2 += ans[j];
                j += 2;
            }
            minv = min(sep1 + sep2, minv);
        }
        print!("{}", minv);
    } else {
        print!("{}", total1);
    }
}

fn main() {
    let mut t = read!(i32);
    while t > 0 {
        t -= 1;
        solve();
        print!("\n");
    }
}
