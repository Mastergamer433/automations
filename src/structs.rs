use serde_json::{Value};
use serde::{Serialize, Deserialize};

// Define a struct to represent the command structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub command: String,
    pub arguments: Value,
}
