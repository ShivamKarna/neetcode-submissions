impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {

        if nums.is_empty(){
            return 0;
        }

        
        nums.sort();
        
        let mut duplicates_free_array = Vec::<i32>::new();

        for num in nums {
            if duplicates_free_array.is_empty() || *duplicates_free_array.last().unwrap() != num{
                duplicates_free_array.push(num);
            }
        }


        let mut longest = 1;
        let mut current_longest = 1;

        for i in 1..duplicates_free_array.len(){
            if duplicates_free_array[i] == duplicates_free_array[i-1] + 1{
                current_longest += 1; 
                longest = longest.max(current_longest);
            }
            else{
                current_longest = 1;
            }
        }

        longest

    }
}
