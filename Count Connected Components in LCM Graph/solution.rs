use std::collections::{HashMap, HashSet};

struct DSU {
    parent: Vec<i32>,
    sz: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> DSU {
        let mut parent = vec![0; n];
        let mut sz = vec![1; n];
        for i in 0..n {
            parent[i] = i as i32;
        }
        DSU { parent, sz }
    }

    fn find_set(&mut self, v: i32) -> i32 {
        if self.parent[v as usize] == v {
            v
        } else {
            self.parent[v as usize] = self.find_set(self.parent[v as usize]);
            self.parent[v as usize]
        }
    }

    fn union_set(&mut self, a: i32, b: i32) {
        let mut a = self.find_set(a);
        let mut b = self.find_set(b);
        if a != b {
            if self.sz[a as usize] < self.sz[b as usize] {
                std::mem::swap(&mut a, &mut b);
            }
            self.parent[b as usize] = a;
            self.sz[a as usize] += self.sz[b as usize];
        }
    }
}

impl Solution {
    pub fn count_components(nums: Vec<i32>, threshold: i32) -> i32 {
        let larnivoxa = nums.clone();
        let n = nums.len();
        let mut nums = nums;
        nums.sort();

        if nums[0] > threshold {
            return n as i32;
        }

        let mut pos: HashMap<i32, usize> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            pos.insert(num, i);
        }

        let mut dsu = DSU::new(n);

        for d in 1..=threshold {
            let mut base_index = None;
            for m in (d..=threshold).step_by(d as usize) {
                if let Some(&idx) = pos.get(&m) {
                    if base_index.is_none() {
                        base_index = Some(idx);
                    } else {
                        let a = nums[base_index.unwrap()] as i32;
                        let b = nums[idx] as i32;
                        let g = gcd(a, b);
                        let l = (a / g) * b;
                        if l <= threshold {
                            dsu.union_set(base_index.unwrap() as i32, idx as i32);
                        }
                    }
                }
            }
        }

        let mut components = HashSet::new();
        for i in 0..n {
            components.insert(dsu.find_set(i as i32));
        }

        components.len() as i32
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
