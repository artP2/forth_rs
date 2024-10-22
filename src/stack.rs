pub struct Stack(Vec<i32>);
use crate::error::ErrorKind;

impl Stack {
    pub fn new() -> Self {
        Stack(Vec::new())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    fn check_underflow(&self, n: usize) -> Result<(), ErrorKind> {
        if n > self.len() {
            return Err(ErrorKind::StackUnderFlowError);
        }
        Ok(())
    }
    pub fn sum(&mut self) -> Result<(), ErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.0.push(b + a);
        Ok(())
    }

    pub fn minus(&mut self) -> Result<(), ErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.0.push(b - a);
        Ok(())
    }

    pub fn div(&mut self) -> Result<(), ErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.0.push(b / a);
        Ok(())
    }

    pub fn mul(&mut self) -> Result<(), ErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.0.push(b * a);
        Ok(())
    }

    pub fn f_mod(&mut self) -> Result<(), ErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.0.push(b % a);
        Ok(())
    }

    pub fn equal(&mut self) -> Result<(), ErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        if a == b {
            self.0.push(-1); // true
        } else {
            self.0.push(0); // false
        }
        Ok(())
    }

    pub fn rot(&mut self) -> Result<(), ErrorKind> {
        self.check_underflow(3)?;
        let c = self.0.remove(self.len() - 3);
        self.0.push(c);
        Ok(())
    }
    pub fn pop(&mut self) -> Result<i32, ErrorKind> {
        self.0.pop().ok_or(ErrorKind::StackUnderFlowError)
    }

    pub fn dup(&mut self) -> Result<(), ErrorKind> {
        self.check_underflow(1)?;
        self.0.push(self.0.last().unwrap().clone());
        Ok(())
    }

    pub fn nip(&mut self) -> Result<(), ErrorKind> {
        self.check_underflow(2)?;
        self.0.remove(self.len() - 2);
        Ok(())
    }
    pub fn over(&mut self) -> Result<(), ErrorKind> {
        self.check_underflow(2)?;
        self.0.push(self.0.get(self.len() - 2).unwrap().clone());
        Ok(())
    }
    pub fn swap(&mut self) -> Result<(), ErrorKind> {
        self.check_underflow(2)?;
        let stack_len = self.len();
        self.0.swap(stack_len - 1, stack_len - 2);
        Ok(())
    }

    pub fn push(&mut self, value: i32) -> Result<(), ErrorKind> {
        self.0.push(value);
        Ok(())
    }

    pub fn see(&mut self) -> &Vec<i32> {
        &self.0
    }
}
