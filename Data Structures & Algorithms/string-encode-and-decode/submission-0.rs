impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut res = String::new();

        for s in strs {
            res.push_str(&s.len().to_string());
            res.push_str("#");
            res.push_str(&s);
        }

        println!("{}", res);
        res
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut res_vector = Vec::new();

        let mut s_bytes = s.as_bytes();
        let mut i = 0;

        while i < s_bytes.len() {
            let mut j = i;

            while s_bytes[j] != b'#' {
                j += 1;
            }

            let len: usize = s[i..j].parse().unwrap();

            let start = j + 1;
            let end = start + len;

            res_vector.push(s[start..end].to_string());
            i = end;
        }
        res_vector
    }
}
