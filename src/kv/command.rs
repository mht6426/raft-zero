
#[derive(Debug, Clone)]
pub enum Command {
    Put {name: String, money: String},
    Get {name: String},
    Delete {name: String},
}
