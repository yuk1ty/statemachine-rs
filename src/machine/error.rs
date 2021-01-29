use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum StateMachineError {
    FailedToBuild(String),
}

impl Display for StateMachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateMachineError::FailedToBuild(field_name) => f.write_str(&format!(
                "Failed to build the builder because {} field is uninitialized.",
                field_name
            )),
        }
    }
}

impl Error for StateMachineError {}
