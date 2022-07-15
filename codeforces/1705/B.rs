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
    // defualt read as string
    // let s = read!();
    () => {{
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().to_string()
    }};
    // read single certain type value
    // let v = read!(i32)
    // let v = read!(i64)
    ($t:ty) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<$t>().unwrap()
    }};
    // read as Vec<> with certain type and name
    // let a: Vec<i32>;
    // read!(a as Vec<i32>);
    ($v:ident as Vec<$t:ty>) => {{
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();
        $v = line
            .split_whitespace()
            .map(|x| x.parse::<$t>().unwrap())
            .collect();
    }};
    // read with type and variable name
    ($v:ident as $t:ty) => {{
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();
        $v = line.trim().parse::<$t>().unwrap();
    }};
}

fn solve() {
    let n = read!(i32);
    let mut a: Vec<i64> = vec![];
    read!(a as Vec<i64>);
    let mut ans: i64 = 0;
    let mut last: i64 = 0;
    let mut is_first = true;
    for i in 0..n-1 {
        if a[i as usize] != 0 {
            last = a[i as usize];
            is_first = true;
            ans += last;
        }else {
            if is_first && last != 0{
                is_first = false;
                ans+=1;
            }else{
                ans += last;
            }
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