use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(string1: String, string2: String) -> bool {
        if string1.len() != string2.len() {
            return false;
        }

        let mut counts = HashMap::new();

       

        for c in string1.chars() {
            if let Some(val) = counts.get_mut(&c) {
                *val += 1
            } else {
                counts.insert(c, 1);
            }
        }

        for c in string2.chars() {
            if let Some(val) = counts.get_mut(&c) {
                *val -= 1
            } else {
                return false;
            }
        }

        for val in counts.values() {
            if *val != 0 {
                return false;
            }
        }

        return true;
    }
}
