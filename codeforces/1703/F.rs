#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;
use std::io;
#[allow(unused_imports)]
use std::cmp::min;
use std::cmp::max;

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
    let a: Vec<i32>;
    read!(a as Vec<i32>);
    let mut dp = vec![0; (n + 1) as usize];
    for (i, &x) in a.iter().enumerate() {
        dp[i + 1 as usize] = dp[i] + ((i as i32 +1) > x) as i32;
    }

    let mut ans = 0i64;
    for (i, &x) in a.iter().enumerate() {
        if i as i32 + 1 > x {
            // 重点是 x - 1
            // 因为要求 i < a_j, 所以满足要求的要从 a_j - 1 开始，因为都是整数嘛 a_j-1 就已经满足了
            // 所以从这里开始计算
            // 取最大值是为了处理 a_j 为 0 的情况
            ans += dp[(max(x-1, 0)) as usize] as i64;
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