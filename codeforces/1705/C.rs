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
            .collect();
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
    let (n,c,q) = read!(i32,i32,i32);

    let s = read!();
    let mut m: HashMap<char, Vec<i64>> = HashMap::new();
    for (i, ch) in s.chars().enumerate() {
        let v = m.entry(ch).or_insert(vec![]);
        v.push(i as i64 + 1);
    }
    //println!("{:?}", m);

    let mut sum = n as i64;
    for round in 0..c {
        let (l, r) = read!(i64, i64);
        //print!("{} {}\n", l, r);
        // for each round
        for cu in 97..123 {
            if !m.contains_key(&(cu as u8 as char)) {
                continue;
            }
            let v = m.get_mut(&(cu as u8 as char)).unwrap();
            let mut app: Vec<i64> = vec![];
            for (_, x) in v.iter().enumerate() {
                if *x >= l && *x <= r {
                    app.push(x-l+1+sum);
                }
            }
            v.extend(app);
            v.sort();
        }
        sum += (r-l+1) as i64;
        //println!("{:?}", m);
    }
    for i in 0..q {
        let pos = read!(i64);
        //println!("pos-{}", pos);
        for (k, v) in m.iter() {
            let ok = v.binary_search(&pos).is_ok();
            if ok {
                println!("{}", k);
                break;
            }
        }
    }
}

fn main() {
    let mut t = read!(i32);
    while t > 0 {
        t -= 1;
        solve();
        //print!("\n");
    }
}
