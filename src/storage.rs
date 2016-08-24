use rand;
use rand::Rng;

use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

pub trait KeyValueStore: 'static + Send + Sync + Sized {
    fn insert(&mut self, value: String) -> String;
    fn get(&self, key: &str) -> Option<String>;
}

pub struct InMemoryKeyValueStore {
    data: HashMap<String, String>,
}

impl InMemoryKeyValueStore {
    #[cfg(test)]
    pub fn new() -> Self {
        InMemoryKeyValueStore { data: HashMap::new() }
    }

    pub fn with_initial_data(data: HashMap<String, String>) -> Self {
        InMemoryKeyValueStore { data: data }
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
        let key = Self::generate_unused_random_key(&self.data);
        self.data.insert(key.clone(), value);
        key
    }

    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).map(|s| s.to_owned())
    }
}

const KEY_VALUE_SEPARATOR: char = ' ';

pub struct PersistedKeyValueStore {
    base: InMemoryKeyValueStore,
    file: File,
}

impl PersistedKeyValueStore {
    pub fn new(filename: String) -> Self {
        let file = OpenOptions::new().read(true).write(true).create(true).open(filename).unwrap();

        let mut file_data: HashMap<String, String> = HashMap::new();
        {
            let buffered_reader = BufReader::new(&file);
            for line in buffered_reader.lines() {
                let line = line.unwrap();
                match line.find(KEY_VALUE_SEPARATOR) {
                    Some(separator_index) => {
                        let key = line[0..separator_index].to_owned();
                        let value = line[separator_index + 1..].to_owned();
                        file_data.insert(key, value);
                    }
                    None => println!("Ignoring line because of invalid format: '{}'", line),
                }
            }
        }

        PersistedKeyValueStore {
            base: InMemoryKeyValueStore::with_initial_data(file_data),
            file: file,
        }
    }
}

impl KeyValueStore for PersistedKeyValueStore {
    fn insert(&mut self, value: String) -> String {
        let key = self.base.insert(value.clone());
        self.file
            .write_all(format!("{}{}{}\n", &key, KEY_VALUE_SEPARATOR, &value).as_bytes())
            .unwrap();
        key
    }

    fn get(&self, key: &str) -> Option<String> {
        self.base.get(key)
    }
}
