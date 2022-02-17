#[derive(Debug, Clone, Copy)]
pub enum Error {
    Overflow,
    Underflow,
}

/// A simple generic stack machine
#[derive(Debug, Clone)]
pub struct Stack<T> {
    pub items: [T; 100],
    pub top: usize,
    pub capacity: usize,
}

impl<T: std::marker::Copy> Stack<T> {
    pub fn new(filler: T) -> Self {
        Stack {
            items: [filler; 100],
            top: 0,
            capacity: 100,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), Error> {
        if self.top >= self.capacity {
            return Err(Error::Overflow);
        }
        self.items[self.top] = item;
        self.top += 1;

        Ok(())
    }
    pub fn pop(&mut self) -> Result<T, Error> {
        if self.top == 0 {
            return Err(Error::Underflow);
        }
        self.top -= 1;

        Ok(self.items[self.top])
    }

    pub fn peek(&self) -> Result<T, Error> {
        if self.top == 0 {
            return Err(Error::Underflow);
        }
        Ok(self.items[self.top - 1])
    }

    pub fn dup(&mut self) -> Result<(), Error> {
        self.push(self.peek()?)?;
        Ok(())
    }

    pub fn swap(&mut self) -> Result<(), Error> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(a)?;
        self.push(b)?;
        Ok(())
    }

    pub fn over(&mut self) -> Result<(), Error> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(b)?;
        self.push(a)?;
        self.push(b)?;
        Ok(())
    }

    pub fn rot(&mut self) -> Result<(), Error> {
        let a = self.pop()?;
        let b = self.pop()?;
        let c = self.pop()?;
        self.push(a)?;
        self.push(c)?;
        self.push(b)?;
        Ok(())
    }
}

impl Stack<i64> {
    pub fn add(&mut self) -> Result<(), Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(a + b)?;
        Ok(())
    }
    pub fn sub(&mut self) -> Result<(), Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(b - a)?;
        Ok(())
    }
    pub fn mul(&mut self) -> Result<(), Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(a * b)?;
        Ok(())
    }
    pub fn div(&mut self) -> Result<(), Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(b / a)?;
        Ok(())
    }

    pub fn equals(&mut self) -> Result<(), Error> {
        let a = self.pop()?;
        let b = self.pop()?;
        if a == b {
            self.push(-1)?;
        } else {
            self.push(0)?;
        }
        Ok(())
    }
    pub fn gt(&mut self) -> Result<(), Error> {
        let a = self.pop()?;
        let b = self.pop()?;
        if b > a {
            self.push(-1)?;
        } else {
            self.push(0)?;
        }
        Ok(())
    }
    pub fn lt(&mut self) -> Result<(), Error> {
        let a = self.pop()?;
        let b = self.pop()?;
        if b < a {
            self.push(-1)?;
        } else {
            self.push(0)?;
        }
        Ok(())
    }
}
