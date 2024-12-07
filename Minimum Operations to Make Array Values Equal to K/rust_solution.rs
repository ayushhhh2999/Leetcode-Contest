impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        for &num in &nums {
            if num < k {
                return -1;
            }
        }

        let mut nums = nums;
        nums.sort_by(|a, b| b.cmp(a));

        let mut count = 0;

        while nums[0] > k {
            let max_val = nums[0];
            count += 1;

            let mut next_val = k;
            for &num in &nums {
                if num < max_val {
                    next_val = num;
                    break;
                }
            }

            for num in nums.iter_mut() {
                if *num == max_val {
                    *num = next_val;
                }
            }
        }

        count
    }
}
