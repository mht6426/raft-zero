use crate::kv::command::Command;
use crate::kv::state::KvState;
use crate::raft::log::LogEntry;

#[derive(Debug, Clone)]
pub struct RaftState {
    /// 当前任期
    pub current_term: u64,
    /// 当前任期投给了谁（None 表示还没投）
    pub voted_for: Option<u64>,
    /// Raft 日志
    pub log: Vec<LogEntry>,
    /// 已提交日志的最大 index
    pub commit_index: u64,
    /// 已应用到状态机的最大 index
    pub last_applied: u64,
}

impl RaftState {
    pub fn new() -> Self {
        RaftState {
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
        }
    }
}

impl RaftState {
    pub fn apply_committed(&mut self, kv: &mut KvState) {
        while self.last_applied < self.commit_index {
            let net_index = self.last_applied + 1;

            let entry = &self.log[(net_index - 1) as usize];
            kv.apply(entry.command.clone());
            self.last_applied = net_index;
        }
    }
    pub fn append_command(&mut self, command: Command) {
        let index = self.log.len() as u64 + 1;
        let entry = LogEntry {
            term: self.current_term,
            index,
            command,
        };
        self.log.push(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_committed_entries() {
        let mut raft = RaftState {
            current_term: 1,
            voted_for: None,
            log: vec![
                LogEntry {
                    term: 1,
                    index: 1,
                    command: Command::Put {
                        key: "a".into(),
                        value: "1".into(),
                    },
                },
                LogEntry {
                    term: 1,
                    index: 2,
                    command: Command::Put {
                        key: "b".into(),
                        value: "2".into(),
                    },
                },
                LogEntry {
                    term: 1,
                    index: 3,
                    command: Command::Put {
                        key: "c".into(),
                        value: "3".into(),
                    },
                },
            ],
            commit_index: 2,
            last_applied: 0,
        };

        let mut kv = KvState::new();
        raft.apply_committed(&mut kv);

        assert_eq!(kv.get("a").map(|s| s.as_str()), Some("1"));
        assert_eq!(kv.get("b").map(|s| s.as_str()), Some("2"));
        assert_eq!(kv.get("c"), None);
        assert_eq!(raft.last_applied, 2);
    }

    #[test]
    fn test_append_command() {
        let mut raft = RaftState::new();
        raft.current_term = 1;

        raft.append_command(Command::Put {
            key: "a".into(),
            value: "1".into(),
        });

        raft.append_command(Command::Put {
            key: "b".into(),
            value: "2".into(),
        });

        assert_eq!(raft.log.len(), 2);
        assert_eq!(raft.log[0].index, 1);
        assert_eq!(raft.log[1].index, 2);
    }
}

