use std::collections::HashMap;

impl Solution {
    pub fn get_largest_outlier(arr: Vec<i32>) -> i32 {
        let total: i32 = arr.iter().sum();
        let mut num_count: HashMap<i32, i32> = HashMap::new();
        
        for &num in &arr {
            *num_count.entry(num).or_insert(0) += 1;
        }
        
        let mut result = i32::MIN;
        
        for &num in &arr {
            if (total - num) % 2 == 0 {
                let target = (total - num) / 2;
                
               
                if let Some(count) = num_count.get_mut(&num) {
                    *count -= 1;
                    if *count == 0 {
                        num_count.remove(&num);
                    }
                }
                
               
                if num_count.get(&target).cloned().unwrap_or(0) > 0 {
                    result = result.max(num);
                }
                
                
                *num_count.entry(num).or_insert(0) += 1;
            }
        }
        
        if result == i32::MIN {
            -1 
        } else {
            result
        }
    }
}