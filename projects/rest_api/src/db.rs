use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{CreateItem, Item, UpdateItem};

#[derive(Clone)]
pub struct Db {
    items: Arc<RwLock<HashMap<String, Item>>>,
}

impl Db {
    pub fn baru() -> Self {
        Db {
            items: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn list(&self) -> Vec<Item> {
        let items = self.items.read().await;
        let mut result: Vec<Item> = items.values().cloned().collect();
        result.sort_by(|a, b| a.nama.cmp(&b.nama));
        result
    }

    pub async fn get(&self, id: &str) -> Option<Item> {
        let items = self.items.read().await;
        items.get(id).cloned()
    }

    pub async fn create(&self, data: CreateItem) -> Item {
        let item = Item::baru(data.nama, data.deskripsi);
        let mut items = self.items.write().await;
        items.insert(item.id.clone(), item.clone());
        item
    }

    pub async fn update(&self, id: &str, data: UpdateItem) -> Option<Item> {
        let mut items = self.items.write().await;
        if let Some(item) = items.get_mut(id) {
            item.update(data);
            Some(item.clone())
        } else {
            None
        }
    }

    pub async fn delete(&self, id: &str) -> bool {
        let mut items = self.items.write().await;
        items.remove(id).is_some()
    }
}
