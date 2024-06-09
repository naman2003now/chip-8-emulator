pub struct Stack {
    data: [u16; 64],
    top: usize,
    size: usize,
}

impl Stack {
    pub fn new() -> Self {
        let data = [0; 64];
        let top = 0;
        let size = 64;

        Self { data, top, size }
    }

    pub fn push(&mut self, value: u16) {
        if self.top == self.size {
            panic!("Stack overflow");
        }
        self.data[self.top] = value;
        self.top += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.top == 0 {
            panic!("Stack underflow");
        }
        self.top -= 1;
        self.data[self.top]
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn size() {
        let stack = Stack::new();
        assert_eq!(stack.size, stack.data.len());
    }

    #[test]
    fn push() {
        let mut stack = Stack::new();
        stack.push(0x200);
        assert_eq!(stack.data[0], 0x200);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::new();
        stack.push(0x200);
        assert_eq!(stack.pop(), 0x200);
    }

    #[test]
    fn top() {
        let stack = Stack::new();
        assert_eq!(stack.top, 0);
    }

    #[test]
    #[should_panic(expected = "Stack overflow")]
    fn stack_overflow() {
        let mut stack = Stack::new();
        for _ in 0..65 {
            stack.push(0x200);
        }
    }

    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn stack_underflow() {
        let mut stack = Stack::new();
        stack.pop();
    }
}
