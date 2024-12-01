class Solution:
     def smallest_number(n: int) -> int:
        if n <= 0:
            raise ValueError("Input must be a positive integer")
        
        def next_power_of_two(n: int) -> int:
            if (n & (n - 1)) == 0:
                n <<= 1  # Double the value if already a power of two
            else:
                n -= 1
                n |= n >> 1
                n |= n >> 2
                n |= n >> 4
                n |= n >> 8
                n |= n >> 16
                n += 1
            return n - 1
        
        return next_power_of_two(n)