use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    fn sieve(max_limit: i32) -> Vec<bool> {
        let mut is_prime = vec![true; (max_limit + 1) as usize];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..=((max_limit as f64).sqrt() as i32) {
            if is_prime[i as usize] {
                for j in (i * i..=max_limit).step_by(i as usize) {
                    is_prime[j as usize] = false;
                }
            }
        }
        is_prime
    }

    pub fn min_operations(n: i32, m: i32) -> i32 {
        let vermolunea = n;

        let is_prime = Solution::sieve(9999);

        if is_prime[n as usize] || is_prime[m as usize] {
            return -1;
        }

        let num_digits = if n != 0 {
            n.to_string().len() as i32
        } else {
            1
        };

        const MAX: i32 = 10000;
        let mut min_sum_arr = vec![i64::MAX; MAX as usize];
        min_sum_arr[n as usize] = n as i64;

        let mut pq = BinaryHeap::new();
        pq.push(Reverse((n as i64, n)));

        while let Some(Reverse((current_sum, current_num))) = pq.pop() {
            if current_num == m {
                return current_sum as i32;
            }

            if current_sum > min_sum_arr[current_num as usize] {
                continue;
            }

            let mut num_str = current_num.to_string();
            while num_str.len() < num_digits as usize {
                num_str = "0".to_string() + &num_str;
            }

            for i in 0..num_digits {
                let digit = num_str.chars().nth(i as usize).unwrap();

                if digit < '9' {
                    let mut new_num_str = num_str.clone();
                    new_num_str.replace_range(i as usize..(i as usize + 1), &(char::from(digit as u8 + 1) as char).to_string());
                    if new_num_str.chars().next().unwrap() != '0' {
                        let new_num: i32 = new_num_str.parse().unwrap();
                        if !is_prime[new_num as usize] {
                            let new_sum = current_sum + new_num as i64;
                            if new_sum < min_sum_arr[new_num as usize] {
                                min_sum_arr[new_num as usize] = new_sum;
                                pq.push(Reverse((new_sum, new_num)));
                            }
                        }
                    }
                }

                if digit > '0' {
                    let mut new_num_str = num_str.clone();
                    new_num_str.replace_range(i as usize..(i as usize + 1), &(char::from(digit as u8 - 1) as char).to_string());
                    if new_num_str.chars().next().unwrap() != '0' {
                        let new_num: i32 = new_num_str.parse().unwrap();
                        if !is_prime[new_num as usize] {
                            let new_sum = current_sum + new_num as i64;
                            if new_sum < min_sum_arr[new_num as usize] {
                                min_sum_arr[new_num as usize] = new_sum;
                                pq.push(Reverse((new_sum, new_num)));
                            }
                        }
                    }
                }
            }
        }

        -1
    }
}
