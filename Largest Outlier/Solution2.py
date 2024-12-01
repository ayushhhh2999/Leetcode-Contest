class Solution:
    def getLargestOutlier(self, arr: List[int]) -> int:
        total = sum(arr)
        num_count = Counter(arr) 
        
        result = float('-inf')
        
        for num in arr:
            if (total - num) % 2 == 0:
                target = (total - num) // 2
                num_count[num] -= 1  
                if num_count[num] == 0:
                    del num_count[num]
                
                if num_count.get(target, 0) > 0:  
                    result = max(result, num)
                
                
                num_count[num] = num_count.get(num, 0) + 1
        
        return result if result != float('-inf') else -1