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
    /// 是否是领导者(单节点，仅供学习过程中使用)
    pub is_leader: bool,
}

impl RaftState {
    pub fn new() -> Self {
        RaftState {
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
            is_leader: false,
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
    pub fn commit_to(&mut self, index: u64) {
        let max_index = self.log.len() as u64;
        let new_commit = index.min(max_index);
        if new_commit > self.commit_index {
            self.commit_index = new_commit;
        }
    }
}

impl RaftState {
    pub fn handle_command_as_leader(
        &mut self,
        command: Command,
        kv: &mut KvState,
    ) -> Result<(), &'static str> {
        if !self.is_leader {
            return Err("Not the leader".into());
        }

        self.append_command(command);
        let last_index = self.log.len() as u64;
        self.commit_to(last_index);
        self.apply_committed(kv);

        Ok(())
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
                        name: "Anna".into(),
                        money: "one dollar".into(),
                    },
                },
                LogEntry {
                    term: 1,
                    index: 2,
                    command: Command::Put {
                        name: "Bob".into(),
                        money: "two dollar".into(),
                    },
                },
                LogEntry {
                    term: 1,
                    index: 3,
                    command: Command::Put {
                        name: "Carl".into(),
                        money: "three dollar".into(),
                    },
                },
            ],
            commit_index: 2,
            last_applied: 0,
            is_leader: false,
        };

        let mut kv = KvState::new();
        raft.apply_committed(&mut kv);

        assert_eq!(kv.get("Anna").map(|s| s.as_str()), Some("one dollar"));
        assert_eq!(kv.get("Bob").map(|s| s.as_str()), Some("two dollar"));
        assert_eq!(kv.get("Carl"), None);
        assert_eq!(raft.last_applied, 2);
    }

    #[test]
    fn test_append_command() {
        let mut raft = RaftState::new();
        raft.current_term = 1;

        raft.append_command(Command::Put {
            name: "Anna".into(),
            money: "one dollar".into(),
        });

        raft.append_command(Command::Put {
            name: "Bob".into(),
            money: "two dollar".into(),
        });

        assert_eq!(raft.log.len(), 2);
        assert_eq!(raft.log[0].index, 1);
        assert_eq!(raft.log[1].index, 2);
    }

    #[test]
    fn test_append_commit_apply_flow() {
        let mut raft = RaftState::new();
        raft.current_term = 1;

        let mut kv = KvState::new();

        raft.append_command(Command::Put {
            name: "Anna".into(),
            money: "one dollar".into(),
        });
        raft.append_command(Command::Put {
            name: "Bob".into(),
            money: "two dollar".into(),
        });

        raft.apply_committed(&mut kv);
        assert_eq!(kv.get("Anna"), None);
        assert_eq!(kv.get("Bob"), None);

        raft.commit_to(2);
        raft.apply_committed(&mut kv);
        assert_eq!(kv.get("Anna").map(|s| s.as_str()), Some("one dollar"));
        assert_eq!(kv.get("Bob").map(|s| s.as_str()), Some("two dollar"));
    }

    #[test]
    fn test_leader_handle_command() {
        let mut raft = RaftState::new();
        raft.current_term = 1;
        raft.is_leader = true;

        let mut kv = KvState::new();

        raft.handle_command_as_leader(
            Command::Put {
                name: "Anna".into(),
                money: "one dollar".into(),
            },
            &mut kv,
        )
        .unwrap();

        assert_eq!(kv.get("Anna").map(|s| s.as_str()), Some("one dollar"));
        assert_eq!(raft.log.len(), 1);
        assert_eq!(raft.commit_index, 1);
        assert_eq!(raft.last_applied, 1);
    }
}
