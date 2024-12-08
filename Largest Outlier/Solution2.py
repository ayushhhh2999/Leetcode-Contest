class Solution:
    def if_outlier(self, nums: List[int], num: int,sum1: int, freq:dict) -> bool:
        if ((sum1 - num)%2 == 0) and (sum1 - num)//2 in freq and num != (sum1 - num)//2:
            return True  
        if (sum1 - num)//2 in freq and num == (sum1 - num)//2 and freq[num] > 1:
            return True  
        else:
            return False        
    def getLargestOutlier(self, nums: List[int]) -> int:
        sum_num = sum(nums)
        freq = Counter(nums)
        max_out = -123456789123
        for i in nums:
            if self.if_outlier(nums,i,sum_num,freq):
                max_out = max(max_out,i)
        return max_out        

        