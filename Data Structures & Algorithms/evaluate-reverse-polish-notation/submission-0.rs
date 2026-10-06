impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::<i32>::new();

        for s in tokens {
            if s == "+" || s == "-" || s == "*" || s == "/" {
                let first_num = stack.pop().unwrap();
                let second_num = stack.pop().unwrap();

                let result = match s.as_str() {
                    "+" => second_num + first_num,
                    "-" => second_num - first_num,
                    "*" => second_num * first_num,
                    "/" => second_num / first_num,
                    _ => unreachable!("Invalid operator : {}", s),
                };

                stack.push(result);
            } else {
                stack.push(s.parse::<i32>().unwrap());
            }
        }

        stack.pop().unwrap()
    }
}
