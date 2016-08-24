pub mod in_memory;
pub mod persisted;

pub trait KeyValueStore: 'static + Send + Sync + Sized {
    fn insert(&mut self, value: String) -> String;
    fn get(&self, key: &str) -> Option<String>;
}
