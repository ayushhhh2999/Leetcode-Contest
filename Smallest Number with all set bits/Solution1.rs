impl Solution {
    pub fn smallest_number(mut n: i32) -> i32 {
        fn next_power_of_two(mut n: i32) -> i32 {
            if n <= 0 {
                panic!("Input must be a positive integer");
            }
            if (n & (n - 1)) == 0 {
            
                n <<= 1; 
            } else {
                n -= 1;
                n |= n >> 1;
                n |= n >> 2;
                n |= n >> 4;
                n |= n >> 8;
                n |= n >> 16;
                n += 1;
            }
            n-1
        }
        
        
        next_power_of_two(n)
    }
}
