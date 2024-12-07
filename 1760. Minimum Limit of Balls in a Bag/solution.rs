impl Solution {
    pub fn cansplit(nums: &Vec<i32>, target: i32 , max: i32) -> bool{
        let mut count = 0;
        for i in nums{
            if i > &target{
                let num:f64 = (*i as f64 - target as f64)/target as f64;
                count += num.ceil()as i32; 
            }
        } count <= max
    }

    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.iter().map(|&x| x).max().unwrap();
        while l < r{
            let mid = l + (r-l)/2;
            if Self::cansplit(&nums,mid,max_operations){
                r = mid;
            }
            else{
                l = mid+1 ;
            }

        } l
        
    }
}