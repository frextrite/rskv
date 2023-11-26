use std::collections::HashMap;

struct KeyValue {
    store: HashMap<String, String>,
}

impl KeyValue {
    fn new() -> KeyValue {
        KeyValue { store: HashMap::new() }
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
}
