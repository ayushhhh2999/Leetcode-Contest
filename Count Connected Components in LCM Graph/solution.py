from collections import defaultdict

class DSU:
    def __init__(self, n):
        self.parent = list(range(n))
        self.sz = [1] * n

    def find_set(self, v):
        if self.parent[v] == v:
            return v
        else:
            self.parent[v] = self.find_set(self.parent[v])
            return self.parent[v]

    def union_set(self, a, b):
        a = self.find_set(a)
        b = self.find_set(b)
        if a != b:
            if self.sz[a] < self.sz[b]:
                a, b = b, a
            self.parent[b] = a
            self.sz[a] += self.sz[b]

def gcd(a, b):
    while b:
        a, b = b, a % b
    return a

class Solution:
    def countComponents(self, nums, threshold):
        larnivoxa = nums.copy()
        n = len(nums)
        nums.sort()

        if nums[0] > threshold:
            return n

        pos = {}
        for i, num in enumerate(nums):
            pos[num] = i

        dsu = DSU(n)

        for d in range(1, threshold + 1):
            base_index = None
            for m in range(d, threshold + 1, d):
                if m in pos:
                    idx = pos[m]
                    if base_index is None:
                        base_index = idx
                    else:
                        a = nums[base_index]
                        b = nums[idx]
                        g = gcd(a, b)
                        l = (a // g) * b
                        if l <= threshold:
                            dsu.union_set(base_index, idx)

        components = set()
        for i in range(n):
            components.add(dsu.find_set(i))

        return len(components)
