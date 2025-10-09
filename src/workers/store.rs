use rusqlite::Connection;

use crate::database::repository::KvRepository;
use crate::database::repository::KvRepositoryTrait;

pub struct StoreConfig {
    kv_store_repository: KvRepository,
}

impl StoreConfig {
    pub fn new(conn: Connection) -> Self {
        Self {
            kv_store_repository: KvRepository::new(conn),
        }
    }

    pub fn save(&self, key: &String, value: &String, sensitive: bool) {
        let _ = self.kv_store_repository.create_new(&key, &value, sensitive);
    }
    pub fn list(&self) {
        let data = self.kv_store_repository.retrieve_all();
        println!("{:#?}", data);
    }
    pub fn find(&self) {}
    pub fn remove(&self) {}
    pub fn export(&self) {}
    pub fn back_up(&self) {}
}
