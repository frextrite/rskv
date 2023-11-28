use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct KeyValue {
    store: HashMap<String, String>,
}

impl KeyValue {
    pub fn new() -> KeyValue {
        KeyValue { store: HashMap::new() }
    }

    pub fn set(&mut self, key: String, val: String) -> Option<String> {
        self.store.insert(key, val)
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.store.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_kv() {
        let kv = KeyValue::new();
        assert_eq!(kv.store.len(), 0);
    }

    #[test]
    fn basics() {
        let mut kv = KeyValue::new();

        assert_eq!(kv.get("key".to_string()), None);

        assert_eq!(kv.set("key".to_string(), "val".to_string()), None);
        assert_eq!(kv.get("key".to_string()), Some("val".to_string()).as_ref());

        assert_eq!(kv.set("key".to_string(), "val2".to_string()), Some("val".to_string()));
        assert_eq!(kv.get("key".to_string()), Some("val2".to_string()).as_ref());
    }
}
