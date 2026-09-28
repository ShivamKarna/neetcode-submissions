impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts : HashMap<i32,i32> = HashMap::new();

        for number in nums{
            *counts.entry(number).or_insert(0) += 1;
        }

        let mut freqs : Vec<(i32,i32)> = counts.into_iter().collect();
        freqs
            .sort_by(|(num_a,a_freq),(num_b,b_freq)| b_freq.cmp(&a_freq));

        freqs
            .into_iter()
            .take(k as usize)
            .map(|(number,_freq)| number)
            .collect()
    }
}
