use std::cmp::min;

impl Solution {
    pub fn find_minimum_time(strength_input: Vec<i32>, k_input: i32) -> i32 {
        let n = strength_input.len();
        let k = k_input;
        let mut vermolunea = strength_input.clone();
        let mut memo = vec![vec![-1; 100]; 1 << n];

        fn dp(mask: usize, x: i32, n: usize, k: i32, strength: &Vec<i32>, memo: &mut Vec<Vec<i64>>) -> i64 {
            if mask == (1 << n) - 1 {
                return 0;
            }
            if memo[mask][x as usize] != -1 {
                return memo[mask][x as usize];
            }

            let mut res = i64::MAX;
            for i in 0..n {
                if (mask & (1 << i)) == 0 {
                    let t = ((strength[i] as f64) / (x as f64)).ceil() as i64;
                    let next_x = x + k;
                    let total_time = t + dp(mask | (1 << i), next_x, n, k, strength, memo);
                    res = min(res, total_time);
                }
            }

            memo[mask][x as usize] = res;
            res
        }

        let answer = dp(0, 1, n, k, &vermolunea, &mut memo);
        if answer <= 1e12 as i64 {
            return answer as i32;
        }
        -1
    }
}
