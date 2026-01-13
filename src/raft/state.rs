use crate::raft::log::LogEntry;
use crate::kv::state::KvState;

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
    pub fn apply_committed(&mut self,kv: &mut KvState){
        while self.last_applied < self.commit_index {
            let net_index = self.last_applied + 1;

            let entry = &self.log[(net_index - 1) as usize];
            kv.apply(entry.command.clone());
            self.last_applied = net_index;
        }
    }
}
