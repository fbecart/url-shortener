use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

use super::KeyValueStore;
use super::in_memory::InMemoryKeyValueStore;

const KEY_VALUE_SEPARATOR: char = ' ';

pub struct PersistedKeyValueStore {
    base: InMemoryKeyValueStore,
    file: File,
}

impl PersistedKeyValueStore {
    pub fn new(filename: String) -> Self {
        let file = OpenOptions::new().read(true).write(true).create(true).open(filename).unwrap();
        let file_data = PersistedKeyValueStore::extract_data(&file);

        PersistedKeyValueStore {
            base: InMemoryKeyValueStore::with_initial_data(file_data),
            file: file,
        }
    }

    fn extract_data(file: &File) -> HashMap<String, String> {
        BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .flat_map(|line| match line.find(KEY_VALUE_SEPARATOR) {
                Some(separator_index) => {
                    let key = line[0..separator_index].to_owned();
                    let value = line[separator_index + 1..].to_owned();
                    Some((key, value))
                }
                None => {
                    println!("Ignoring line because of invalid format: '{}'", line);
                    None
                }
            })
            .fold(HashMap::new(), |mut acc, (key, value)| {
                acc.insert(key, value);
                acc
            })
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
