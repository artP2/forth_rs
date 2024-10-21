pub struct Stack(Vec<i32>);
impl Stack {
    pub fn new() -> Self {
        Stack(Vec::new())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn sum(&mut self) -> () {
        let a = self.0.pop().unwrap();
        let b = self.0.pop().unwrap();
        self.0.push(b + a);
    }

    pub fn minus(&mut self) -> () {
        let a = self.0.pop().unwrap();
        let b = self.0.pop().unwrap();
        self.0.push(b - a);
    }

    pub fn div(&mut self) -> () {
        let a = self.0.pop().unwrap();
        let b = self.0.pop().unwrap();
        self.0.push(b / a);
    }

    pub fn mul(&mut self) -> () {
        let a = self.0.pop().unwrap();
        let b = self.0.pop().unwrap();
        self.0.push(b * a);
    }

    pub fn f_mod(&mut self) -> () {
        let a = self.0.pop().unwrap();
        let b = self.0.pop().unwrap();
        self.0.push(b % a);
    }

    pub fn equal(&mut self) -> () {
        let a = self.0.pop().unwrap();
        let b = self.0.pop().unwrap();
        if a == b {
            self.0.push(-1); // true
        } else {
            self.0.push(0); // false
        }
    }

    pub fn rot(&mut self) -> () {
        let c = self.0.remove(self.len() - 3);
        self.0.push(c);
    }
    pub fn pop(&mut self) -> i32 {
        self.0.pop().unwrap()
    }

    pub fn dup(&mut self) -> () {
        self.0.push(self.0.last().unwrap().clone());
    }

    pub fn nip(&mut self) -> () {
        self.0.remove(self.len() - 2);
    }
    pub fn over(&mut self) -> () {
        self.0.push(self.0.get(self.len() - 2).unwrap().clone());
    }
    pub fn swap(&mut self) -> () {
        let stack_len = self.len();
        self.0.swap(stack_len - 1, stack_len - 2);
    }

    pub fn push(&mut self, value: i32) -> () {
        self.0.push(value);
    }

    pub fn see(&mut self) -> &Vec<i32> {
        &self.0
    }
}
