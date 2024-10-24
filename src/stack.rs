pub struct Stack(Vec<i32>);
use crate::error::ErrorKind;

impl Stack {
    pub fn new() -> Self {
        Stack(Vec::new())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    fn check_underflow(&self, n: usize) -> Result<String, ErrorKind> {
        if n > self.len() {
            return Err(ErrorKind::StackUnderFlowError);
        }
        Ok("".to_string())
    }
    pub fn sum(&mut self) -> Result<String, ErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.0.push(b + a);
        Ok("".to_string())
    }

    pub fn minus(&mut self) -> Result<String, ErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.0.push(b - a);
        Ok("".to_string())
    }

    pub fn div(&mut self) -> Result<String, ErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        if a == 0 {
            return Err(ErrorKind::DivisionByZero);
        }
        self.0.push(b / a);
        Ok("".to_string())
    }

    pub fn mul(&mut self) -> Result<String, ErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.0.push(b * a);
        Ok("".to_string())
    }

    pub fn f_mod(&mut self) -> Result<String, ErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.0.push(b % a);
        Ok("".to_string())
    }

    pub fn equal(&mut self) -> Result<String, ErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        if a == b {
            self.0.push(-1); // true
        } else {
            self.0.push(0); // false
        }
        Ok("".to_string())
    }

    pub fn rot(&mut self) -> Result<String, ErrorKind> {
        self.check_underflow(3)?;
        let c = self.0.remove(self.len() - 3);
        self.0.push(c);
        Ok("".to_string())
    }
    pub fn pop(&mut self) -> Result<i32, ErrorKind> {
        self.0.pop().ok_or(ErrorKind::StackUnderFlowError)
    }
    pub fn drop(&mut self) -> Result<String, ErrorKind> {
        self.pop()?;
        Ok("".to_string())
    }

    pub fn dup(&mut self) -> Result<String, ErrorKind> {
        self.check_underflow(1)?;
        self.0.push(self.0.last().unwrap().clone());
        Ok("".to_string())
    }

    pub fn nip(&mut self) -> Result<String, ErrorKind> {
        self.check_underflow(2)?;
        self.0.remove(self.len() - 2);
        Ok("".to_string())
    }
    pub fn over(&mut self) -> Result<String, ErrorKind> {
        self.check_underflow(2)?;
        self.0.push(self.0.get(self.len() - 2).unwrap().clone());
        Ok("".to_string())
    }
    pub fn swap(&mut self) -> Result<String, ErrorKind> {
        self.check_underflow(2)?;
        let stack_len = self.len();
        self.0.swap(stack_len - 1, stack_len - 2);
        Ok("".to_string())
    }

    pub fn push(&mut self, value: i32) -> Result<String, ErrorKind> {
        self.0.push(value);
        Ok("".to_string())
    }

    pub fn dot_stack(&mut self) -> Result<String, ErrorKind> {
        let str = format!("{:?}", self.0);
        Ok(str)
    }

    pub fn emit(&mut self) -> Result<String, ErrorKind> {
        let str = self.pop()? as u8 as char;
        Ok(str.to_string())
    }

    pub fn dot(&mut self) -> Result<String, ErrorKind> {
        let str = self.pop()?;
        Ok(str.to_string())
    }
}
