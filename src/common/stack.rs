use std::collections::{linked_list, LinkedList};

/// A First-In-First-Out stack for generic items.
/// 1. push
/// 2. pop
/// 3. peek
/// 4. is_empty
/// 5. iter

pub struct Stack<T> {
    list: LinkedList<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.list.push_back(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.list.pop_back()
    }

    fn peek(&self) -> Option<&T> {
        self.list.back()
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    fn iter(&self) -> linked_list::Iter<T> {
        self.list.iter()
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn test_new() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_push() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(123);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.peek(), Some(&3));
    }

    #[test]
    fn test_pop() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
    }

    #[test]
    fn test_iter() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        let mut it = stack.iter();
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), None);
    }
}
