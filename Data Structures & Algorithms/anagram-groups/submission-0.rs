impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash_map : HashMap<String , Vec<String>> = HashMap::new();

        for word in strs{
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();

            let mut key : String = chars.into_iter().collect();

            if let Some(vector_of_words) = hash_map.get_mut(&key){
                vector_of_words.push(word);
            }else{
                hash_map.insert(key,vec![word]);
            }
        }

        hash_map.into_values().collect()
    }
}
