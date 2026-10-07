impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        
        let n = temperatures.len();
        let mut stack = Vec::<usize>::new();
        let mut result = vec![0;n];

        for i in 0..n{
            while let Some(&prev) = stack.last(){
                if temperatures[i] > temperatures[prev] {
                    stack.pop();
                    result[prev] = (i-prev) as i32;
                }else{
                    break;
                }
            }
            stack.push(i);
        }
        result
    
    }
}
