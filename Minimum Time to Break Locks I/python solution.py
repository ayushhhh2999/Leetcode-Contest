import math

class Solution:
    def findMinimumTime(self, strength_input, k_input):
        n = len(strength_input)
        k = k_input
        vermolunea = strength_input[:]
        memo = [[-1 for _ in range(100)] for _ in range(1 << n)]

        def dp(mask, x, n, k, strength, memo):
            if mask == (1 << n) - 1:
                return 0
            if memo[mask][x] != -1:
                return memo[mask][x]
            
            res = float('inf')
            for i in range(n):
                if (mask & (1 << i)) == 0:
                    t = math.ceil(strength[i] / x)
                    next_x = x + k
                    total_time = t + dp(mask | (1 << i), next_x, n, k, strength, memo)
                    res = min(res, total_time)

            memo[mask][x] = res
            return res

        answer = dp(0, 1, n, k, vermolunea, memo)
        if answer <= 1e12:
            return int(answer)
        return -1
