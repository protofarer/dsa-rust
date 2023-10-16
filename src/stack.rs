#![allow(unused)]
// * linked list impl
// - PRO: 1. easy to add/rm nodes 2.mem allocated as needed
// - CON: 1. pointer overhead 2. slow access due pointer traversal 3. probably complicated

// * fixed-size impl
// - PRO: 1. fast access via index O(1) 2. no pointers 3. simplest impl 4. predictable memory usage
// - CON: 1. resize operation  2. unused capacity is wasted memory

// * dynamic-size impl
// - PRO: 1. flexible
// - CON: 1. slower than fixed-size 2. larger memory footprint

// NB: if size is part of type (size known at compile time), stronger type safety and clarity, but all stack's of this type are of course only of this size
use std::error::Error;

struct Stack<T, const N: usize> {
    data: [Option<T>; N],
    size: usize,
}

#[derive(PartialEq, Debug)]
enum StackError {
    Full,
    Empty,
}

impl<T: Clone + Copy + Default, const N: usize> Stack<T, N> {
    fn new() -> Self {
        Stack {
            data: [None; N],
            size: 0,
        }
    }
    // put item on top of stack
    pub fn push(&mut self, item: T) -> Result<(), StackError> {
        if self.size >= N {
            eprintln!("Stack is full!");
            return Err(StackError::Full);
        }
        self.data[self.size] = Some(item);
        self.size += 1;
        Ok(())
    }
    // remove item from top of stack
    pub fn pop(&mut self) -> Result<Option<T>, StackError> {
        if self.size == 0 {
            return Err(StackError::Empty);
        }

        self.size -= 1;
        Ok(self.data[self.size].take())
    }
    // view top of stack
    pub fn peek(&self) -> Option<T> {
        if self.size == 0 {
            eprintln!("Nothing to peek");
            return None;
        }
        self.data[self.size - 1]
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut stack: Stack<i32, 2> = Stack::new();

        assert_eq!(stack.size(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.is_empty(), true);
        assert!(stack.pop().is_err());

        let _ = stack.push(0);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.peek(), Some(0));
        assert_eq!(stack.is_empty(), false);
        assert!(stack.pop().is_ok());
        assert!(stack.pop().is_err());

        let _ = stack.push(1);
        let _ = stack.push(-1);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.peek(), Some(-1));
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.pop(), Ok(Some(-1)));
        assert_eq!(stack.size(), 1);
        assert!(stack.pop().is_ok());
        assert_eq!(stack.pop(), Err(StackError::Empty));

        let _ = stack.push(1000);
        let _ = stack.push(314159265);
        assert_eq!(stack.push(10), Err(StackError::Full));
    }
}
