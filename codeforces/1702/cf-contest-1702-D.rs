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

fn main() {
    let mut t = read!(i32);
    while t > 0 {
        t -= 1;
        let s = read!();
        let p = read!(i32);

        let mut sum = s.chars().fold(0, |acc, x| acc + x as i32 - 'a' as i32 + 1);
        //println!("{}", sum);

        let mut m = HashMap::new();
        for c in s.chars() {
            let v = c as i32 - 'a' as i32 + 1;
            assert!(v >= 1 && v <= 26);
            let count = m.entry(v).or_insert(0);
            *count += 1;
        }
        //println!("{:?}", m);
        for i in (1..27).rev() {
            // print!("{} ", i);
            let count = m.entry(i).or_insert(0);
            //println!("{}: {}", i, count);
            while *count > 0 && sum > p {
                *count -= 1;
                sum -= i;
            }
        }
        //println!("{}", sum);
        //println!("{:?}", m);
        s.chars().for_each(|c| {
            let v = c as i32 - 'a' as i32 + 1;
            let count = m.entry(v).or_insert(0);
            if *count > 0 {
                *count -= 1;
                print!("{}", c);
            }
        });
        print!("\n");
    }
}
