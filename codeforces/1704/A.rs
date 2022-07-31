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
    let a: Vec<char> = read!().chars().collect();
    let b: Vec<char>  = read!().chars().collect();
    if a.len() == b.len() {
        if a == b {
            print!("YES");
        }else {
            print!("NO");
        }
        return;
    }

    let ml = min(a.len(), b.len());


    if a[a.len() - ml + 1..] == b[b.len() - ml + 1..] {
        let ai = (a.len() - ml) as usize;
        let bi = (b.len() - ml) as usize;

                    let ok = a[..ai+1].iter().any(|x| x == &b[bi]);
                    if ok {
                        print!("YES");
                    } else {
                        print!("NO");
                    }

    } else {
        print!("NO");
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