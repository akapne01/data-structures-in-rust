/// Queue implementation
/// Ordering: FIFI (First-in First-out)
/// As in line or queue at the ticket stand, items are
/// removed from the data structure un the same order as
/// they are added.

const DEFAULT_CAPACITY_QUEUE: usize = 256;

pub struct Queue<T> {
    data: Vec<Option<T>>,
    capacity: usize,
    current_size: usize,
}

#[allow(dead_code)]
impl<T: Copy> Queue<T> {
    fn new() -> Self {
        Queue {
            data: Vec::with_capacity(DEFAULT_CAPACITY_QUEUE),
            capacity: DEFAULT_CAPACITY_QUEUE,
            current_size: 0,
        }
    }

    /// Add an item to the end of the queue
    /// Underlying vector increases capacity automatically
    /// once it becomes full. Increasing capacity variable
    /// to reflect this change.
    fn add(&mut self, item: T) {
        if self.is_full() {
            self.capacity += DEFAULT_CAPACITY_QUEUE;
        }
        self.data.insert(self.current_size as usize, Some(item));
        self.current_size += 1;
    }

    /// Remove the first item in the queue
    fn remove(&mut self) -> Option<T> {
        const FIRST_ITEM_INDEX: usize = 0;
        if self.is_empty() {
            return None;
        }
        let result = self.data.remove(FIRST_ITEM_INDEX);
        self.current_size -= 1;
        result
    }

    /// Return the top of the queue
    fn peek(&self) -> Option<T> {
        if let Some(last_element) = self.data.last().cloned() {
            return last_element;
        }
        None
    }

    /// Return true if and only if the queue is empty
    fn is_empty(&self) -> bool {
        self.current_size == 0
    }

    fn is_full(&self) -> bool {
        self.current_size == self.capacity
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn when_queue_is_created_it_is_empty() {
        let empty_queue = Queue::<i32>::new();

        assert!(empty_queue.is_empty());
        assert_eq!(empty_queue.current_size, 0);
    }

    #[test]
    fn test_add_item_to_queue() {
        let mut queue = Queue::<i32>::new();

        queue.add(13);

        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.current_size, 1);
        assert!(queue.data.contains(&Some(13)));
    }

    #[test]
    fn test_add_multiple_items_to_queue() {
        let mut queue = Queue::<i32>::new();

        for num in 0..14 {
            queue.add(num);
        }

        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.current_size, 14);
        for num in 0..14 {
            assert!(queue.data.contains(&Some(num)));
        }
    }

    #[test]
    fn test_queue_is_full() {
        let mut queue = Queue::<i32>::new();

        for num in 0..DEFAULT_CAPACITY_QUEUE as i32 {
            queue.add(num);
        }

        assert!(queue.is_full());
        assert_eq!(queue.current_size, DEFAULT_CAPACITY_QUEUE);
        for num in 0..DEFAULT_CAPACITY_QUEUE as i32 {
            assert!(queue.data.contains(&Some(num)));
        }
    }

    #[test]
    fn test_adding_more_items_than_capacity_increases_underlying_queue() {
        let mut queue = Queue::<i32>::new();

        for num in 0..(DEFAULT_CAPACITY_QUEUE + 1) as i32 {
            queue.add(num);
        }

        assert_eq!(queue.is_full(), false, "After reaching full capacity, vector doubles in size.");
        assert_eq!(queue.current_size, DEFAULT_CAPACITY_QUEUE + 1);
        assert_eq!(queue.data.len(), DEFAULT_CAPACITY_QUEUE + 1);
        assert_eq!(queue.capacity, DEFAULT_CAPACITY_QUEUE * 2);
        assert_eq!(queue.data.capacity(), DEFAULT_CAPACITY_QUEUE * 2);

        for num in 0..(DEFAULT_CAPACITY_QUEUE + 1) as i32 {
            assert!(
                queue.data.contains(&Some(num)),
                "Assert that all elements are present in the resized queue"
            );
        }
    }

    #[test]
    fn test_removing_items_from_an_empty_queue() {
        let mut empty_queue = Queue::<i32>::new();

        let result = empty_queue.remove();

        assert!(empty_queue.is_empty());
        assert!(result.is_none());
    }

    #[test]
    fn test_removing_items_from_queue() {
        let mut queue = Queue::<i32>::new();

        for item in 0..14 {
            queue.add(item);
        }

        let result = queue.remove();

        assert!(result.is_some());
        assert_eq!(result, Some(0));
        assert_eq!(queue.current_size, 13);
    }

    #[test]
    fn test_removing_all_items_in_queue_it_is_empty() {
        let mut queue = Queue::<i32>::new();
        for item in 0..14 {
            queue.add(item);
        }

        for item in 0..14 {
            let result = queue.remove();
            assert!(result.is_some());
            assert_eq!(result, Some(item));
        }

        assert!(queue.is_empty());
        assert_eq!(queue.current_size, 0);
    }

    #[test]
    fn test_peek_when_empty_queue() {
        let empty_queue = Queue::<i32>::new();

        let result = empty_queue.peek();

        assert!(empty_queue.is_empty());
        assert!(result.is_none());
    }

    #[test]
    fn test_peek_when_queue_has_values() {
        let mut queue = Queue::<i32>::new();
        for item in 0..7 {
            queue.add(item);
        }

        let result = queue.peek();

        assert!(result.is_some());
        assert_eq!(result, Some(6));
        let actual_last_element = queue.data.last().cloned().unwrap();
        assert_eq!(actual_last_element, Some(6));
    }
}
