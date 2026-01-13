use crate::kv::command::Command;
use std::collections::HashMap;

#[derive(Debug)]
pub struct KvState {
    data: HashMap<String, String>,
}

impl KvState {
    pub fn new() -> Self {
        KvState {
            data: HashMap::new(),
        }
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

impl KvState {
    pub fn apply(&mut self, cmd: Command) {
        match cmd {
            Command::Put { key, value } => {
                self.data.insert(key, value);
            }
            Command::Get { key: _ } => {
                // Get command does not modify state
            }
            Command::Delete { key } => {
                self.data.remove(&key);
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::kv::command::Command;

    #[test]
    fn test_kv_put_get_delete() {
        let mut state = KvState::new();

        // put
        state.apply(Command::Put {
            key: "a".to_string(),
            value: "1".to_string(),
        });

        // get
        let v = state.get("a");
        assert_eq!(v, Some(&"1".to_string()));

        // delete
        state.apply(Command::Delete {
            key: "a".to_string(),
        });

        // get again
        let v = state.get("a");
        assert_eq!(v, None);
    }
}
