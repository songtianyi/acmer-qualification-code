use std::collections::HashSet;
use std::io;

// input macros
#[allow(unused_macros)]

macro_rules! read {
    () => ({
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().to_string()
    }
    );
    ($t:ty) => ({
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<$t>().unwrap()
    });
    ($v:ident as $t:ty) => ({
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();
        $v = line.trim().parse::<$t>().unwrap();
    });
    ($($v:ident),*) => {
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        $(
            let $v = iter.next().unwrap().parse::<$t>().unwrap();
        )*
    };

}

fn main() {
    let mut t: i32 = read!(i32);
    while t > 0 {
        let n: i32;
        read!(n as i32);
        let mut strs: Vec<String> = vec![];
        let mut strset = HashSet::<&str>::new();
        for _i in 0..n {
            strs.push(read!().clone());
        }
        for i in 0..n {
            strset.insert(&strs[i as usize]);
        }
        for i in 0..n {
            let mut found = false;
            for l in 1..strs[i as usize].len() {
                let s: &str = strs[i as usize].get(0..l).unwrap();
                let t: &str = strs[i as usize].get(l..).unwrap();
                if strset.contains(s) && strset.contains(t) {
                    print!("{}", 1);
                    found = true;
                    break;
                }
            }
            if !found {
                print!("{}", 0);
            }
        }
        print!("\n");
        t -= 1;
    }
}
