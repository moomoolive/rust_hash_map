use std::clone::{ Clone };
use std::default::{ Default };
use std::mem::{ replace };

mod entry {
    use super::{ Clone };
    use super::{ replace };

    #[derive(Debug, Clone)]
    pub struct HashMapEntry<T: Clone> {
        key: String,
        value: Option<T>
    }
    
    impl <T: Clone> HashMapEntry<T> {
    
        pub fn new(key: String, value: T) -> HashMapEntry<T> {
            let value = Some(value);
            return HashMapEntry { key, value }
        }
    
        pub fn get_value(&self) -> &Option<T> {
            return &self.value
        }

        pub fn get_key(&self) -> &str {
            return &self.key
        }
        
        pub fn remove_value(&mut self) -> Option<T> {
            return replace(&mut self.value, None)
        }
    }

}

use entry::{ HashMapEntry };

/// A generic implementation of a hashmap in Rust
/// hashing collisions are resolved by linear probing strategy
/// Does not resize buffer if buffer is at full capacity
/// Hashmap will simply not input the data in that circumstance
#[derive(Debug, Clone)]
pub struct HashMap<T: Clone> {
    buffer: Vec<Option<HashMapEntry<T>>>
}

impl <T: Clone> HashMap <T> {

    /// constructor
    pub fn new(buffer_length: usize) -> HashMap<T> {
        let default_item: Option<HashMapEntry<T>> = None;
        let buffer = vec![default_item; buffer_length];
        return HashMap { buffer }
    }

    fn buffer_capacity(&self) -> usize {
        return self.buffer.len()
    }

    fn create_hash(&self, s: &str) -> usize {
        let mut sum = 0;
        let mut factor = 1;
        for (i, byte) in s.as_bytes().iter().enumerate() {
            factor = if i % 2 == 0 { 1 } else { factor * 256 };
            let casted_byte = *byte as i64;
            sum += casted_byte * factor;
        }
        return sum as usize % self.buffer_capacity()
    }

    fn valid_next_index(&self, i: usize) -> usize {
        return if i >= self.buffer_capacity() { 0 } else { i }
    }

    fn is_same_key(&self, i: usize, key: &str) -> bool {
        let entry_key = match &self.buffer[i] {
            Some(x) => Some(x.get_key()),
            None => None
        };
        return entry_key == Some(key)
    }

    fn find_nearest_empty_slot(&self, key: &str) -> i32 {
        let mut current_index = self.create_hash(key);
        let mut iteration_count = 0;
        while self.buffer[current_index].is_some() {
            let key_exists = self.is_same_key(current_index, key);
            if iteration_count > self.buffer_capacity() || key_exists  {
                return -1
            }
            current_index = self.valid_next_index(current_index + 1);
            iteration_count += 1;
        }
        return current_index as i32
    }

    pub fn insert(&mut self, key: &str, value: T) {
        let hash = self.find_nearest_empty_slot(key);
        if hash < 0 {
            return
        }
        let key = String::from(key);
        let entry = HashMapEntry::new(key, value);
        self.buffer[hash as usize] = Some(entry);
    }

    fn find_index_by_key(&self, key: &str) -> i32 {
        let mut current_index = self.create_hash(key);
        let mut iteration_count = 0;
        while self.buffer[current_index].is_some() {
            if iteration_count > self.buffer_capacity() {
                return -1
            }
            if self.is_same_key(current_index, key) {
                return current_index as i32
            }
            current_index = self.valid_next_index(current_index + 1);
            iteration_count += 1;
        }
        return -1
    }

    pub fn get(&self, key: &str) -> &Option<T> {
        let hash = self.find_index_by_key(key);
        if hash < 0 {
            return &None
        }
        let val = &self.buffer[hash as usize];
        return match val {
            Some(x) => x.get_value(),
            None => &None
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<T> {
        let hash = self.find_index_by_key(key);
        if hash < 0 {
            return None
        }
        let entry = replace(&mut self.buffer[hash as usize], None);
        return match entry {
            Some(mut x) => x.remove_value(),
            None => None
        }
    }
}

impl<T: Clone> Default for HashMap<T> {

    fn default() -> Self {
        return HashMap::<T>::new(100);
    }
}

#[cfg(test)]
mod test {
    use super::{ HashMap };

    #[test]
    fn inserted_key_should_be_able_to_be_recalled_part_1() {
        let mut map: HashMap<i32> = HashMap::default();
        let key = "hello";
        let value = 32;
        map.insert(key, value);
        let recalled = map.get(key);
        assert_eq!(recalled, &Some(value))
    }

    #[test]
    fn inserted_key_should_be_able_to_be_recalled_part_2() {
        let mut map: HashMap<String> = HashMap::default();
        let key = "hello";
        map.insert(key, String::from("dragon"));
        let recalled = map.get(key);
        assert_eq!(recalled, &Some(String::from("dragon")))
    }

    #[test]
    fn inserted_key_should_be_able_to_be_recalled_part_3() {
        let mut map: HashMap<bool> = HashMap::default();
        let key1 = "kk";
        let value1 = true;
        map.insert(key1, value1);
        let key2 = "yeah";
        let value2 = false;
        map.insert(key2, value2);
        let recalled = map.get(key1);
        assert_eq!(recalled, &Some(value1));
        let recalled = map.get(key2);
        assert_eq!(recalled, &Some(value2));
    }

    #[test]
    fn removed_key_should_return_none_part_1() {
        let mut map: HashMap<i32> = HashMap::default();
        let key = "hello";
        let value = 32;
        map.insert(key, value);
        let recalled = map.get(key);
        assert_eq!(recalled, &Some(value));
        assert_eq!(map.remove(key), Some(value));
        assert_eq!(map.get(key), &None);
    }

    #[test]
    fn removed_key_should_return_none_part_2() {
        let mut map: HashMap<String> = HashMap::default();
        let key = "hello";
        let value = "dragon";
        map.insert(key, String::from(value));
        assert_eq!(map.get(key), &Some(String::from(value)));
        assert_eq!(map.remove(key), Some(String::from(value)));
        assert_eq!(map.get(key), &None);
    }

    #[test]
    fn duplicate_key_should_not_be_inserted() {
        let mut map: HashMap<i32> = HashMap::default();
        let key = "hello";
        let value = 32;
        map.insert(key, value);
        assert_eq!(map.get(key), &Some(value));
        map.insert(key, 44);
        assert_eq!(map.get(key), &Some(value));
    }
}