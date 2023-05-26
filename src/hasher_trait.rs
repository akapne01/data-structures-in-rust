use std::hash::{ Hash, Hasher };
use std::collections::hash_map::DefaultHasher;

pub const DEFAULT_MAX_SIZE: usize = 256;

pub trait KeyToIndexHasherTrait<K: Hash> {
    fn get_index(&self, key: K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let key_hash = hasher.finish();
        (key_hash % (DEFAULT_MAX_SIZE as u64)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestKeyToIndexStruct {}
    impl<K: Hash> KeyToIndexHasherTrait<K> for TestKeyToIndexStruct {}

    impl TestKeyToIndexStruct {
        fn new() -> Self {
            TestKeyToIndexStruct {}
        }
    }

    #[test]
    fn get_index_string() {
        let test_struct = TestKeyToIndexStruct::new();

        let index = test_struct.get_index(&"A");

        assert_eq!(index, 163);
    }

    #[test]
    fn get_index_integer() {
        let test_struct = TestKeyToIndexStruct::new();

        let index = test_struct.get_index(128);

        assert_eq!(index, 15);
    }

    #[test]
    fn test_that_same_key_returns_the_same_index() {
        let test_struct = TestKeyToIndexStruct::new();
        let index_1 = test_struct.get_index("KeyA");
        let index_2 = test_struct.get_index("KeyA");

        assert_eq!(index_1, index_2, "Same keys always return_the_same_index.");
    }

    #[test]
    fn test_that_different_keys_mapped_to_different_indexes() {
        let test_struct = TestKeyToIndexStruct::new();
        let index_1 = test_struct.get_index("KeyA");
        let index_2 = test_struct.get_index("KeyB");

        assert_ne!(index_1, index_2, "Keys that are different, map to differnt indexes.");
    }
}
