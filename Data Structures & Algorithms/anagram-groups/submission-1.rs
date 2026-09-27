impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash_map : HashMap<[i32; 26] , Vec<String>> = HashMap::new();

        for word in strs{
            let mut count = [0;26];

            for byte in word.bytes(){
                let index = (byte - b'a') as usize;
                count[index] +=1;
            }

            if let Some(vector_of_words) = hash_map.get_mut(&count){
                vector_of_words.push(word);
            }else{
                hash_map.insert(count,vec![word]);
            }
        }

        hash_map.into_values().collect()
    }
}
