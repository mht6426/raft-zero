use crate::raft::log::LogEntry; 

#[derive(Debug, Clone)]
pub struct AppendEntries {
    pub entries: Vec<LogEntry>,
    pub leader_commit: u64,
}