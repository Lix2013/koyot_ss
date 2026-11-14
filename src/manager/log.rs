use colored::*;


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LogStates {
    Info,
    Error,
    Warn,
    Sucres
}

impl LogStates {
    pub fn as_str(&self) -> &str {
        match self {
            LogStates::Info => "[INFO]",
            LogStates::Error => "[ERROR]",
            LogStates::Sucres => "[SUCRES]",
            LogStates::Warn => "[WARN]"
        }
    }
}

pub fn log(log_state: LogStates, msg: &str) {
    match log_state {
        LogStates::Info => println!("[SYS_LOG] - {} : {}", log_state.as_str().blue().bold().underline(), msg.bold()),
        LogStates::Error => println!("[SYS_LOG] - {} : {}", log_state.as_str().red().bold().underline(), msg.bold()),
        LogStates::Sucres => println!("[SYS_LOG] - {} : {}", log_state.as_str().green().bold().underline(), msg.bold()),
        LogStates::Warn => println!("[SYS_LOG] - {} : {}", log_state.as_str().yellow().bold().underline(), msg.bold())
    }
}