#[allow(dead_code)]

pub struct Stack {
    elements: Vec<i32>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, value: i32) {
        self.elements.push(value);
    }

    pub fn pop(&mut self) -> Result<i32, String>{
        if self.is_empty() {
            Err("Stack empty".to_string())
        } else {
            Ok(self.elements.pop().unwrap())
        }
    }

    pub fn peek(&self) -> Result<&i32, String> {
        if self.is_empty() {
            Err("Stack empty".to_string())
        } else {
            Ok(self.elements.last().unwrap())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}