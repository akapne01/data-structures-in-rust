// Implement Hash Map from scratch using Singly Linked List
// to avoid collisions.

use std::{
    collections::hash_map::DefaultHasher,
    hash::{ Hash, Hasher },
    fmt::{ self, Display, Debug },
};

use crate::singly_linked_list::SinglyLinkedList;

const DEFAULT_MAX_SIZE: u64 = 256;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct KeyValueStore<K: Clone + Display, V: Clone> {
    key: K,
    value: V,
    next: Option<SinglyLinkedList<KeyValueStore<K, V>>>,
}

impl<K: Clone + Display, V: Clone + Display> fmt::Display for KeyValueStore<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key: {}, Value: {}", self.key, self.value)
    }
}

#[allow(dead_code)]
impl<K: Clone + Display + Debug + PartialEq, V: Clone + Display + Debug + PartialEq> KeyValueStore<
    K,
    V
> {
    pub fn new(key: K, value: V) -> KeyValueStore<K, V> {
        KeyValueStore {
            key,
            value,
            next: None,
        }
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    pub fn update_next(&mut self, next: KeyValueStore<K, V>) {
        match &mut self.next {
            Some(list) => {
                list.append(next);
            }
            None => {
                let mut list: SinglyLinkedList<KeyValueStore<K, V>> = SinglyLinkedList::new();
                list.append(next);
                self.next = Some(list);
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HashMap<K: Clone + Display, V: std::clone::Clone> {
    current_size: usize,
    array: [Option<KeyValueStore<K, V>>; DEFAULT_MAX_SIZE as usize],
}

#[allow(dead_code)]
fn hash_key<K: Hash>(key: K) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

#[allow(dead_code)]
impl<K: Clone + Hash + Display + Debug + PartialEq, V: Clone + Display + Debug + PartialEq> HashMap<
    K,
    V
> {
    // Allows to work around lack of 'Copy' trait
    const INIT: Option<KeyValueStore<K, V>> = None;

    pub fn is_empty(&self) -> bool {
        self.current_size == 0
    }

    pub fn new() -> Self {
        HashMap { current_size: 0, array: [Self::INIT; DEFAULT_MAX_SIZE as usize] }
    }

    // Inserts Key value pair in the hashmap
    // Returns None if the value didn't exist, or returns the old value if the
    // key was present.
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        todo!();
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
    use super::*;

    // Custom assertion macro to check if the list contains specific data
    macro_rules! assert_list_contains_data {
        ($list:expr, $expected_data:expr) => {
            let mut current = $list.as_ref().unwrap().first.as_ref();
            for expected in $expected_data {
                assert_eq!(current.map(|node| &node.data), Some(expected));
                current = current.unwrap().next.as_ref();
            }
            assert!(current.is_none());
        };
    }

    #[test]
    fn test_key_value_store_creation_when_no_next() {
        let key = "key";
        let value = "value";
        let store = KeyValueStore::new(key, value);

        assert_eq!(store.key, key);
        assert_eq!(store.value, value);
        assert!(store.next.is_none());
    }

    #[test]
    fn test_key_value_store_when_has_next() {
        let mut store = KeyValueStore::new("A", "A Value");
        let next = KeyValueStore::new("B", "Value B");

        let mut expected_next = SinglyLinkedList::new();
        expected_next.append(next.clone());

        store.update_next(next);

        assert_eq!(store.key, "A");
        assert_eq!(store.value, "A Value");
        assert!(store.next.is_some());
        assert_eq!(expected_next, store.next.unwrap());
    }

    #[test]
    fn test_key_value_store_when_next_already_exists() {
        let mut store = KeyValueStore::new("A", "Value A");
        let next = KeyValueStore::new("B", "Value B");
        let new_next_node = KeyValueStore::new("C", "Value C");

        store.update_next(next.clone());
        store.update_next(new_next_node.clone());

        let expected_values = vec![next, new_next_node];

        assert_eq!(store.key, "A");
        assert_eq!(store.value, "Value A");
        assert!(store.has_next());
        assert_list_contains_data!(&store.next, &expected_values);
    }

    #[test]
    fn when_new_hashmap_created_it_is_empty() {
        let map: HashMap<String, String> = HashMap::new();

        assert!(map.is_empty());
    }
}