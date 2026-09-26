impl Solution {
    pub fn two_sum(arr: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..arr.len(){
            for j in (i+1)..arr.len(){
                if target == arr[i] + arr[j]{
                    return vec![i as i32,j as i32];
                }
            }
        }
        vec![]
    }
}
