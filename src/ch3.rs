struct Stack {
    arr: Vec<u8>,
    min: Vec<u8>,
}

impl Stack {
    pub fn push(&mut self, elem: u8) {
        self.arr.push(elem);
        if self.min.is_empty() || *self.min.last().unwrap() >= elem {
            self.min.push(elem);
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        let result = self.arr.pop();
        if result.is_some() && result.as_ref().eq(&self.min.last()) {
            self.min.pop();
        }
        result
    }

    pub fn min(&self) -> Option<&u8> {
        self.min.last()
    }
}

struct SetOfStacks {
    stacks: Vec<Vec<u8>>,
    threshold: usize,
}

impl SetOfStacks {
    pub fn new(threshold: usize) -> Self {
        Self {
            stacks: Vec::new(),
            threshold,
        }
    }

    pub fn push(&mut self, elem: u8) {
        if let Some(stack) = self.stacks.last_mut() {
            if stack.len() < self.threshold {
                stack.push(elem);
            } else {
                self.stacks.push(vec![elem]);
            }
        } else {
            self.stacks.push(vec![elem])
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if let Some(mut stack) = self.stacks.pop() {
            let elem = stack.pop();
            if !stack.is_empty() {
                self.stacks.push(stack);
            }
            elem
        } else {
            None
        }
    }
}

struct MyQueue {
    stack: Vec<u8>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, elem: u8) {
        self.stack.push(elem)
    }

    pub fn pop(&mut self) -> Option<u8> {
        let mut temp = Vec::new();
        while let Some(elem) = self.stack.pop() {
            temp.push(elem);
        }

        let result = temp.pop();
        while let Some(elem) = temp.pop() {
            self.stack.push(elem)
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ref_cmp() {
        let s1 = String::from("hello_world");
        let s1_ref = Some(&s1);
        //println!("s1 address: {:p}", s1_ref.unwrap());
        let s2 = String::from("hello_world");
        let s2_ref = Some(&s2);
        //println!("s2 address: {:p}", s2_ref.unwrap());
        assert!(s2_ref.eq(&s1_ref));
    }
}
