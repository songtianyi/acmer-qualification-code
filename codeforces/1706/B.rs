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
    let a: Vec<i32> = read!(Vec<i32>);
    let mut ans: Vec<usize> = vec![0; (n+1) as usize];
    let mut dp: Vec<usize> = vec![0; (n+1) as usize];
    
    for (i, x) in a.iter().enumerate() {
        let j = i + 1;
        let v = *x as usize;
        if dp[v] == 0 {
            // no value before
            dp[v] = j;
            ans[v] += 1;
        }else{
            let diff = j - dp[v] - 1; // different colors
            if diff % 2 == 0 {
                ans[v] += 1;
                dp[v] = j;
            }
        }
        //println!("dp -{:?}", dp);
        //println!("ans-{:?}\n", ans);
    }
 
    for i in 1..=n {
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