import math
class Solution:
    def cansplit(self, nums: List[int], target:int,max :int):
            count = 0
            for i in nums:
                if i > target:
                    count += math.ceil((i-target)/target)
            return count <= max
    def minimumSize(self, nums: List[int], maxOperations: int) -> int:
        l = 1
        r = max(nums)
        res = 0
        while l < r:
            mid = l + (r-l)//2
            if self.cansplit(nums,mid,maxOperations):
                r = mid
                res = mid
            else:
                l = mid + 1
        return l                  
        