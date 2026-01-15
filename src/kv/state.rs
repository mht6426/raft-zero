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
    pub fn get(&self, name: &str) -> Option<&String> {
        self.data.get(name)
    }
}

impl KvState {
    pub fn apply(&mut self, cmd: Command) {
        match cmd {
            Command::Put { name, money } => {
                self.data.insert(name, money);
            }
            Command::Get { name: _ } => {
                // Get command does not modify state
            }
            Command::Delete { name } => {
                self.data.remove(&name);
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
            name: "Anna".to_string(),
            money: "one dollar".to_string(),
        });

        // get
        let v = state.get("Anna");
        assert_eq!(v.map(|s| s.as_str()), Some("one dollar"));

        // delete
        state.apply(Command::Delete {
            name: "Anna".to_string(),
        });

        // get again
        let v = state.get("Anna");
        assert_eq!(v, None);
    }
}
