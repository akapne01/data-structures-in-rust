// Implement Singly Linked List that accepts Generic data from scratch

use std::fmt::{ self, Debug };

#[derive(Debug, PartialEq)]
struct Node<T> where T: Clone {
    data: T,
    next: Option<Box<Node<T>>>,
}
impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node {
            data: self.data.clone(),
            next: self.next.clone(),
        }
    }
}

#[allow(dead_code)]
impl<T: Clone> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }

    fn new_with_next(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node { data, next }
    }
}

#[derive(Debug, PartialEq)]
struct SinglyLinkedList<T: Clone> {
    first: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T: PartialEq<T> + Debug + Clone + std::fmt::Display> SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList { first: None }
    }

    fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    fn clear(&mut self) {
        self.first = None;
    }

    fn find_last_node(&mut self) -> Option<&mut Box<Node<T>>> {
        let mut current = &mut self.first;

        while let Some(node) = current {
            if node.next.is_none() {
                return Some(node);
            }

            current = &mut node.next;
        }
        None
    }

    fn find_before_last(&mut self) -> Option<&mut Box<Node<T>>> {
        let mut current_node = &mut self.first;

        while let Some(node) = current_node {
            if let Some(next_node) = &mut node.next {
                if next_node.next.is_none() {
                    return Some(node);
                }
            }
            current_node = &mut node.next;
        }
        None
    }

    fn find_node(&mut self, given_data: &T) -> Option<&mut Box<Node<T>>> {
        let mut current_node = &mut self.first;

        while let Some(node) = current_node {
            if &node.data == given_data {
                return Some(node); // Return early after inserting the new node
            }
            current_node = &mut node.next;
        }
        None
    }

    fn find_previous_node(&mut self, given_data: &T) -> Option<&mut Box<Node<T>>> {
        let mut current_node = &mut self.first;

        while let Some(node) = current_node {
            if let Some(next_node) = &mut node.next {
                if &next_node.data == given_data {
                    return Some(node);
                }
            }
            current_node = &mut node.next;
        }
        None
    }

    fn append(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));
        let last_node = self.find_last_node();
        match last_node {
            Some(node) => {
                node.next = Some(new_node);
            }
            None => {
                self.first = Some(new_node);
            }
        }
    }

    fn prepend(&mut self, data: T) {
        let new_node = Box::new(Node::new_with_next(data, self.first.take()));
        self.first = Some(new_node);
    }

    fn insert_after_given(&mut self, data: T, given_data: T) {
        if self.is_empty() {
            panic!("List is empty, this action is not possible.");
        }

        let node_with_data = &mut self.find_node(&given_data);
        match node_with_data {
            Some(node) => {
                let new_node = Box::new(Node::new_with_next(data, node.next.take()));
                node.next = Some(new_node);
            }
            None => {
                panic!("Given node '{}' not found in the list!", given_data);
            }
        }
    }

    fn insert_before_given(&mut self, data: T, given_data: T) {
        if self.is_empty() {
            panic!("List is empty, this action is not possible.");
        }

        let node_before = self.find_previous_node(&given_data);
        match node_before {
            Some(node) => {
                let new_node = Box::new(Node::new_with_next(data, node.next.take()));
                node.next = Some(new_node);
            }
            None => {
                panic!("Given node '{}' not found in the list!", given_data);
            }
        }
    }

    fn delete_first(&mut self) {
        if self.is_empty() {
            panic!("Cannot delete the first element from an empty list!");
        }
        let new_first = self.first.take().unwrap().next;
        self.first = new_first;
    }

    fn delete_last(&mut self) {
        let last_node = self.find_before_last();
        match last_node {
            Some(node) => {
                node.next = None;
            }
            None => {
                panic!("Cannot delete the last element from an empty list!");
            }
        }
    }

    fn delete_node_with_data(&mut self, data: T) {
        let data_node = self.find_node(&data);
        match data_node {
            Some(node) => {
                let reference = node.next.take();
                let previous_node = self.find_previous_node(&data);
                match previous_node {
                    Some(previous) => {
                        previous.next = reference;
                    }
                    None => {
                        self.first = None;
                    }
                }
            }
            None => {
                panic!("Node with given data not found!");
            }
        }
    }
}

impl<T: fmt::Display + Clone + std::fmt::Display> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.first;

        // Iterate over the nodes and format their data
        while let Some(node) = current {
            write!(f, "{} -> ", node.data)?;
            current = &node.next;
        }

        Ok(())
    }
}

pub fn run() {
    println!("In Singly Linked Lists");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Custom assertion macro to check if the list contains specific data
    macro_rules! assert_list_contains_data {
        ($list:expr, $expected_data:expr) => {
            let mut current = $list.first.as_ref();
            for expected in $expected_data {
                assert_eq!(current.map(|node| &node.data), Some(expected));
                current = current.unwrap().next.as_ref();
            }
            assert!(current.is_none());
        };
    }

    #[test]
    fn test_new_list_is_empty() {
        let list: SinglyLinkedList<String> = SinglyLinkedList::new();

        assert_eq!(list.first, None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_append_single_node() {
        let data = "Data Block 1";

        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append(data);

        assert_eq!(list.first, Some(Box::new(Node::new(data))));
        assert_eq!(
            list.first.as_ref().map(|node| &node.data),
            Some(&data)
        );
        assert_eq!(list.first.as_ref().unwrap().next, None);
    }

    #[test]
    fn test_append_multiple_nodes() {
        let values = ["A", "B", "C", "D"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();

        for value in &values {
            list.append(value);
        }

        let mut current = list.first.as_ref();

        for value in values {
            assert_eq!(
                current.map(|node| &node.data),
                Some(&value)
            );
            current = current.and_then(|node| node.next.as_ref());
        }
        assert_eq!(
            current.map(|node| &node.data),
            None
        );
    }

    #[test]
    fn test_prepend_empty_list() {
        let a = "A";
        let mut actual_list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        actual_list.prepend(a);

        assert_eq!(
            actual_list.first.as_ref().map(|node| &node.data),
            Some(&a)
        );
        assert_eq!(actual_list.first.as_ref().unwrap().next, None);
    }

    #[test]
    fn test_prepend_single_node_to_empty_list() {
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.prepend("A");

        assert_eq!(
            list.first.as_ref().map(|node| &node.data),
            Some(&"A")
        );
        assert_eq!(list.first.as_ref().unwrap().next, None);
    }

    #[test]
    fn test_prepend_to_non_empty_list() {
        let values = vec!["A", "B"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append(&values[0]);
        list.append(&values[1]);

        let first = list.first.as_ref().map(|node| &node.data);
        let second = list.first.as_ref().and_then(|node| node.next.as_ref().map(|node| &node.data));

        assert_eq!(first, Some(&values[0]));
        assert_eq!(second, Some(&values[1]));
    }

    #[test]
    fn test_prepend_adding_multiple_nodes() {
        let values = vec!["A", "B", "C"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        for value in values.iter().take(2) {
            list.append(value);
        }

        list.prepend(&values[2]);

        let expected_data = vec!["C", "A", "B"];

        assert_list_contains_data!(&list, &expected_data);
    }

    #[test]
    #[should_panic(expected = "List is empty, this action is not possible.")]
    fn test_insert_after_empty_list_panics() {
        let mut empty_list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        empty_list.insert_after_given("A", "B");
    }

    #[test]
    #[should_panic(expected = "Given node 'B' not found in the list!")]
    fn test_insert_after_given_data_not_found_panics() {
        let mut actual_list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        actual_list.append("A");
        actual_list.insert_after_given("C", "B");
    }

    #[test]
    fn test_insert_after_given_two_nodes_inserts_in_between_them() {
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append("A");
        list.append("B");

        list.insert_after_given("C", "A");

        let expected_data = vec!["A", "C", "B"];

        assert_list_contains_data!(&list, &expected_data);
    }

    #[test]
    #[should_panic(expected = "List is empty, this action is not possible.")]
    fn test_that_insert_before_panics_if_empty_list_given() {
        let mut empty_list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        empty_list.insert_before_given("A", "B")
    }

    #[test]
    #[should_panic(expected = "Given node 'B' not found in the list!")]
    fn test_that_insert_before_panics_if_given_node_not_found() {
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append("A");
        list.insert_before_given("C", "B");
    }

    #[test]
    fn test_insert_before_if_two_nodes_already_added_insert_between_them() {
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append("A");
        list.append("B");

        list.insert_before_given("C", "B");

        let expected_data = vec!["A", "C", "B"];

        assert_list_contains_data!(&list, &expected_data);
    }

    #[test]
    fn find_last_node_in_empty_list() {
        let mut empty_list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        let result = empty_list.find_last_node();
        assert_eq!(result, None)
    }

    #[test]
    fn find_last_node_when_list_has_single_node() {
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append("A");

        let result = list.find_last_node();

        assert_eq!(
            result.map(|node| &node.data),
            Some(&"A")
        );
    }

    #[test]
    fn find_last_node_when_multiple_nodes() {
        let values = vec!["A", "B", "C", "D"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();

        for value in &values {
            list.append(&value);
        }

        let result = list.find_last_node();
        assert_eq!(
            result.map(|node| &node.data),
            Some(&"D")
        );
    }

    #[test]
    fn find_before_last_when_empty_list() {
        let mut empty_list: SinglyLinkedList<&str> = SinglyLinkedList::new();

        let result = empty_list.find_before_last();

        assert_eq!(result, None);
    }

    #[test]
    fn find_before_last_when_single_node() {
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append("A");

        let result = list.find_before_last();

        assert_eq!(result, None);
    }

    #[test]
    fn find_before_last_when_multiple_nodes() {
        let values = vec!["A", "B", "C", "D"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        for value in &values {
            list.append(&value);
        }

        let result = list.find_before_last();

        assert_eq!(
            result.map(|node| &node.data),
            Some(&"C")
        );
    }

    #[test]
    fn find_node_when_empty_list() {
        let mut empty_list: SinglyLinkedList<&str> = SinglyLinkedList::new();

        let result = empty_list.find_node(&"A");

        assert_eq!(result, None);
    }

    #[test]
    fn find_node_when_single_node_in_list() {
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append("A");

        let result = list.find_node(&"A");

        assert_eq!(
            result.map(|node| &node.data),
            Some(&"A")
        );
    }

    #[test]
    fn find_node_when_single_node_but_given_node_not_found() {
        let values = vec!["A", "B", "C", "D"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        for value in &values {
            list.append(&value);
        }

        let result = list.find_node(&"Z");

        assert_eq!(result, None);
    }

    #[test]
    fn find_node_when_multiple_nodes_and_given_data_found() {
        let values = vec!["A", "B", "C", "D"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        for value in &values {
            list.append(&value);
        }
        let result = list.find_node(&"C");

        assert_eq!(
            result.map(|node| &node.data),
            Some(&"C")
        );
    }

    #[test]
    fn find_previous_node_when_empty_list() {
        let mut empty_list: SinglyLinkedList<&str> = SinglyLinkedList::new();

        let result = empty_list.find_previous_node(&"A");

        assert_eq!(result, None);
    }

    #[test]
    fn find_previous_node_when_single_node_in_list() {
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append("A");

        let result = list.find_previous_node(&"A");

        assert_eq!(result, None);
    }

    #[test]
    fn find_precious_node_when_multiple_nodes_in_list() {
        let values = vec!["A", "B", "C", "D"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        for value in &values {
            list.append(&value);
        }

        let result = list.find_previous_node(&"C");

        assert_eq!(
            result.map(|node| &node.data),
            Some(&"B")
        );
    }

    #[test]
    fn find_previous_node_when_multiple_nodes_data_not_found() {
        let values = vec!["A", "B", "C", "D"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        for value in &values {
            list.append(&value);
        }

        let result = list.find_previous_node(&"Z");

        assert_eq!(result, None);
    }

    #[test]
    #[should_panic(expected = "Cannot delete the first element from an empty list!")]
    fn delete_first_when_empty_list_panics() {
        let mut empty_list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        empty_list.delete_first();
    }

    #[test]
    fn delete_first_when_list_has_elements() {
        let values = vec!["A", "B", "C"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append(&values[0]);
        list.append(&values[1]);
        list.append(&values[2]);

        list.delete_first();

        let expected_data = vec!["B", "C"];

        assert_list_contains_data!(&list, &expected_data);
    }

    #[test]
    #[should_panic(expected = "Cannot delete the last element from an empty list!")]
    fn delete_last_panics_when_empty_list() {
        let mut empty_list: SinglyLinkedList<String> = SinglyLinkedList::new();
        empty_list.delete_last();
    }

    #[test]
    fn delete_last_when_list_has_elements() {
        let values = vec!["A", "B", "C"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append(&values[0]);
        list.append(&values[1]);
        list.append(&values[2]);

        list.delete_last();

        let expected_data = vec!["A", "B"];

        println!("### List : {:?}", list);

        assert_list_contains_data!(&list, &expected_data);
    }

    #[test]
    #[should_panic(expected = "Node with given data not found!")]
    fn delete_node_with_data_when_empty_list() {
        let mut empty_list: SinglyLinkedList<&str> = SinglyLinkedList::new();

        empty_list.delete_node_with_data("A");
    }

    #[test]
    #[should_panic(expected = "Node with given data not found!")]
    fn delete_node_with_data_when_nodes_present_but_data_not_found() {
        let values = vec!["A", "B", "C", "D"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        for value in &values {
            list.append(&value);
        }

        list.delete_node_with_data("Z");
    }

    #[test]
    fn delete_node_with_data_when_single_node_and_data_found() {
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append("A");
        list.delete_node_with_data("A");

        println!("### List Looks like this: {:?}", list);

        assert!(list.is_empty());
    }

    #[test]
    fn delete_node_with_data_when_multiple_nodes_and_node_present() {
        let values = vec!["A", "B", "C", "D"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        for value in &values {
            list.append(&value);
        }

        list.delete_node_with_data("C");

        let expected_data = vec!["A", "B", "D"];

        assert_list_contains_data!(&list, &expected_data);
    }

    #[test]
    fn test_display_empty_list() {
        let empty_list: SinglyLinkedList<&str> = SinglyLinkedList::new();

        assert_eq!(format!("{}", empty_list), "");
    }

    #[test]
    fn display_linked_list_with_single_node() {
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        list.append("A");

        assert_eq!(format!("{}", list), "A -> ")
    }

    #[test]
    fn display_linked_list_multiple_nodes() {
        let values = vec!["A", "B", "C", "D"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        for value in &values {
            list.append(&value);
        }

        assert_eq!(format!("{}", list), "A -> B -> C -> D -> ");
    }

    #[test]
    fn clear_when_multiple_nodes_exist_in_the_list() {
        let values = vec!["A", "B", "C", "D"];
        let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        for value in &values {
            list.append(&value);
        }

        list.clear();

        assert!(list.is_empty());
    }
}