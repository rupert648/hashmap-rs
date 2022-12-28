use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::DerefMut,
};

#[cfg(test)]
mod tests;

const DEFAULT_MAX_SIZE: u64 = 256;
// hacky workaround the lack of String::copy()
// for initializing the array
const NONE: Option<KeyValue> = None;

pub struct HashMap {
    curr_size: usize,
    arr: [Option<KeyValue>; DEFAULT_MAX_SIZE as usize],
}

#[derive(Clone, Debug)]
pub struct KeyValue {
    key: String,
    value: i32,
    next: Option<Box<KeyValue>>,
}

// string, i32 for now
impl HashMap {
    pub fn new() -> Self {
        HashMap {
            curr_size: 0,
            arr: [NONE; DEFAULT_MAX_SIZE as usize],
        }
    }

    /// Inserts a key: value pair into the hashmap
    ///
    /// Returns None if the key didn't exist
    /// Returns the old value if the key wasn't present
    /// and updates it with the new value.
    pub fn put(&mut self, key: String, val: i32) -> Option<i32> {
        let hash_val: u64 = hash_string(key.clone());

        let position = hash_val % DEFAULT_MAX_SIZE;

        let result = match &self.arr[position as usize] {
            Some(_) => self.update_or_link_new_val(key, val, position as usize),
            None => {
                self.insert_new_value(key, val, position as usize);
                return None;
            }
        };

        result
    }

    /// Gets a the given value for a key.
    ///
    /// Returns the value if it exists
    /// None otherwise
    pub fn get(&self, key: String) -> Option<i32> {
        let hash_val: u64 = hash_string(key.clone());
        let position = hash_val % DEFAULT_MAX_SIZE;

        let result = match &self.arr[position as usize] {
            Some(_) => self.check_list_for_key(key, position as usize),
            None => None,
        };

        result
    }

    /// Removes a value from the map, returning the value
    /// if that key existed.
    ///
    /// Returns none if the value does not exist.
    pub fn remove(&mut self, key: String) -> Option<i32> {
        let hash_val: u64 = hash_string(key.clone());
        let position: u64 = hash_val % DEFAULT_MAX_SIZE;

        let result = match &self.arr[position as usize] {
            Some(_) => self.check_item_in_list_and_remove(key, position as usize),
            None => None,
        };

        result
    }

    /// Clears the HashMap
    pub fn clear(&self) {
        !todo!()
    }

    /// Returns the number of keys in
    /// the HashMap
    pub fn length(&self) {
        !todo!()
    }

    fn insert_new_value(&mut self, key: String, val: i32, position: usize) {
        let new_entry = KeyValue::new(key, val);

        self.arr[position] = Some(new_entry);
    }

    fn update_or_link_new_val(&mut self, key: String, val: i32, position: usize) -> Option<i32> {
        // traverse linked list until either find value (update)
        // or stick a new value on the end

        // can safely unwrap as we've already checked this pos exists
        let key_val = self.arr[position].as_mut().unwrap();
        if key_val.key == key {
            let old_val = key_val.value;
            key_val.value = val;
            // return the old value
            return Some(old_val);
        }

        let mut current = key_val;
        while !current.next.is_none() {
            let node = current.next.as_mut().unwrap();

            if node.key == key {
                // update the value
                let old_val = node.value;
                node.value = val;
                return Some(old_val);
            }

            current = node;
        }

        // append the new value to the end of the linked list
        let new_key_val = KeyValue::new(key, val);

        current.next = Some(Box::new(new_key_val));

        None
    }

    fn check_list_for_key(&self, key: String, position: usize) -> Option<i32> {
        let mut current = self.arr[position].as_ref().unwrap();
        if current.key == key {
            return Some(current.value);
        }

        while let Some(node) = current.next.as_ref() {
            if node.key == key {
                return Some(node.value);
            }

            current = node;
        }

        None
    }

    fn check_item_in_list_and_remove(&mut self, key: String, position: usize) -> Option<i32> {
        let mut current = self.arr[position].as_ref().unwrap();
        if current.key == key {
            let return_val = current.value;

            // check if there is a next val
            // if there is next, update array to point to this val
            if let Some(node) = current.next.to_owned() {
                self.arr[position] = Some(*node);
            } else {
                self.arr[position] = NONE
            }

            // return the value the node held
            return Some(return_val);
        }

        while let Some(node) = current.next.as_ref() {
            if node.key == key {
                let return_val = node.value;

                // check if there is a next val
                // if there is next, update array to point to this val
                if let Some(node_next) = node.next.to_owned() {
                    self.arr[position] = Some(*node_next);
                } else {
                    self.arr[position] = NONE
                }

                // return the value the node held
                return Some(return_val);
            }

            current = node;
        }

        None
    }
}

impl KeyValue {
    pub fn new(key: String, value: i32) -> Self {
        let copied = key.clone();
        KeyValue {
            key: copied,
            value,
            next: None,
        }
    }
}

fn hash_string(key: String) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let hash_val = hasher.finish();
    hash_val
}
