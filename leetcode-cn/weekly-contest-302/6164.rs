const MAX_DS: usize = 100;
pub fn digit_sum(n: i64) -> i64 {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut a = nums.clone();
        let mut sumi: Vec<i32> = vec![-1; MAX_DS];
        let mut sumj: Vec<i32> = vec![-1; MAX_DS];
        for (k, x) in a.iter().enumerate() {
            let ds = digit_sum(*x as i64) as usize;
        }
        println!();
        for (k, x) in a.iter().enumerate() {
            let ds = digit_sum(*x as i64) as usize;

            if sumi[ds] == -1 {
                sumi[ds] = k as i32;
            } else if sumj[ds] == -1 {
                sumj[ds] = k as i32;
            } else {
                let i = sumi[ds] as usize;
                let j = sumj[ds] as usize;
                if a[i] < a[j] && a[i] < a[k] {
                    sumi[ds] = k as i32;
                } else if a[j] < a[k] {
                    sumj[ds] = k as i32;
                }
            }
        }
        let mut max = -1;
        for k in 0..MAX_DS {
            if sumi[k] >= 0 && sumj[k] >= 0 {
                let i = sumi[k] as usize;
                let j = sumj[k] as usize;
                if a[i] + a[j] > max {
                    max = a[i] + a[j];
                }
            }
        }
        max
    }
}
