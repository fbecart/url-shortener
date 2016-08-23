use rand;
use rand::Rng;

use std::collections::HashMap;

pub trait KeyValueStore: 'static + Send + Sync + Sized {
    fn insert(&mut self, value: String) -> String;
    fn get(&self, key: &str) -> Option<String>;
}

pub struct InMemoryKeyValueStore {
    hashmap: HashMap<String, String>,
}

impl InMemoryKeyValueStore {
    pub fn new() -> Self {
        InMemoryKeyValueStore { hashmap: HashMap::new() }
    }

    fn generate_unused_random_key(hashmap: &HashMap<String, String>) -> String {
        let mut url_key: Option<String> = None;
        while url_key.is_none() {
            let random_key: String = rand::thread_rng().gen_ascii_chars().take(7).collect();
            url_key = if hashmap.contains_key(&random_key) {
                None
            } else {
                Some(random_key)
            }
        }
        url_key.unwrap()
    }
}

impl KeyValueStore for InMemoryKeyValueStore {
    fn insert(&mut self, value: String) -> String {
        let key = Self::generate_unused_random_key(&self.hashmap);
        self.hashmap.insert(key.clone(), value);
        key
    }

    fn get(&self, key: &str) -> Option<String> {
        self.hashmap.get(key).map(|s| s.to_owned())
    }
}
