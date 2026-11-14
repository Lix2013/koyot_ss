use colored::*;


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
            LogStates::Success => "[SUCRES]",
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