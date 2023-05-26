// Implement Hash Map from scratch using Singly Linked List
// to avoid collisions.

use std::{ hash::Hash, fmt::Debug };
use std::collections::LinkedList;

use crate::hasher_trait::KeyToIndexHasherTrait;
use crate::hasher_trait::DEFAULT_MAX_SIZE;

fn find_index_linked_list<K, V>(list: &LinkedList<(K, V)>, key: &K) -> Option<usize>
    where K: PartialEq
{
    let mut current = list.iter();
    let mut index = 0;

    while let Some((k, _)) = current.next() {
        if k == key {
            return Some(index);
        }

        index += 1;
    }

    None
}

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

    // Inserts key and value pair in the hashmap. If key didn't exist, returns None
    // If key is present, returns the old value and updates stored value to the new value.
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
        let index = self.get_index(key.clone());
        let list_with_value = &self.array[index];
        match list_with_value {
            Some(list) => {
                if let Some(node) = list.iter().find(|(k, _v)| *k == key) {
                    return Some(node.1.clone());
                } else {
                    None
                }
            }
            None => { None }
        }
    }

    // Removes the key-value pair from the map for a given key.
    // Returns the value is the key existed, None otherwise.
    // To remove we need to know index
    pub fn remove(&mut self, key: K) -> Option<V> {
        let mut return_value = None;
        let index = self.get_index(key.clone());
        if let Some(list) = &mut self.array[index] {
            if let Some(node) = list.iter().find(|(k, _v)| *k == key) {
                return_value = Some(node.1.clone());
            }
            let remove_data_at_index = find_index_linked_list(list, &key);
            if remove_data_at_index.is_some() {
                // There is an issue to remove data from linked list in GitHub,
                // implemented suggested workaround to split list, and unite again.
                // https://github.com/rust-lang/rust/issues/69210#issuecomment-647864685
                let index_to_remove = remove_data_at_index.unwrap();
                if index_to_remove != 0 {
                    let mut split_list = list.split_off(index_to_remove);
                    split_list.pop_front();
                    list.append(&mut split_list);
                } else {
                    self.array[index] = None;
                }
                self.current_size -= 1;
            }
        }
        return_value
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
    struct HashMapTestBuilder<K, V> {
        expected: [Option<LinkedList<(K, V)>>; DEFAULT_MAX_SIZE],
    }

    impl<K: Hash + Clone, V> KeyToIndexHasherTrait<K> for HashMapTestBuilder<K, V> {}

    impl<
        K: Clone + Hash + Display + Debug + PartialEq,
        V: Clone + Display + Debug + PartialEq
    > HashMapTestBuilder<K, V> {
        const INIT: Option<LinkedList<(K, V)>> = None;

        fn new() -> Self {
            HashMapTestBuilder { expected: [Self::INIT; DEFAULT_MAX_SIZE] }
        }

        fn build_expected_array(
            &mut self,
            values: &Vec<(K, V)>
        ) -> [Option<LinkedList<(K, V)>>; DEFAULT_MAX_SIZE] {
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
            self.expected.clone()
        }

        fn new_map_with_values_set(values: &Vec<(K, V)>) -> HashMap<K, V> {
            let mut map: HashMap<K, V> = HashMap::new();
            for (key, value) in values {
                map.insert(key.clone(), value.clone());
            }
            map
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
        let values = vec![("A", "Some Value A")];
        let mut test_builder = HashMapTestBuilder::new();
        let expected_array = test_builder.build_expected_array(&values);

        let mut map: HashMap<&str, &str> = HashMap::new();
        let result = map.insert(values[0].0, values[0].1);

        assert!(result.is_none(), "Result is none, because Key didn't exist");
        assert_eq!(&map.array, &expected_array);
        assert_eq!(map.current_size, 1);
    }

    #[test]
    fn test_insert_when_adding_multiple_elements() {
        let values = vec![("A", "Value A"), ("B", "Value B"), ("C", "Value C"), ("D", "Value D")];
        let mut test_builder = HashMapTestBuilder::new();
        let expected_array = test_builder.build_expected_array(&values);
        let mut map: HashMap<&str, &str> = HashMap::new();

        for (key, value) in values.iter() {
            assert_eq!(map.insert(key, value), None);
        }

        assert_eq!(&map.array, &expected_array);
        assert_eq!(map.current_size, 4);
    }

    #[test]
    fn test_insert_when_key_already_present_value_updated_old_value_returned() {
        let key = "A";
        let old_value = "Old Value A";
        let new_value = "New Value A";
        let mut test_builder = HashMapTestBuilder::new();
        let expected_array = test_builder.build_expected_array(&vec![(key, new_value)]);
        let mut map = HashMap::new();

        let result_1 = map.insert(key, old_value);
        let result_2 = map.insert(key, new_value);

        assert_eq!(result_1, None, "Puting Key first time returns None");
        assert_eq!(result_2, Some(old_value), "When key present, existing value returned");
        assert_eq!(&map.array, &expected_array);
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
        let map: HashMap<&str, &str> = HashMapTestBuilder::new_map_with_values_set(&values);

        assert_eq!(
            &map.get_index(&values[1].0),
            &map.get_index(&values[2].0),
            "Keys K and Q map to the same index."
        );

        let mut test_builder = HashMapTestBuilder::new();
        let expected = test_builder.build_expected_array(&values);
        assert_eq!(&expected, &map.array);
        assert_eq!(map.current_size, 4);
    }

    #[test]
    fn test_get_when_value_not_present_returns_none() {
        let empty_map: HashMap<&str, &str> = HashMap::new();

        let result = empty_map.get("Key A");

        assert_eq!(empty_map.current_size, 0);
        assert!(empty_map.is_empty());
        assert!(result.is_none());
    }

    #[test]
    fn test_get_when_one_node_value_returned() {
        let values = vec![("Key A", "Value A")];
        let map = HashMapTestBuilder::<&str, &str>::new_map_with_values_set(&values);

        let result = map.get(values[0].0);

        assert!(result.is_some());
        assert_eq!(result, Some(values[0].1));
        assert_eq!(map.current_size, 1);
    }

    #[test]
    fn test_get_with_multiple_nodes() {
        let values = vec![("A", "Value A"), ("B", "Value B"), ("C", "Value C"), ("D", "Value D")];
        let map = HashMapTestBuilder::<&str, &str>::new_map_with_values_set(&values);

        for (key, value) in values {
            let result = map.get(key);
            assert_eq!(result, Some(value));
        }
        assert_eq!(map.current_size, 4);
    }

    #[test]
    fn test_get_when_key_updated_and_multiple_nodes() {
        let values = vec![
            ("A", "Old Value A"),
            ("B", "Value B"),
            ("C", "Value C"),
            ("D", "Value D"),
            ("A", "New Value A")
        ];
        let map = HashMapTestBuilder::<&str, &str>::new_map_with_values_set(&values);

        let expected_values = vec![
            ("A", "New Value A"),
            ("B", "Value B"),
            ("C", "Value C"),
            ("D", "Value D")
        ];

        for (key, value) in expected_values {
            let result = map.get(key);
            assert!(result.is_some());
            assert_eq!(result, Some(value));
        }
        assert_eq!(map.current_size, 4);
    }

    #[test]
    fn test_get_when_collision_of_indexes() {
        let values = vec![
            ("A", "Value A"),
            ("B", "Value B"),
            ("C", "Value C"),
            ("K", "Value K"),
            ("Q", "Value Q")
        ];
        let map = HashMapTestBuilder::<&str, &str>::new_map_with_values_set(&values);

        assert_eq!(
            map.get_index(values[3].0),
            map.get_index(values[4].0),
            "Keys K and Q map to the same index."
        );

        for (key, value) in values {
            let result = map.get(key);
            assert!(result.is_some());
            assert_eq!(result, Some(value));
        }
    }

    #[test]
    fn test_remove_when_one_node_added_key_not_found() {
        let values = vec![("A", "Value A")];
        let mut map = HashMapTestBuilder::new_map_with_values_set(&values);

        let result = map.remove("Z");

        assert!(result.is_none());
        assert_eq!(map.current_size, 1);
    }

    #[test]
    fn test_remove_when_one_node_added_key_present() {
        let values = vec![("A", "Value A")];
        let mut map = HashMapTestBuilder::new_map_with_values_set(&values);

        let result = map.remove("A");

        assert!(result.is_some());
        assert_eq!(result, Some("Value A"));
        assert_eq!(map.current_size, 0);
    }

    #[test]
    fn test_remove_when_multiple_nodes() {
        let values = vec![
            ("A", "Value A"),
            ("B", "Value B"),
            ("C", "Value C"),
            ("D", "Value D"),
            ("E", "Value E"),
            ("F", "Value F"),
            ("G", "Value G"),
            ("H", "Value H"),
            ("I", "Value I")
        ];
        let mut map = HashMapTestBuilder::new_map_with_values_set(&values);
        let keys_to_remove = vec![
            ("A", "Value A"),
            ("C", "Value C"),
            ("D", "Value D"),
            ("F", "Value F"),
            ("H", "Value H")
        ];
        let expected_values = vec![
            ("B", "Value B"),
            ("E", "Value E"),
            ("G", "Value G"),
            ("I", "Value I")
        ];
        let expected_array = HashMapTestBuilder::new().build_expected_array(&expected_values);

        for (key, value) in keys_to_remove {
            let result = map.remove(key);
            assert!(result.is_some());
            assert_eq!(result, Some(value), "Remove returns value that key had");
        }

        assert_eq!(map.current_size, 4);
        assert_eq!(&map.array, &expected_array);
    }

    #[test]
    fn test_remove_when_two_differnt_keys_map_to_same_index() {
        let values = vec![
            ("A", "Value A"),
            ("B", "Value B"),
            ("C", "Value C"),
            ("K", "Value K"),
            ("Q", "Value Q")
        ];
        let values_to_remove = vec![("A", "Value A"), ("Q", "Value Q"), ("K", "Value K")];
        let mut map = HashMapTestBuilder::<&str, &str>::new_map_with_values_set(&values);
        let expected_values = vec![("B", "Value B"), ("C", "Value C")];
        let expected_array = HashMapTestBuilder::new().build_expected_array(&expected_values);
        assert_eq!(
            map.get_index(values[3].0),
            map.get_index(values[4].0),
            "Keys K and Q map to the same index."
        );

        for (key, value) in values_to_remove {
            let result = map.remove(key);
            assert!(result.is_some());
            assert_eq!(result, Some(value));
        }

        assert_eq!(map.array, expected_array);
        assert_eq!(map.current_size, 2);
    }

    #[test]
    fn test_remove_when_all_values_removed() {
        let values = vec![
            ("A", "Value A"),
            ("B", "Value B"),
            ("C", "Value C"),
            ("D", "Value D"),
            ("E", "Value E")
        ];
        let mut map = HashMapTestBuilder::new_map_with_values_set(&values);

        for &(key, value) in &values {
            let result = map.remove(key);
            assert!(result.is_some());
            assert_eq!(result, Some(value));
        }

        assert!(map.is_empty());
        assert_eq!(map.current_size, 0);
    }
}
