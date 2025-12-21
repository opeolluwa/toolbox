use rusqlite::Connection;

use crate::database::repository::KvRepository;
use crate::database::repository::KvRepositoryTrait;
use crate::helpers::console::LogMessage;

pub struct StoreConfig {
    kv_store_repository: KvRepository,
}

impl StoreConfig {
    pub fn new(conn: Connection) -> Self {
        Self {
            kv_store_repository: KvRepository::new(conn),
        }
    }

    pub fn save(&self, key: &str, value: &str, sensitive: bool) {
        let _ = self.kv_store_repository.create_new(key, value, sensitive);
    }
    pub fn list(&self) {
        let dataset = self.kv_store_repository.retrieve_all();
        if dataset.is_empty() {
            LogMessage::neutral("Store is empty");
            return;
        }
        for entry in dataset {
            println!("{} -> {}", entry.key, entry.value)
        }
    }
    pub fn find(&self) {}
    pub fn remove(&self) {}
    pub fn export(&self) {}
    pub fn back_up(&self) {}
}
