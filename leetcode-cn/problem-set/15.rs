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

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut b = nums.clone();
        b.sort();
        println!("{:?}", b);
        let mut exist: HashSet<String> = HashSet::new();
        let mut ans: Vec<Vec<i32>> = vec![];
        for i in 0..b.len() - 2 {
            if b[i] > 0 {
                break;
            }
            for j in i + 1..b.len() - 1 {
                if b[i] + b[j] > 0 {
                    break;
                }
                let ak = -(b[i] + b[j]);
                let k = &b[j + 1..].binary_search(&(ak));
                if !k.is_ok() {
                    continue;
                }
                let key = b[i].to_string() + &b[j].to_string() + &ak.to_string();
                if !exist.contains(&key) {
                    ans.push(vec![b[i], b[j], ak]);
                    exist.insert(key);
                }
            }
        }
        println!("{:?}", ans);
        ans
    }
}

fn case1() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let res = Solution::three_sum(nums);
    assert_eq!(vec![vec![-1, -1, 2], vec![-1, 0, 1]], res);
}

fn case2() {
    let nums = vec![0, 0, 0];
    let res = Solution::three_sum(nums);
    assert_eq!(vec![vec![0, 0, 0]], res);
}

fn case3() {
    let nums = vec![];
    let res = Solution::three_sum(nums);
    let target: Vec<Vec<i32>> = vec![];
    assert_eq!(target, res);
}

#[test]
fn test() {
    // rustc --test 15.rs; ./15
    case1();
    case2();
    case3();
}
