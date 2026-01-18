use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
static STORE: OnceLock<Store> = OnceLock::new();

pub struct Store {
  db: Arc<RwLock<HashMap<String, String>>>
}

impl Store {
   pub fn New() -> Self {
     Store {
        db: Arc:: new(RwLock::new(HashMap::new()))
     }
   }

    pub fn getStore() -> &'static Store {
        STORE.get_or_init(|| Store {
            db: Arc::new(RwLock::new(HashMap::new())),
        })
    }

   pub async fn set(&self, key: String, value: String) {
     let mut db = self.db.write().await;
     db.insert(key, value);
   } 

   pub async fn get(&self, key: String) -> Option<String> {
     let db = self.db.write().await;
     if !db.contains_key(&key) {
        return None;
     } 
       let val = db.get(&key).unwrap();
       return Some(val.to_owned());
   }

   pub async fn delete(&self, key: String) -> bool {
    let mut db = self.db.write().await;
    if !db.contains_key(&key) {
      return false
    }
      db.remove(&key);
      return true;
   } 
}