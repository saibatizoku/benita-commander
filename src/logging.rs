//! Basic logging utilities.
use chrono;
use fern;
use log;
use std;

// Errors and their descriptions
mod errors {
    error_chain! {
        errors {
            LogFileNotFound(path: String) {
                description("could not find the log file")
                display("could not find the log file: {}", path)
            }
            LoggerFailed {
                description("logger could not be started")
            }
        }
    }
}

pub use self::errors::*;

/// Basic logger for shell terminals
pub fn base_term_logger() -> Result<fern::Dispatch> {
    let base_config = fern::Dispatch::new().format(|out, message, record| {
        out.finish(format_args!(
            "[{}][{}] {}",
            record.target(),
            record.level(),
            message
        ))
    });
    Ok(base_config)
}

/// Basic logger for file storage.
pub fn base_file_logger(path: &str) -> Result<fern::Dispatch> {
    let base_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(path)
            .chain_err(|| ErrorKind::LogFileNotFound(path.to_string()))?);
    Ok(base_config)
}

/// Configure and start logging.
pub fn start_log() -> Result<()> {
    let base_config = fern::Dispatch::new();

    let err_logger_file = base_file_logger("benita-commander.err")?.filter(|metadata| {
        // Reject messages with the `Info` log level, accept others.
        //
        // This could be useful for sending Error messages to stderr
        // and avoiding duplicate messages in stdout.
        metadata.level() != log::LogLevelFilter::Info
    });

    let err_logger = base_term_logger()?
        .level(log::LogLevelFilter::Trace)
        .filter(|metadata| {
            // Reject messages with the `Info` log level, accept others.
            //
            // This could be useful for sending Error messages to stderr
            // and avoiding duplicate messages in stdout.
            metadata.level() != log::LogLevelFilter::Info
        })
        .chain(std::io::stderr());

    let info_logger_file = base_file_logger("benita-commander.log")?.filter(|metadata| {
        // Reject messages with the `Info` log level, accept others.
        //
        // This could be useful for sending Error messages to stderr
        // and avoiding duplicate messages in stdout.
        metadata.level() == log::LogLevelFilter::Info
    });

    let term_logger = base_term_logger()?
        .level(log::LogLevelFilter::Info)
        .filter(|metadata| {
            // Reject messages with the `Info` log level, accept others.
            //
            // This could be useful for sending Error messages to stderr
            // and avoiding duplicate messages in stdout.
            metadata.level() == log::LogLevelFilter::Info
        })
        .chain(std::io::stdout());

    let _start = base_config
        .chain(err_logger_file)
        .chain(err_logger)
        .chain(info_logger_file)
        .chain(term_logger)
        .apply()
        .chain_err(|| ErrorKind::LoggerFailed)?;
    Ok(())
}
