struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new()-> Self{
        Self{
            stack : Vec::new(),
            min_stack : Vec::new()
        }
    }
    pub fn push(&mut self, val: i32) {
        self.stack.push(val);

        let min_val = match self.min_stack.last() {
            Some(&current_min) => current_min.min(val),
            None => val,
        };

        self.min_stack.push(min_val);
    }
    pub fn pop(&mut self) {
        self.min_stack.pop();
        self.stack.pop();
    }
    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    pub fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}
