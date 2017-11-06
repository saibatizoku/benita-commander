//! Control interface for `benita` services
#![recursion_limit = "1024"]

extern crate benita;
extern crate chrono;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate fern;

/// Errors and their descriptions
pub mod errors {
    use benita;
    error_chain! {
        links {
            Benita(benita::errors::Error, benita::errors::ErrorKind);
        }
    }
}

use errors::*;

/// Configure and start logger.
fn start_logger() -> Result<()> {
    let base_config = fern::Dispatch::new();

    let err_logger_file = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .filter(|metadata| {
                   // Reject messages with the `Info` log level, accept others.
                   //
                   // This could be useful for sending Error messages to stderr
                   // and avoiding duplicate messages in stdout.
                   metadata.level() != log::LogLevelFilter::Info
        })
        .chain(fern::log_file("benita-commander.err")
               .chain_err(|| "could not setup error log file")?);

    let err_logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LogLevelFilter::Trace)
        .filter(|metadata| {
                   // Reject messages with the `Info` log level, accept others.
                   //
                   // This could be useful for sending Error messages to stderr
                   // and avoiding duplicate messages in stdout.
                   metadata.level() != log::LogLevelFilter::Info
        })
        .chain(std::io::stderr());

    let info_logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LogLevelFilter::Error)
        .chain(std::io::stdout())
        .chain(fern::log_file("benita-commander.log")
            .chain_err(|| "failed to open log file")?);

    let _start = base_config
        .chain(err_logger_file)
        .chain(err_logger)
        .chain(info_logger)
        .apply()
        .chain_err(|| "Could not setup error logging")?;
    Ok(())
}

/// Main program. Starts logger, then evaluates args from stdin.
fn run_code() -> Result<()> {
    // Initialize logging.
    let _log = start_logger()?;
    info!("Starting calibrated-service");
    println!("Hello, world!");
    Ok(())
}

// fn main() wrapped to handle error chains
quick_main!(run_code);
