//! # logs system
//! this is a mini logs system
//! ---
//! ## how to read this logs ?
//! if the log starts with [INFO]
//! He has a opretion or a mini information
//! or the log starts with [ERROR]
//! He has a error
//! and if the log starts with [WARN]
//! He have a warning
//! or the log starts with [SUCCESS]
//! He has a operation finished and don't has an error

use colored::*;

#[allow(dead_code)]
#[allow(unused)]

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LogStates {
    Info,
    Error,
    Warn,
    Success
}

impl LogStates {
    pub fn as_str(&self) -> &str {
        match self {
            LogStates::Info => "[INFO]",
            LogStates::Error => "[ERROR]",
            LogStates::Success => "[SUCCESS]",
            LogStates::Warn => "[WARN]"
        }
    }
}

pub fn log(log_state: LogStates, msg: &str) {
    let colored_log_type = match log_state {
        LogStates::Info => log_state.as_str().blue().bold().underline(),
        LogStates::Error => log_state.as_str().red().bold().underline(),
        LogStates::Success => log_state.as_str().green().bold().underline(),
        LogStates::Warn =>  log_state.as_str().yellow().bold().underline()
    };

    println!("[Logs System] - {} : {}", colored_log_type, msg.bold());
}