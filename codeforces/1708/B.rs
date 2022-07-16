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
    let (n, l, r) = read!(i64, i64, i64);
    let mut ans: Vec<i64> = vec![0; (n + 1) as usize];
    // we need to constuct a array with n number which gcd(i, a[i]]) = i
    // casuse a[n] = {1, 2, 3, 4, .. n} is the smallnest numbers that fullfill the condition
    // gcd(i, i) = i, if we increase one of them, the answer alwayes be i
    // eg gcd(6, 6) = 6, gcd(6, 12) = 6 .. gcd(6, 6*x) = 6
    // then we just find the a[i] in range [l,r]
    // if l%i == 0 means l is the a[i] we want
    // else we need shit another i from l/i
    for i in 1..n + 1 {
        let x = l / i + (l % i != 0) as i64;
        if x * i > r {
            print!("NO");
            return;
        }
        ans[i as usize] = x * i;
    }
    println!("YES");
    for i in 1..n + 1 {
        print!("{} ", ans[i as usize]);
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
