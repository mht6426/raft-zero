
#[derive(Debug, Clone)]
pub enum Command {
    Put {key: String, value: String},
    Get {key: String},
    Delete {key: String},
}
