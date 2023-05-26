/// Stack implementation
/// Uses LIFO (last-in first-out) ordering.
/// The most recently added is the first item to be removed.

pub struct Stack<T> {
    data: Vec<T>,
    pointer_to_top: i32,
    size: u32,
}

#[allow(dead_code)]
impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: vec![], pointer_to_top: -1, size: 0 }
    }

    /// Returns true if and only if the stack is empty
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Add an item on top of the stack
    /// When stack is full, then it is said to be
    /// an Overflow condition.
    fn push(&mut self, item: T) {
        self.pointer_to_top += 1;
        self.data.insert(self.pointer_to_top as usize, item);
        self.size += 1;
    }

    /// Return the top of the stack, but doesn't remove it
    /// from the stack
    fn peek(&self) -> Option<&T> {
        if self.pointer_to_top.is_negative() {
            return None;
        } else {
            self.data.get(self.pointer_to_top as usize)
        }
    }

    /// Remove the top item from the stack
    /// Removed in reverse order as pushed.
    /// If the stack is empty, it is an Underflow condition.
    fn pop(&mut self) -> Option<T> {
        if self.pointer_to_top.is_negative() {
            return None;
        }
        self.size -= 1;

        let result = Some(self.data.remove(self.pointer_to_top as usize));
        self.pointer_to_top -= 1;
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn when_stack_is_created_it_is_empty() {
        let stack = Stack::<&str>::new();

        assert!(stack.is_empty());
        assert_eq!(stack.size, 0);
    }

    #[test]
    fn test_adding_items_to_empty_stack() {
        let values = vec!["A", "B", "C", "D", "E"];
        let mut stack = Stack::<&str>::new();
        for item in &values {
            stack.push(item);
        }

        assert_eq!(stack.data, values);
        assert_eq!(stack.size, 5);
    }

    #[test]
    fn test_peek_when_empty_stack() {
        let empty_stack = Stack::<i32>::new();

        let result = empty_stack.peek();

        assert!(result.is_none());
    }

    #[test]
    fn test_peek_when_one_item_in_stack() {
        let mut stack = Stack::new();
        stack.push("A");

        let result = stack.peek();

        assert!(result.is_some());
        assert_eq!(result, Some(&"A"));
    }

    #[test]
    fn test_peek_when_multiple_items_in_stack() {
        let mut stack = Stack::new();
        stack.push("A");
        stack.push("B");
        stack.push("C");
        stack.push("D");

        let result = stack.peek();

        assert!(result.is_some());
        assert_eq!(result, Some(&"D"));
    }

    #[test]
    fn test_pop_when_empty_stack() {
        let mut empty_stack = Stack::<&str>::new();

        let result = empty_stack.pop();

        assert!(result.is_none());
        assert_eq!(empty_stack.size, 0);
    }

    #[test]
    fn test_pop_when_one_item_in_stack() {
        let mut stack = Stack::new();
        stack.push("A");

        let result = stack.pop();

        assert!(result.is_some());
        assert!(stack.is_empty());
        assert_eq!(result, Some("A"));
        assert_eq!(stack.size, 0);
    }

    #[test]
    fn test_pop_when_multiple_items_in_stack() {
        let mut stack = Stack::new();
        stack.push("A");
        stack.push("B");
        stack.push("C");
        stack.push("D");
        stack.push("E");

        let result_1 = stack.pop();
        let result_2 = stack.pop();

        assert!(result_1.is_some());
        assert!(result_2.is_some());
        assert_eq!(result_1, Some("E"));
        assert_eq!(result_2, Some("D"));
        assert_eq!(stack.size, 3);
    }

    #[test]
    fn test_pop_when_all_items_removed() {
        let values = vec!["A", "B", "C", "D", "E"];
        let mut stack = Stack::new();
        for &item in &values {
            stack.push(item);
        }

        for value in values.into_iter().rev() {
            let result = stack.pop();
            assert_eq!(result, Some(value), "Items from stack removed in reverse order");
        }

        assert!(stack.is_empty());
        assert_eq!(stack.size, 0);
    }
}
