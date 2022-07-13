
use std::io;
#[allow(unused_imports)]
use std::collections::HashSet;


// input macros
#[allow(unused_macros)]

macro_rules! read {
    // defualt read as string
    () => ({
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().to_string()
    }
    );
    // read single certain type value
    ($t:ty) => ({
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<$t>().unwrap()
    });
    // read as Vec<> with certain type and name
    ($v:ident as Vec<$t:ty>) => ({
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();
        $v = line.split_whitespace().map(|x| x.parse::<$t>().unwrap()).collect();
    });
    // read with type and variable name
    ($v:ident as $t:ty) => ({
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();
        $v = line.trim().parse::<$t>().unwrap();
    });
}

fn main() {
    let mut t: i32 = read!(i32);
    while t > 0 {
        t-=1;
        let n: i32;
        read!(n as i32);
        let a: Vec<i32>;
        read!(a as Vec<i32>);
        let mut b: Vec<i32> = vec![];
        let (mut pos, mut neg, mut zero) = (0, 0, 0);
        a.iter().for_each(|&x| {
            if x > 0 {
                pos += 1;
                b.push(x);
            } else if x < 0 {
                neg += 1;
                b.push(x);
            }else {
                zero += 1;
                if zero < 2 {
                    b.push(0);
                }
            }
        });

        if pos > 2 || neg > 2 {
            println!("NO");
            continue;
        }

        let mut ok = true;
        let si = b.len();
        'outer: for i in 0..si {
            for j in i+1..si {
                for k in j+1..si {
                    ok = b.iter().any(|x| {
                        *x == b[i as usize] + b[j as usize] + b[k as usize]
                    });
                    if !ok {
                        println!("NO");
                        break 'outer;
                    }
                }
            }
        }
        if ok {
            println!("YES");
        }
    }
}