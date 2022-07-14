#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;
use std::io;
use std::cmp::min;

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
    // Base 1d array
    let mut grid_raw = vec![0; n as usize * n as usize];
    // Vector of 'width' elements slices
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(n as usize).collect();
    // Final 2d array `&mut [&mut [_]]`
    let square = grid_base.as_mut_slice();
    for i in 0..n {
        let a = read!(String);
        let s = a.as_bytes();
        for j in 0..s.len() {
            if s[j] == '1' as u8 {
                square[i as usize][j as usize] = 1;
            }
        }
    }
    let mut ans = 0;
    let un = n as usize;
    let check = n/2 + n%2;
    //println!("{:?}", square);
    for row in 0..check {
        for col in row..(n-row-1){
            let r = row as usize;
            let c = col as usize;

            // .             1 2 .         1   2                  3 .   2 .              3 .    2
            let sum = square[r][c] + square[c][un-r-1] + square[un-r-1][un-c-1] + square[un-c-1][r];
            //println!("{}{} -> {}{} -> {}{} -> {}{}", r, c, c, un-r-1, un-c-1, un-r-1, un-c-1, r);
            ans += min(sum, 4-sum); 
        }
    }
    print!("{}", ans);
    //println!("{:?}", square);
}

fn main() {
    let mut t = read!(i32);
    while t > 0 {
        t -= 1;
        solve();
        print!("\n");
    }
}
