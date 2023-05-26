// Implement Hash Map from scratch using Singly Linked List
// to avoid collisions.

use std::{ hash::Hash, fmt::Debug };
use std::collections::LinkedList;

use crate::hasher_trait::KeyToIndexHasherTrait;
use crate::hasher_trait::DEFAULT_MAX_SIZE;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HashMap<K, V> {
    current_size: usize,
    array: [Option<LinkedList<(K, V)>>; DEFAULT_MAX_SIZE],
}

impl<K: Hash + Clone, V> KeyToIndexHasherTrait<K> for HashMap<K, V> {}

#[allow(dead_code)]
impl<K: Hash + Clone + PartialEq + Debug, V: Clone + Debug> HashMap<K, V> {
    // Allows to work around lack of 'Copy' trait
    const INIT: Option<LinkedList<(K, V)>> = None;

    pub fn is_empty(&self) -> bool {
        self.current_size == 0
    }

    pub fn new() -> Self {
        HashMap {
            current_size: 0,
            array: [Self::INIT; DEFAULT_MAX_SIZE],
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.get_index(key.clone());
        if let Some(list) = &mut self.array[index] {
            if let Some(node) = list.iter_mut().find(|(k, _v)| *k == key) {
                let old_value = Some(node.1.clone());
                *node = (key, value);
                return old_value;
            } else {
                list.push_back((key, value));
            }
        } else {
            let mut list = LinkedList::new();
            list.push_back((key, value));
            self.array[index] = Some(list);
        }
        self.current_size += 1;
        None
    }

    // Gets value for a given key. Returns the value if it exists,
    // or None otherwise.
    pub fn get(&self, key: K) -> Option<V> {
        todo!()
    }

    // Removes the key-value pair from the map for a given key.
    // Returns the value is the key existed, None otherwise.
    pub fn remove(&self, key: K) -> Option<V> {
        todo!()
    }

    // Clears the hashmap.
    pub fn clear(&mut self) {
        todo!()
    }
}
pub fn run() {
    println!("Hash Table data structure added as module");
}

#[cfg(test)]
mod tests {
    use std::{ vec, fmt::Display };

    use super::*;

    #[allow(dead_code)]
    struct TestBuilder<K, V> {
        expected: [Option<LinkedList<(K, V)>>; DEFAULT_MAX_SIZE],
    }

    impl<K: Hash + Clone, V> KeyToIndexHasherTrait<K> for TestBuilder<K, V> {}

    impl<K: Clone + Hash + Display, V: Clone + Display> TestBuilder<K, V> {
        const INIT: Option<LinkedList<(K, V)>> = None;

        fn new() -> Self {
            TestBuilder { expected: [Self::INIT; DEFAULT_MAX_SIZE] }
        }

        fn build_expected_array(
            &mut self,
            values: &Vec<(K, V)>
        ) -> &[Option<LinkedList<(K, V)>>; DEFAULT_MAX_SIZE] {
            for (key, value) in values {
                let index = self.get_index(key.clone());
                if let Some(list) = &mut self.expected[index] {
                    list.push_back((key.clone(), value.clone()));
                } else {
                    let mut linked_list = LinkedList::new();
                    linked_list.push_back((key.clone(), value.clone()));
                    self.expected[index] = Some(linked_list);
                }
            }
            &self.expected
        }
    }

    #[test]
    fn when_new_hashmap_created_it_is_empty() {
        let map: HashMap<String, String> = HashMap::new();

        assert!(map.is_empty());
        assert_eq!(map.current_size, 0);
    }

    #[test]
    fn test_insert_when_no_elements_present_in_index() {
        let mut map: HashMap<&str, &str> = HashMap::new();
        let element = ("A", "Some Value A");

        let mut expected = LinkedList::new();
        expected.push_back(element);

        let result = map.insert(element.0, element.1);

        assert!(result.is_none(), "Result is none, because Key didn't exist");
        assert!(map.array.contains(&Some(expected)));
        assert_eq!(map.current_size, 1);
    }

    #[test]
    fn test_insert_when_adding_multiple_elements() {
        let values = vec![("A", "Value A"), ("B", "Value B"), ("C", "Value C"), ("D", "Value D")];
        let mut map: HashMap<&str, &str> = HashMap::new();

        for (key, value) in values.iter() {
            assert_eq!(map.insert(key, value), None);
        }

        for (key, value) in values {
            let mut expected = LinkedList::new();
            expected.push_back((key, value));
            assert!(map.array.contains(&Some(expected)));
        }
        assert_eq!(map.current_size, 4);
    }

    #[test]
    fn test_insert_when_key_already_present_value_updated_old_value_returned() {
        let key = "A";
        let old_value = "Old Value A";
        let new_value = "New Value A";
        let mut expected_list = LinkedList::new();
        expected_list.push_back((key, new_value));

        let mut map = HashMap::new();

        let result_1 = map.insert(key, old_value);
        let result_2 = map.insert(key, new_value);

        assert_eq!(result_1, None, "Puting Key first time returns None");
        assert_eq!(result_2, Some(old_value), "When key present, existing value returned");
        assert!(map.array.contains(&Some(expected_list)), "Key value is updated to new value");
        assert_eq!(map.current_size, 1);
    }

    #[test]
    fn when_two_different_keys_map_to_same_index() {
        let values = vec![
            ("A", "Value for A"),
            ("K", "Value for K"),
            ("Q", "Value for Q"),
            ("Z", "Value for Z")
        ];
        let mut map: HashMap<&str, &str> = HashMap::new();
        for (key, value) in &values {
            map.insert(key, value);
        }

        assert_eq!(
            &map.get_index(&values[1].0),
            &map.get_index(&values[2].0),
            "Keys K and Q map to the same index."
        );

        let mut test_builder = TestBuilder::new();
        let expected = test_builder.build_expected_array(&values);
        assert_eq!(expected, &map.array);
        assert_eq!(map.current_size, 4);
    }
}
