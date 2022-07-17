impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut a = nums.clone();
        a.sort();
        let mut cnt = 0;
        let mut sum: Vec<i32> = vec![0; 101];
        a.iter().for_each(|x| {
            sum[*x as usize] += 1;
        });
        //println!("{:?}\n{:?}", sum, a);

        let mut ans = 0;
        sum.iter().for_each(|x| {
            cnt += x / 2;
            ans += x % 2;
        });

        println!("{}, {}", cnt, ans);
        vec![cnt, ans]
    }
}
