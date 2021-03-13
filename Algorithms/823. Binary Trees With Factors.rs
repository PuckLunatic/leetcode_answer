use std::collections::HashMap;

impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let mut a: Vec<i32> = arr;
        let n = a.len();
        let mut dp: Vec<i64> = vec![1; n];
        let modulo = 1_000_000_007;
        let mut hm: HashMap<i32, usize> = HashMap::new();
        a.sort_unstable();
        let mut res = 0;
        for i in 0..n {
            hm.insert(a[i], i);
            for j in 0..i {
                if a[i] % a[j] == 0 {
                    if let Some(&k) = hm.get(&(a[i] / a[j])) {
                        dp[i] += dp[j] * dp[k];
                    }
                }
            }
            res = (res + dp[i]) % modulo;
        }
        res as i32
    }
}
