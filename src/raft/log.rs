use crate::kv::command::Command;

#[derive(Clone, Debug)]
pub struct LogEntry {
    // 任期
    pub term: u64,
    // 日志索引
    pub index: u64,
    // 命令
    pub command: Command,
}