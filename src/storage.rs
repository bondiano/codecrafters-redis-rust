use std::time::SystemTime;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct Data {
    value: String,
    expire_at: Option<u128>,
}

#[derive(Debug, Clone)]
pub struct Storage {
    data: Arc<Mutex<HashMap<String, Data>>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        if let Some(data) = self.data.lock().unwrap().get(key) {
            if let Some(expire_at) = data.expire_at {
                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();

                if expire_at <= now {
                    return None;
                }
            }
            return Some(data.value.clone());
        }

        None
    }

    pub fn set(&mut self, key: &str, value: &str, ttl: Option<u128>) {
        let expire_at = if let Some(ttl) = ttl {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();

            Some(now + ttl)
        } else {
            None
        };

        self.data.lock().unwrap().insert(
            key.to_string(),
            Data {
                value: value.to_string(),
                expire_at,
            },
        );
    }
}
