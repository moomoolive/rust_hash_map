fn main() {
    let mut map = HMap::new();
    println!("{:?}", map);
    map.insert(&"yeah", 4);
    println!("{:?}", map);
    println!("{:?}", map.get_key(&"yeah"))
}

// this hashmap can only take 32-bit integers
#[derive(Debug)]
struct HashMapEntry {
    key: String,
    value: i32
}

// need for Rust borrow checker
impl Clone for HashMapEntry {
    fn clone(&self) -> HashMapEntry {
        return HashMapEntry { key: self.key.clone(), value: self.value }
    }
}

// implemented via an array of Vectors
#[derive(Debug)]
struct HMap {
    buckets: Vec<Vec<HashMapEntry>>
}

impl HMap {

    fn new() -> HMap {
        let number_of_letters_in_alphabet = 26;
        return HMap {
            buckets: vec![Vec::new(); number_of_letters_in_alphabet]
        }
    }

    // uses only lower case ASCII codes as hashes
    fn get_hash(&self, k: &str) -> i8 {
        let not_hashable_index: i8 = -1;
        // reject if key name is empty string
        if k.is_empty() {
            return not_hashable_index
        }
        let first_letter = k.to_ascii_lowercase().as_bytes()[0];
        let max_lowercase_ascii_code: u8 = 123;
        let min_lowercase_ascii_code: u8 = 97;
        // reject if key name is not alphabetic
        if first_letter > max_lowercase_ascii_code || first_letter < min_lowercase_ascii_code {
            return not_hashable_index
        }
        return (first_letter - min_lowercase_ascii_code) as i8
    }

    fn insert(&mut self, k: &str, v: i32) -> Option<i32> {
        let hash = self.get_hash(k);
        // if invalid key do not record value
        if hash < 0 {
            return None
        }
        let target_bucket_index = hash as usize;
        let mut doesExist = false;
        for e in &self.buckets[target_bucket_index] {
            if &e.key == k {
                doesExist = true;
                break
            }
        }
        // if key exists do not overwrite
        if doesExist {
            return None
        }
        let new_value = HashMapEntry { key: String::from(k), value: v };
        self.buckets[target_bucket_index].push(new_value);
        return Some(0)
    }

    fn get_key(&self, k: &str) -> Option<i32> {
        let hash = self.get_hash(k);
        // if invalid key return null value
        if hash < 0 {
            return None
        }
        let target_bucket = &self.buckets[hash as usize];
        for e in target_bucket {
            if &e.key == k {
                return Some(e.value)
            }
        }
        return None
    }

}

mod test {
    use super::*;

    #[test]
    fn inserting_and_retrieving_same_key_works_1() {
        let val = 4;
        let key = &"yeah";
        let mut map = HMap::new();
        map.insert(key, val);
        assert_eq!(val, map.get_key(key).unwrap())
    }

    #[test]
    fn inserting_and_retrieving_same_key_works_2() {
        let val = 7;
        let key = &"blah";
        let mut map = HMap::new();
        map.insert(key, val);
        assert_eq!(val, map.get_key(key).unwrap())
    }

    #[test]
    fn empty_string_is_not_accepted_as_a_valid_key() {
        let val = 7;
        let key = &"";
        let mut map = HMap::new();
        map.insert(key, val);
        assert_eq!(None, map.get_key(key))
    }

    #[test]
    fn non_alphabetic_character_not_accepted_as_key_1() {
        let val = 7;
        let key = &"+";
        let mut map = HMap::new();
        map.insert(key, val);
        assert_eq!(None, map.get_key(key))
    }

    #[test]
    fn non_alphabetic_character_not_accepted_as_key_2() {
        let val = 7;
        let key = &"1";
        let mut map = HMap::new();
        map.insert(key, val);
        assert_eq!(None, map.get_key(key))
    }

    #[test]
    fn non_alphabetic_character_not_accepted_as_key_3() {
        let val = 7;
        let key = &" ";
        let mut map = HMap::new();
        map.insert(key, val);
        assert_eq!(None, map.get_key(key))
    }

    #[test]
    fn inserting_with_key_that_already_exists_fails() {
        let val = 7;
        let key = &"blah";
        let mut map = HMap::new();
        map.insert(key, val);
        // same key, new value
        map.insert(key, 5);
        assert_eq!(val, map.get_key(key).unwrap())
    }
}